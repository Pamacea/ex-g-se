@echo off
setlocal enabledelayedexpansion
REM EXG - EX-G-SE CLI

set "SCRIPT_DIR=%~dp0"
set "NODE_EXE=node"
set "COMMAND=%~1"

if "%COMMAND%"=="config" (
    "%NODE_EXE%" "!SCRIPT_DIR!config.js"
    exit /b !ERRORLEVEL!
)

if "%COMMAND%"=="record" (
    "%NODE_EXE%" "!SCRIPT_DIR!index.js"
    exit /b !ERRORLEVEL!
)

if "%COMMAND%"=="update" (
    npm update -g @oalacea/ex-g-se
    exit /b !ERRORLEVEL!
)

REM Default: show help
"%NODE_EXE%" "!SCRIPT_DIR!index.js"
exit /b %ERRORLEVEL%
