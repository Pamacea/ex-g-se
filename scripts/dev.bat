@echo off
REM dev.bat - Local development build and run script (Windows)
REM
REM This script builds the Rust binary for the current platform
REM and runs the CLI for local development and testing.
REM
REM REQUIREMENTS:
REM - Rust toolchain with cargo
REM - Node.js and npm (for CLI wrapper)
REM
REM Usage:
REM   dev.bat                - Build and run for current platform
REM   dev.bat --skip-build   - Skip building, just run
REM   dev.bat --help         - Show CLI help

SETLOCAL EnableDelayedExpansion

REM Configuration
SET PROJECT_NAME=ex-g-se
SET SCRIPT_DIR=%~dp0
SET PROJECT_ROOT=%SCRIPT_DIR:~0,-8%
SET CORE_DIR=%PROJECT_ROOT%core
SET CLI_DIR=%PROJECT_ROOT%cli
SET BIN_DIR=%PROJECT_ROOT%bin

REM Parse arguments
SET SKIP_BUILD=0
SET CLI_ARGS=

:parse_args
IF "%~1"=="--skip-build" (
    SET SKIP_BUILD=1
    SHIFT
    GOTO parse_args
)
IF NOT "%~1"=="" (
    SET CLI_ARGS=%CLI_ARGS% %1
    SHIFT
    GOTO parse_args
)

echo ============================================================
echo EX-G-SE Development Environment ^(Windows^)
echo ============================================================
echo.
echo Project root: %PROJECT_ROOT%
echo.

REM Step 1: Build binary
IF %SKIP_BUILD%==0 (
    echo Step 1: Building %PROJECT_NAME% for current platform...
    echo.

    REM Check if core directory exists
    IF NOT EXIST "%CORE_DIR%" (
        echo Error: core\ directory not found
        EXIT /B 1
    )

    CD /D "%CORE_DIR%"

    cargo build --release
    IF ERRORLEVEL 1 (
        echo Error: Build failed
        EXIT /B 1
    )

    echo Build successful
    echo.

    REM Create bin directory and copy binary
    IF NOT EXIST "%BIN_DIR%" mkdir "%BIN_DIR%"

    COPY "target\release\%PROJECT_NAME%.exe" "%BIN_DIR%\%PROJECT_NAME%.exe" >nul
    IF ERRORLEVEL 1 (
        echo Error: Failed to copy binary
        EXIT /B 1
    )

    echo Binary copied to: %BIN_DIR%\%PROJECT_NAME%.exe
    echo.

    CD /D "%PROJECT_ROOT%"
) ELSE (
    echo Step 1: Skipping build...
    echo.
)

REM Step 2: Set up CLI
echo Step 2: Setting up CLI environment...
echo.

REM Check if CLI directory exists
IF NOT EXIST "%CLI_DIR%" (
    echo Warning: cli\ directory not found
    echo Running binary directly...

    SET BINARY_PATH=%BIN_DIR%\%PROJECT_NAME%.exe

    IF NOT EXIST "%BINARY_PATH%" (
        echo Error: Binary not found at %BINARY_PATH%
        EXIT /B 1
    )

    REM Run binary directly
    echo.
    echo Running: %BINARY_PATH%%CLI_ARGS%
    echo.
    "%BINARY_PATH%" %CLI_ARGS%
    EXIT /B 0
)

REM Check if CLI dependencies are installed
IF NOT EXIST "%CLI_DIR%\node_modules" (
    echo CLI dependencies not found. Installing...
    CD /D "%CLI_DIR%"
    CALL npm install
    IF ERRORLEVEL 1 (
        echo Error: Failed to install dependencies
        EXIT /B 1
    )
    CD /D "%PROJECT_ROOT%"
    echo Dependencies installed
    echo.
)

echo CLI ready
echo.

REM Step 3: Run application
echo Step 3: Running application...
echo.

CD /D "%CLI_DIR%"

IF "%CLI_ARGS%"=="" (
    echo Running without arguments ^(use --help for usage^)
    echo.
    node bin\index.js
) ELSE (
    echo Running with arguments:%CLI_ARGS%
    echo.
    node bin\index.js %CLI_ARGS%
)

ENDLOCAL
