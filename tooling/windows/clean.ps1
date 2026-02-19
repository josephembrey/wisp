#Requires -Version 5.1
<#
.SYNOPSIS
    Cleans build artifacts for Wisp.
.DESCRIPTION
    .\win.ps1 clean        - cleans Rust target + frontend build output
    .\win.ps1 clean all    - also removes node_modules (full reset)
#>

param(
    [Parameter(Position = 0)]
    [ValidateSet('', 'all')]
    [string]$Scope
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

function Remove-Dir($Path, $Label) {
    if (Test-Path $Path) {
        Write-Host "  Removing $Label ($Path)..." -ForegroundColor Yellow
        Remove-Item -Recurse -Force $Path
    }
}

Write-Host "`nCleaning build artifacts..." -ForegroundColor Cyan

Remove-Dir $env:CARGO_TARGET_DIR           'Cargo target'
Remove-Dir (Join-Path $root '.svelte-kit') 'SvelteKit cache'
Remove-Dir (Join-Path $root 'build')       'Vite build output'
Remove-Dir (Join-Path $root 'src-tauri\gen\schemas') 'Tauri generated schemas'

if ($Scope -eq 'all') {
    Remove-Dir (Join-Path $root 'node_modules') 'Node modules'
}

Write-Host "`nClean." -ForegroundColor Green
