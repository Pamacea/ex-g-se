#!/usr/bin/env pwsh

<#
.SYNOPSIS
    EX-G-SE Configuration
.DESCRIPTION
    Configure AI provider and API key
.EXAMPLE
    npx @oalacea/ex-g-se config
#>

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Run config
& node "$ScriptDir\config.js" @args
exit $LASTEXITCODE
