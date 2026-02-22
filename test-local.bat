@echo off
REM Local test script for EX-G-SE (Windows)

echo ================================================================
echo [EX-G-SE] Local Testing Script
echo ================================================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust/Cargo not found. Please install Rust first.
    echo Visit: https://rustup.rs/
    exit /b 1
)

echo [1/4] Building Rust core...
cd core
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Rust build failed
    cd ..
    exit /b 1
)
echo [OK] Rust core built successfully
cd ..

echo.
echo [2/4] Setting up binary directory...
if not exist bin mkdir bin

REM Copy binary
copy core\target\release\ex-g-se.exe bin\ex-g-se.exe >nul
if %errorlevel% neq 0 (
    echo [ERROR] Failed to copy binary
    exit /b 1
)
echo [OK] Binary copied to bin\ex-g-se.exe

echo.
echo [3/4] Running tests...
cd core
cargo test --quiet
if %errorlevel% neq 0 (
    echo [ERROR] Tests failed
    cd ..
    exit /b 1
)
echo [OK] All tests passed
cd ..

echo.
echo [4/4] Testing binary execution...
echo [INFO] Creating test directory...
set TEST_DIR=%TEMP%\ex-g-se-test-%RANDOM%
mkdir "%TEST_DIR%"
cd /d "%TEST_DIR%"

echo [INFO] Starting EX-G-SE in test mode...
echo [INFO] The binary will run for 5 seconds...
echo.

REM Run for 5 seconds (using timeout)
start /b "" ..\..\bin\ex-g-se.exe
timeout /t 5 /nobreak >nul
taskkill /F /IM ex-g-se.exe >nul 2>&1

if exist raw_logs.json (
    echo.
    echo [OK] Logs generated successfully!
    echo [INFO] Log file: %TEST_DIR%\raw_logs.json
    echo.
    echo [INFO] First 20 lines of logs:
    powershell -Command "Get-Content raw_logs.json -Head 20"
    echo.
    echo [SUCCESS] EX-G-SE is working correctly!
) else (
    echo.
    echo [WARNING] No logs generated (expected if stopped immediately)
)

cd /d ..
rmdir /s /q "%TEST_DIR%" 2>nul

echo.
echo ================================================================
echo [EX-G-SE] Test Complete
echo ================================================================
echo.
echo Next steps:
echo   1. Run manually: .\bin\ex-g-se.exe
echo   2. Or use npm: npm run dev
echo   3. Build all platforms: npm run build
echo.
