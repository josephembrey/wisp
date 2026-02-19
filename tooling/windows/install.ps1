#Requires -Version 5.1
<#
.SYNOPSIS
    Installs development dependencies for Wisp on Windows via winget.
.DESCRIPTION
    .\win.ps1 install       - core build deps (Rust, VS Build Tools, LLVM, Vulkan, Bun)
    .\win.ps1 install sign  - also installs signing tools (Windows SDK, Azure CLI,
                              Azure Trusted Signing Client Tools)
    Run from an elevated (Administrator) PowerShell for the VS Build Tools install.
#>

param(
    [Parameter(Position = 0)]
    [ValidateSet('', 'sign')]
    [string]$Scope
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Test-Command($Name) {
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

if (-not (Test-Command 'winget')) {
    Write-Error "winget is not available. Install App Installer from the Microsoft Store."
    exit 1
}

Write-Host "`n=== Wisp Windows dependency installer ===" -ForegroundColor Cyan

# --- Enable long paths (whisper.cpp Vulkan build exceeds MAX_PATH) ---
$longPathKey = 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem'
$longPathEnabled = (Get-ItemProperty -Path $longPathKey -Name 'LongPathsEnabled' -ErrorAction SilentlyContinue).LongPathsEnabled
if ($longPathEnabled -eq 1) {
    Write-Host "`n[Long paths] Already enabled - skipping." -ForegroundColor Green
} else {
    Write-Host "`n[Long paths] Enabling Win32 long path support (required for whisper.cpp build)..." -ForegroundColor Yellow
    try {
        Set-ItemProperty -Path $longPathKey -Name 'LongPathsEnabled' -Value 1 -Type DWord
        Write-Host "[Long paths] Enabled. A reboot may be needed for full effect." -ForegroundColor Yellow
    } catch {
        Write-Host "[Long paths] Failed - run this script as Administrator to enable." -ForegroundColor Red
        Write-Host "  Without long paths, the whisper.cpp Vulkan build will fail." -ForegroundColor Red
    }
}

# --- Rust via rustup ---
if (Test-Command 'rustup') {
    Write-Host "`n[rustup] Already installed - updating..." -ForegroundColor Green
    rustup update
} else {
    Write-Host "`n[rustup] Installing Rust toolchain..." -ForegroundColor Yellow
    winget install --id Rustlang.Rustup -e --accept-source-agreements --accept-package-agreements
}

# --- Visual Studio Build Tools (C++ workload) ---
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$hasVCTools = $false
if (Test-Path $vsWhere) {
    $instances = & $vsWhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($instances) { $hasVCTools = $true }
}

if ($hasVCTools) {
    Write-Host "`n[VS Build Tools] C++ workload already installed - skipping." -ForegroundColor Green
} else {
    Write-Host "`n[VS Build Tools] Installing with C++ workload (may take several minutes)..." -ForegroundColor Yellow
    winget install --id Microsoft.VisualStudio.2022.BuildTools -e `
        --accept-source-agreements --accept-package-agreements `
        --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
}

# --- LLVM / Clang (libclang needed by bindgen for whisper-rs FFI bindings) ---
$llvmDir = "${env:ProgramFiles}\LLVM"
if (Test-Path "$llvmDir\bin\libclang.dll") {
    Write-Host "`n[LLVM] Already installed - skipping." -ForegroundColor Green
} else {
    Write-Host "`n[LLVM] Installing LLVM (provides libclang for bindgen)..." -ForegroundColor Yellow
    winget install --id LLVM.LLVM -e --accept-source-agreements --accept-package-agreements
}

# Set LIBCLANG_PATH if not already set
$llvmBin = "${env:ProgramFiles}\LLVM\bin"
if (-not $env:LIBCLANG_PATH -and (Test-Path "$llvmBin\libclang.dll")) {
    Write-Host "[LLVM] Setting LIBCLANG_PATH environment variable..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable('LIBCLANG_PATH', $llvmBin, 'User')
    $env:LIBCLANG_PATH = $llvmBin
}

# --- Vulkan SDK (required by whisper-rs vulkan feature) ---
$vulkanDir = if ($env:VULKAN_SDK) { $env:VULKAN_SDK } else {
    Get-ChildItem 'C:\VulkanSDK' -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending | Select-Object -First 1 -ExpandProperty FullName
}
if ($vulkanDir -and (Test-Path $vulkanDir)) {
    Write-Host "`n[Vulkan SDK] Already installed at $vulkanDir - skipping." -ForegroundColor Green
} else {
    Write-Host "`n[Vulkan SDK] Installing Vulkan SDK..." -ForegroundColor Yellow
    winget install --id KhronosGroup.VulkanSDK -e --accept-source-agreements --accept-package-agreements
    $vulkanDir = Get-ChildItem 'C:\VulkanSDK' -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending | Select-Object -First 1 -ExpandProperty FullName
}

if ($vulkanDir -and -not $env:VULKAN_SDK) {
    Write-Host "[Vulkan SDK] Setting VULKAN_SDK environment variable..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable('VULKAN_SDK', $vulkanDir, 'User')
    $env:VULKAN_SDK = $vulkanDir
}

# --- Bun (in case the user doesn't have it yet) ---
if (Test-Command 'bun') {
    Write-Host "`n[bun] Already installed - skipping." -ForegroundColor Green
} else {
    Write-Host "`n[bun] Installing bun..." -ForegroundColor Yellow
    winget install --id Oven-sh.Bun -e --accept-source-agreements --accept-package-agreements
}

# --- Signing tools (optional) ---
if ($Scope -eq 'sign') {
    # Windows SDK (provides signtool.exe)
    $hasSignTool = (Get-Command signtool.exe -ErrorAction SilentlyContinue) -or (Test-Path "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\signtool.exe")
    if ($hasSignTool) {
        Write-Host "`n[Windows SDK] signtool.exe found - skipping." -ForegroundColor Green
    } else {
        Write-Host "`n[Windows SDK] Installing (provides signtool.exe)..." -ForegroundColor Yellow
        winget install --id Microsoft.WindowsSDK.10.0.26100 -e --accept-source-agreements --accept-package-agreements
    }

    # Azure CLI
    if (Test-Command 'az') {
        Write-Host "`n[Azure CLI] Already installed - skipping." -ForegroundColor Green
    } else {
        Write-Host "`n[Azure CLI] Installing..." -ForegroundColor Yellow
        winget install --id Microsoft.AzureCLI -e --accept-source-agreements --accept-package-agreements
    }

    # Azure Trusted Signing Client Tools
    $dlibPath = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'Microsoft\MicrosoftTrustedSigningClientTools\Azure.CodeSigning.Dlib.dll'
    if (Test-Path $dlibPath) {
        Write-Host "`n[Trusted Signing] Already installed - skipping." -ForegroundColor Green
    } else {
        Write-Host "`n[Trusted Signing] Installing Azure Trusted Signing Client Tools..." -ForegroundColor Yellow
        winget install --id Microsoft.Azure.TrustedSigningClientTools -e --accept-source-agreements --accept-package-agreements
    }
}

# --- Summary ---
Write-Host "`n=== Done ===" -ForegroundColor Cyan
Write-Host @"

Restart your terminal, then verify:
  rustup --version
  cargo --version
  cmake --version
  clang --version
  bun --version

Then from the repo root:
  bun install
  bun tauri dev
"@
