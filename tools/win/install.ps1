# Install dev dependencies for Wisp on Windows.
# Run as Administrator for VS Build Tools and long-path support.
# Usage: just install        (interactive)
#        just install ci      (non-interactive, skips optional prompts)

param([switch]$ci)

$ErrorActionPreference = 'Stop'

function wg($id) { winget install --id $id -e --accept-source-agreements --accept-package-agreements }

# Long paths (Vulkan shader builds exceed MAX_PATH)
try { Set-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem' -Name LongPathsEnabled -Value 1 -Type DWord } catch {}

# Rust (update if installed, otherwise install)
if (Get-Command rustup -EA SilentlyContinue) { rustup update } else { wg Rustlang.Rustup }

# VS Build Tools needs a special override for the C++ workload
$vsw = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not ((Test-Path $vsw) -and (& $vsw -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath))) {
    winget install --id Microsoft.VisualStudio.2022.BuildTools -e `
        --accept-source-agreements --accept-package-agreements `
        --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
}

wg LLVM.LLVM
wg KhronosGroup.VulkanSDK
wg Oven-sh.Bun
# Skip Just — it's the running process, so winget can't replace the locked exe.
# Upgrade manually: winget upgrade Casey.Just
if (-not (Get-Command just -EA SilentlyContinue)) { wg Casey.Just }

if (-not $ci) {
    wg direnv.direnv
    wg j178.Prek
}

# Refresh PATH so newly installed tools are available in this session
$env:Path = [Environment]::GetEnvironmentVariable('Path', 'Machine') + ';' + [Environment]::GetEnvironmentVariable('Path', 'User')
# Bun installs to ~/.bun/bin which may not be in the refreshed PATH yet
$bunDir = "$env:USERPROFILE\.bun\bin"
if ((Test-Path $bunDir) -and ($env:Path -notlike "*$bunDir*")) { $env:Path += ";$bunDir" }

# Persistent env vars (if not already set)
$llvm = "${env:ProgramFiles}\LLVM\bin"
if (-not $env:LIBCLANG_PATH -and (Test-Path "$llvm\libclang.dll")) {
    [Environment]::SetEnvironmentVariable('LIBCLANG_PATH', $llvm, 'User')
}
$vulkan = Get-ChildItem C:\VulkanSDK -Directory -EA SilentlyContinue | Sort-Object Name -Descending | Select-Object -First 1 -ExpandProperty FullName
if ($vulkan -and -not $env:VULKAN_SDK) {
    [Environment]::SetEnvironmentVariable('VULKAN_SDK', $vulkan, 'User')
}

# Signing tools (interactive only)
if (-not $ci) {
    $answer = Read-Host "`nInstall code signing tools? (y/N)"
    if ($answer -eq 'y') {
        wg Microsoft.WindowsSDK.10.0.26100
        wg Microsoft.AzureCLI
        wg Microsoft.Azure.TrustedSigningClientTools
    }

    # Git hooks
    prek install -q --config tools/prek.toml --hook-type pre-commit --hook-type commit-msg
}

Write-Host "`nDone." -ForegroundColor Cyan
