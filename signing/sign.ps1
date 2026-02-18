$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not $RepoRoot) { $RepoRoot = Split-Path -Parent $PSScriptRoot }

$Exe = Get-ChildItem "$RepoRoot\target\x86_64-pc-windows-msvc\release\wisp.exe" -ErrorAction SilentlyContinue
if (-not $Exe) {
    Write-Error "wisp.exe not found — run build-windows first"
    exit 1
}

$DlibPath = "$env:LOCALAPPDATA\Microsoft\MicrosoftTrustedSigningClientTools\Azure.CodeSigning.Dlib.dll"
if (-not (Test-Path $DlibPath)) {
    Write-Error "Azure Trusted Signing client tools not found at $DlibPath`nInstall with: winget install -e --id Microsoft.Azure.TrustedSigningClientTools"
    exit 1
}

$Metadata = "$PSScriptRoot\metadata.json"
if (-not (Test-Path $Metadata)) {
    Write-Error "metadata.json not found at $Metadata"
    exit 1
}

$SignTool = Get-Command signtool.exe -ErrorAction SilentlyContinue
if (-not $SignTool) {
    Write-Error "signtool.exe not found — install Windows SDK"
    exit 1
}

Write-Host "Signing $($Exe.FullName) ..." -ForegroundColor Cyan

& signtool.exe sign /v /debug `
    /fd SHA256 `
    /tr "http://timestamp.acs.microsoft.com" `
    /td SHA256 `
    /dlib $DlibPath `
    /dmdf $Metadata `
    $Exe.FullName

if ($LASTEXITCODE -ne 0) {
    Write-Error "Signing failed (exit code $LASTEXITCODE)"
    exit $LASTEXITCODE
}

Write-Host "Signed successfully." -ForegroundColor Green

& signtool.exe verify /pa /v $Exe.FullName
