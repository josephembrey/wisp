#Requires -Version 5.1
<#
.SYNOPSIS
    Builds Wisp for production.
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

    Write-Host "Generating TypeScript bindings..." -ForegroundColor Cyan
    cargo run --manifest-path src-tauri/Cargo.toml --bin generate_bindings
    Write-Host "Bindings generated." -ForegroundColor Green

    bun tauri build
} finally { Pop-Location }
