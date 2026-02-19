$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot

$TargetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $RepoRoot 'target' }
$candidates = @(
    Join-Path $TargetDir 'release\wisp.exe'
    Join-Path $TargetDir 'x86_64-pc-windows-msvc\release\wisp.exe'
)
$Exe = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1 | Get-Item
if (-not $Exe) {
    Write-Host "ERROR: wisp.exe not found. Run .\win.ps1 build first." -ForegroundColor Red
    exit 1
}

$LocalAppData = [Environment]::GetFolderPath('LocalApplicationData')
$DlibPath = Join-Path $LocalAppData 'Microsoft\MicrosoftTrustedSigningClientTools\Azure.CodeSigning.Dlib.dll'
if (-not (Test-Path $DlibPath)) {
    Write-Host ("ERROR: Azure Trusted Signing client tools not found at " + $DlibPath) -ForegroundColor Red
    Write-Host "Install with: winget install --exact --id Microsoft.Azure.TrustedSigningClientTools" -ForegroundColor Yellow
    exit 1
}

$Metadata = Join-Path $PSScriptRoot 'metadata.json'
if (-not (Test-Path $Metadata)) {
    Write-Host ("ERROR: metadata.json not found at " + $Metadata) -ForegroundColor Red
    exit 1
}

$cmd = Get-Command signtool.exe -ErrorAction SilentlyContinue
$SignTool = if ($cmd) { $cmd.Source } else { $null }
if (-not $SignTool) {
    $SignTool = Get-ChildItem 'C:\Program Files (x86)\Windows Kits\10\bin\*\x64\signtool.exe' -ErrorAction SilentlyContinue |
        Sort-Object { $_.Directory.Name } -Descending |
        Select-Object -First 1 -ExpandProperty FullName
}
if (-not $SignTool) {
    Write-Host "ERROR: signtool.exe not found. Install Windows SDK." -ForegroundColor Red
    exit 1
}
# Ensure Azure CLI is on PATH for the signing dlib (needed when invoked from WSL)
$AzCliPaths = @(
    'C:\Program Files\Microsoft SDKs\Azure\CLI2\wbin',
    'C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin'
)
foreach ($p in $AzCliPaths) {
    if ((Test-Path $p) -and ($env:PATH -notlike "*$p*")) {
        $env:PATH = "$p;$env:PATH"
        break
    }
}

Write-Host ("Using " + $SignTool) -ForegroundColor DarkGray

Write-Host ("Signing " + $Exe.FullName + " ...") -ForegroundColor Cyan

& $SignTool sign /v /debug `
    /fd SHA256 `
    /tr "http://timestamp.acs.microsoft.com" `
    /td SHA256 `
    /dlib $DlibPath `
    /dmdf $Metadata `
    $Exe.FullName

if ($LASTEXITCODE -ne 0) {
    Write-Host ("Signing failed with exit code " + $LASTEXITCODE) -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "Signed successfully." -ForegroundColor Green

& $SignTool verify /pa /v $Exe.FullName
