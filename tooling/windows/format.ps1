#Requires -Version 5.1
<#
.SYNOPSIS
    Auto-formats all code in the Wisp project.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = Resolve-Path "$PSScriptRoot\..\.."

Write-Host "`n[prettier] Formatting frontend..." -ForegroundColor Cyan
Push-Location $root
try { bun run format } finally { Pop-Location }

Write-Host "`n[cargo fmt] Formatting Rust..." -ForegroundColor Cyan
cargo fmt --manifest-path "$root\src-tauri\Cargo.toml"

Write-Host "`nFormatted." -ForegroundColor Green
