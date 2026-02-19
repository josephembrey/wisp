#Requires -Version 5.1
<#
.SYNOPSIS
    Builds and signs Wisp with verbose logging enabled.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

& "$PSScriptRoot\build-debug.ps1"
& "$PSScriptRoot\sign.ps1"
