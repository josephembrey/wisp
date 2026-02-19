#Requires -Version 5.1
<#
.SYNOPSIS
    Runs Wisp in development mode.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

Push-Location $root
try {
    if (-not (Test-Path (Join-Path $root 'node_modules'))) {
        Write-Host "Installing dependencies..." -ForegroundColor Cyan
        bun install
    }

    bun tauri dev
} finally { Pop-Location }
