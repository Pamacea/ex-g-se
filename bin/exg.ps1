#!/usr/bin/env pwsh

<#
.SYNOPSIS
    EXG - EX-G-SE CLI
.DESCRIPTION
    Ghost Mode Observability tool
.EXAMPLE
    exg              Show help
    exg config       Configure AI provider
    exg record       Start recording
#>

param(
    [Parameter(Position = 0)]
    [string]$Command
)

switch ($Command) {
    "config" {
        $ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
        & node "$ScriptDir\config.js" @args
        exit $LASTEXITCODE
    }
    "record" {
        $ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
        & node "$ScriptDir\index.js" @args
        exit $LASTEXITCODE
    }
    default {
        $ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
        & node "$ScriptDir\index.js" @args
        exit $LASTEXITCODE
    }
}
