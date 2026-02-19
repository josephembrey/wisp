#Requires -Version 5.1
<#
.SYNOPSIS
    Runs compilation and lint checks for Wisp.
.DESCRIPTION
    .\win.ps1 check          - runs all checks
    .\win.ps1 check rust     - cargo check only
    .\win.ps1 check svelte   - svelte-check only
    .\win.ps1 check lint     - eslint + prettier only
#>

param(
    [Parameter(Position = 0)]
    [ValidateSet('', 'rust', 'svelte', 'lint')]
    [string]$Scope
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"
$all = -not $Scope

function Invoke-Rust {
    Write-Host "`n[cargo check] Checking Rust compilation..." -ForegroundColor Cyan
    cargo check --manifest-path "$root\src-tauri\Cargo.toml"
}

function Invoke-Svelte {
    Write-Host "`n[svelte-check] Checking SvelteKit types..." -ForegroundColor Cyan
    Push-Location $root
    try { bun run check } finally { Pop-Location }
}

function Invoke-Lint {
    Write-Host "`n[prettier] Checking formatting..." -ForegroundColor Cyan
    Push-Location $root
    try { bun run prettier --check . } finally { Pop-Location }

    Write-Host "`n[eslint] Linting..." -ForegroundColor Cyan
    Push-Location $root
    try { bun run eslint . } finally { Pop-Location }
}

if ($all -or $Scope -eq 'rust')   { Invoke-Rust }
if ($all -or $Scope -eq 'svelte') { Invoke-Svelte }
if ($all -or $Scope -eq 'lint')   { Invoke-Lint }

Write-Host "`nAll checks passed." -ForegroundColor Green
