#Requires -Version 5.1
<#
.SYNOPSIS
    Builds Wisp for production with verbose logging enabled.
.DESCRIPTION
    Same as build, but enables the verbose-log feature for diagnostics.
    Logs will be written to the app's log directory.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

Push-Location $root
try {
    Write-Host "Generating TypeScript bindings..." -ForegroundColor Cyan
    cargo run --manifest-path src-tauri/Cargo.toml --bin generate-bindings
    Write-Host "Bindings generated." -ForegroundColor Green

    Write-Host "Building with verbose-log feature..." -ForegroundColor Yellow
    bun tauri build -- --features verbose-log
} finally { Pop-Location }
