@echo off
REM build-all.bat - Cross-compile Rust binaries for all platforms (Windows)
REM
REM This is the Windows equivalent of build-all.sh
REM Builds the EX-G-SE binary for multiple target platforms:
REM - Linux (x86_64) - requires cross-compilation setup
REM - Windows (x86_64)
REM - macOS (x86_64 and ARM64) - requires cross-compilation setup
REM
REM REQUIREMENTS:
REM - Rust toolchain with rustup
REM - For cross-compilation: cross tool or appropriate toolchains
REM
REM CROSS-COMPILATION LIMITATIONS:
REM - Linux targets require MinGW or similar on Windows
REM - macOS targets cannot be cross-compiled from Windows (use GitHub Actions or macOS)
REM - aarch64-apple-darwin (Apple Silicon) requires macOS
REM
REM Usage: build-all.bat [--dry-run]

SETLOCAL EnableDelayedExpansion

REM Configuration
SET PROJECT_NAME=ex-g-se
SET SCRIPT_DIR=%~dp0
SET PROJECT_ROOT=%SCRIPT_DIR:~0,-8%
SET DIST_DIR=%PROJECT_ROOT%bin

REM Parse arguments
SET DRY_RUN=0
IF "%1"=="--dry-run" SET DRY_RUN=1

IF %DRY_RUN%==1 (
    echo DRY RUN MODE: No actual builds will be performed
    echo.
)

REM Colors (not supported in standard cmd, but we use echo prefixes)
SET INFO=[INFO]
SET SUCCESS=[OK]
SET WARNING=[WARNING]
SET ERROR=[ERROR]

echo ============================================================
echo EX-G-SE Cross-Compilation Build Script ^(Windows^)
echo ============================================================
echo.
echo Project root: %PROJECT_ROOT%
echo Output directory: %DIST_DIR%
echo.

REM Check prerequisites
echo %INFO% Checking prerequisites...
where rustup >nul 2>nul
IF ERRORLEVEL 1 (
    echo %ERROR% rustup not found. Please install Rust from https://rustup.rs/
    EXIT /B 1
)

where cargo >nul 2>nul
IF ERRORLEVEL 1 (
    echo %ERROR% cargo not found. Please install Rust from https://rustup.rs/
    EXIT /B 1
)

echo %SUCCESS% Rust toolchain found
echo.

REM Create output directory
echo %INFO% Creating output directory...
IF NOT EXIST "%DIST_DIR%" mkdir "%DIST_DIR%"
echo %SUCCESS% Output directory created: %DIST_DIR%
echo.

REM Build for Windows x64 (native)
echo %INFO% Processing target: x86_64-pc-windows-msvc
echo   Adding target: x86_64-pc-windows-msvc
IF %DRY_RUN%==0 (
    rustup target add x86_64-pc-windows-msvc >nul 2>nul
)

echo   Building for target: x86_64-pc-windows-msvc
IF %DRY_RUN%==1 (
    echo   [DRY RUN] Would run: cargo build --release --target x86_64-pc-windows-msvc
) ELSE (
    CD /D "%PROJECT_ROOT%"

    REM Check if core/Cargo.toml exists
    IF EXIST "core\Cargo.toml" (
        CD core
    ) ELSE IF NOT EXIST "Cargo.toml" (
        echo %ERROR% Cargo.toml not found. Please run from project root or ensure core\ exists.
        EXIT /B 1
    )

    cargo build --release --target x86_64-pc-windows-msvc
    IF ERRORLEVEL 1 (
        echo %ERROR% Build failed for x86_64-pc-windows-msvc
        EXIT /B 1
    )

    echo   %SUCCESS% Build successful
    CD /D "%PROJECT_ROOT%"
)

echo.

REM Copy Windows binary to dist directory
IF %DRY_RUN%==0 (
    echo %INFO% Copying binaries to output directory...

    SET BINARY_PATH=target\x86_64-pc-windows-msvc\release\%PROJECT_NAME%.exe

    REM Handle different project structures
    IF EXIST "core\%BINARY_PATH%" (
        SET BINARY_PATH=core\%BINARY_PATH%
    ) ELSE IF NOT EXIST "%BINARY_PATH%" (
        echo %WARNING% Binary not found: %BINARY_PATH%
    )

    IF EXIST "%BINARY_PATH%" (
        copy "%BINARY_PATH%" "%DIST_DIR%\%PROJECT_NAME%-win-x64.exe" >nul
        IF NOT ERRORLEVEL 1 (
            echo   %SUCCESS% Copied: %PROJECT_NAME%-win-x64.exe
        )
    )

    echo.
    echo ============================================================
    echo Build Complete
    echo ============================================================
    echo Binaries are available in: %DIST_DIR%
    DIR /B "%DIST_DIR%"
) ELSE (
    echo ============================================================
    echo Dry Run Complete
    echo ============================================================
    echo No builds were performed. Run without --dry-run to execute builds.
)

echo.
REM Note about cross-platform builds
echo %INFO% Cross-compilation notes:
echo   - Windows builds: Supported natively
echo   - Linux builds: Requires cross-compilation setup ^(MinGW/cross^)
echo   - macOS builds: Not possible from Windows ^(use GitHub Actions or macOS^)
echo   - For full cross-platform builds, use GitHub Actions
echo.

ENDLOCAL
