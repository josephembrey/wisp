#Requires -Version 5.1
<#
.SYNOPSIS
    Wisp Windows tooling entry point.
.USAGE
    .\win.ps1 <command>
.EXAMPLE
    .\win.ps1 install
#>

param(
    [Parameter(Position = 0)]
    [string]$Command,
    [Parameter(Position = 1, ValueFromRemainingArguments)]
    [string[]]$ExtraArgs
)

$scriptDir = "$PSScriptRoot\tooling\windows"

$commands = @{
    install = "$scriptDir\install.ps1"
    dev     = "$scriptDir\dev.ps1"
    build   = "$scriptDir\build.ps1"
    check   = "$scriptDir\check.ps1"
    format  = "$scriptDir\format.ps1"
    clean   = "$scriptDir\clean.ps1"
    sign          = "$scriptDir\sign.ps1"
    'build-sign'  = "$scriptDir\build-sign.ps1"
    'build-debug'      = "$scriptDir\build-debug.ps1"
    'build-sign-debug' = "$scriptDir\build-sign-debug.ps1"
}

if (-not $Command) {
    Write-Host "Usage: .\win.ps1 <command>" -ForegroundColor Cyan
    Write-Host "Available commands: $($commands.Keys -join ', ')"
    exit 0
}

$script = $commands[$Command]
if (-not $script) {
    Write-Error "Unknown command '$Command'. Available: $($commands.Keys -join ', ')"
    exit 1
}

& $script @ExtraArgs
