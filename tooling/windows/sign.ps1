#Requires -Version 5.1
<#
.SYNOPSIS
    Signs the built Wisp executable using Azure Trusted Signing.
.DESCRIPTION
    Requires: Windows SDK (signtool), Azure Trusted Signing Client Tools, Azure CLI.
    Run .\win.ps1 install sign to install these.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

$candidates = @(
    Join-Path $env:CARGO_TARGET_DIR 'release\wisp.exe'
    Join-Path $env:CARGO_TARGET_DIR 'x86_64-pc-windows-msvc\release\wisp.exe'
)

$exe = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $exe) {
    Write-Error "wisp.exe not found. Run .\win.ps1 build first."
    exit 1
}

# TODO: signing/sign.ps1 hardcodes the cross-compiled exe path
# (target\x86_64-pc-windows-msvc\release). Consider updating it to accept an exe
# path parameter, then have both this script and devenv's sign command pass it in.
& "$root\signing\sign.ps1"
