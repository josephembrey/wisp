#Requires -Version 5.1
<#
.SYNOPSIS
    Signs the built Wisp executable using Azure Trusted Signing.
.DESCRIPTION
    Requires: Windows SDK (signtool), Azure Trusted Signing Client Tools, Azure CLI.
    Run .\win.ps1 install sign to install these.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\_env.ps1"

& "$root\signing\sign.ps1"
