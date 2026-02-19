#Requires -Version 5.1
<#
.SYNOPSIS
    Builds and signs Wisp for production.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

& "$PSScriptRoot\build.ps1"
& "$PSScriptRoot\sign.ps1"
