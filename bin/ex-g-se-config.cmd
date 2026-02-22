@echo off
setlocal enabledelayedexpansion
REM EX-G-SE Configuration

set "SCRIPT_DIR=%~dp0"
set "NODE_EXE=node"

"%NODE_EXE%" "!SCRIPT_DIR!config.js" %*
exit /b %ERRORLEVEL%
