@echo off
setlocal enabledelayedexpansion
REM EX-G-SE Main Entry Point

set "SCRIPT_DIR=%~dp0"
set "NODE_EXE=node"

REM Check if first argument is "config"
if "%~1"=="config" (
    "%NODE_EXE%" "!SCRIPT_DIR!config.js" %*
    exit /b !ERRORLEVEL!
)

REM Default: run main
"%NODE_EXE%" "!SCRIPT_DIR!index.js" %*
exit /b %ERRORLEVEL%
