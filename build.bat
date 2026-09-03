@echo off
echo ==========================================
echo GAMING HUB - WINDOWS BUILD SCRIPT
echo ==========================================

echo [1/3] Verifying Node.js and Rust installation...
where npm >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Node.js (npm) is not installed or not in PATH!
    echo Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Rust (cargo) is not installed or not in PATH!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo [2/3] Installing NPM dependencies...
call npm install
if %errorlevel% neq 0 (
    echo ERROR: Failed to install NPM dependencies.
    pause
    exit /b 1
)

echo [3/3] Building Tauri Application (.exe)...
echo This might take a while on the first run as it compiles C++ and Rust dependencies.
call npm run tauri build
if %errorlevel% neq 0 (
    echo ERROR: Build failed. Check the logs above.
    pause
    exit /b 1
)

echo ==========================================
echo BUILD SUCCESSFUL! 🎉
echo ==========================================
echo Your .exe installer is located at:
echo src-tauri\target\release\bundle\nsis\
echo.
pause
