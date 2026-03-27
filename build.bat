@echo off
chcp 65001 >nul
title CS2Checker Build


where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo [!] Rust not found. Installing...
    winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements
    if %errorlevel% neq 0 (
        echo [ERROR] Failed to install Rust. Install manually: https://rustup.rs
        pause
        exit /b 1
    )
    echo [*] Restart this script after Rust installation completes.
    pause
    exit /b 0
)

where node >nul 2>&1
if %errorlevel% neq 0 (
    echo [!] Node.js not found. Installing...
    winget install OpenJS.NodeJS.LTS --accept-source-agreements --accept-package-agreements
    if %errorlevel% neq 0 (
        echo [ERROR] Failed to install Node.js. Install manually: https://nodejs.org
        pause
        exit /b 1
    )
    echo [*] Restart this script after Node.js installation completes.
    pause
    exit /b 0
)

echo [OK] Rust:
rustc --version
echo [OK] Node:
node --version
echo.

echo   Building Launcher (deti00checker-v2)
cd /d "%~dp0launcher\src-tauri"
echo [*] Compiling Rust backend...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Launcher build failed!
    pause
    exit /b 1
)
echo [OK] Launcher built successfully
echo.

echo   Building Checker (cs2-checker)
cd /d "%~dp0checker\src-tauri"
echo [*] Compiling Rust backend...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Checker build failed!
    pause
    exit /b 1
)
echo [OK] Checker built successfully
echo.

echo   Copying binaries

cd /d "%~dp0"
if not exist "output" mkdir "output"

copy /Y "launcher\src-tauri\target\release\deti00checker-v2.exe" "output\deti00checker_v2.exe" >nul
copy /Y "checker\src-tauri\target\release\cs2-checker.exe" "output\cs2checker.exe" >nul

echo.
echo   BUILD COMPLETE
echo.
echo   Launcher: output\deti00checker_v2.exe
echo   Checker:  output\cs2checker.exe
echo.
echo   To run: launch deti00checker_v2.exe
echo.

pause
