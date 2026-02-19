#Requires -Version 5.1
<#
.SYNOPSIS
    Runs Wisp in development mode.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

Push-Location $root
try { bun tauri dev } finally { Pop-Location }
