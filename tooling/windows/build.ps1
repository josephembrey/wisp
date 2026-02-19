#Requires -Version 5.1
<#
.SYNOPSIS
    Builds Wisp for production.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

Push-Location $root
try { bun tauri build } finally { Pop-Location }
