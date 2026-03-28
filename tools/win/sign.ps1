# Sign the built Wisp executable with Azure Trusted Signing.
# Usage: just sign

$ErrorActionPreference = 'Stop'
function bail($msg) { Write-Error $msg; exit 1 }
function need($path, $msg) { if (-not (Test-Path $path)) { bail $msg } }

# Locate wisp.exe
$target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path (Resolve-Path "$PSScriptRoot\..\..") 'target' }
$exe = "$target\release\wisp.exe"
if (-not (Test-Path $exe)) { $exe = "$target\x86_64-pc-windows-msvc\release\wisp.exe" }
if (-not (Test-Path $exe)) { bail "wisp.exe not found. Run 'just build'." }

# Locate signing tools
$dlib = Join-Path $env:LOCALAPPDATA 'Microsoft\MicrosoftTrustedSigningClientTools\Azure.CodeSigning.Dlib.dll'
$meta = Join-Path $PSScriptRoot 'metadata.json'
need $dlib "Trusted Signing tools not found. Run 'just install sign'."
need $meta "metadata.json not found."

$signtool = (Get-Command signtool.exe -EA SilentlyContinue).Source
if (-not $signtool) {
    $signtool = (Get-ChildItem 'C:\Program Files (x86)\Windows Kits\10\bin\*\x64\signtool.exe' -EA SilentlyContinue |
        Sort-Object { $_.Directory.Name } -Descending | Select-Object -First 1).FullName
}
if (-not $signtool) { bail "signtool.exe not found. Run 'just install sign'." }

# Sign and verify
Write-Host "Signing $exe..." -ForegroundColor Cyan
& $signtool sign /v /debug /fd SHA256 /tr "http://timestamp.acs.microsoft.com" /td SHA256 /dlib $dlib /dmdf $meta $exe
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Signed." -ForegroundColor Green
& $signtool verify /pa /v $exe
