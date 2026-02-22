#!/usr/bin/env pwsh

<#
.SYNOPSIS
    EX-G-SE - Main Entry Point
.DESCRIPTION
    Start ghost mode recording session
.EXAMPLE
    npx @oalacea/ex-g-se
.EXAMPLE
    npx @oalacea/ex-g-se config
#>

param(
    [Parameter(Position = 0)]
    [string]$Command
)

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Handle subcommand
if ($Command -eq "config") {
    & node "$ScriptDir\config.js" @args
    exit $LASTEXITCODE
}

# Default: run main recording
& node "$ScriptDir\index.js" @args
exit $LASTEXITCODE
