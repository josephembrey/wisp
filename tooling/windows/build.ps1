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
    Write-Host "Generating TypeScript bindings..." -ForegroundColor Cyan
    cargo run --manifest-path src-tauri/Cargo.toml --bin generate_bindings
    Write-Host "Bindings generated." -ForegroundColor Green

    bun tauri build
} finally { Pop-Location }
