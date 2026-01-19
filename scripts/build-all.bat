@echo off
REM Build script for all platforms (Windows CMD)

echo 🚀 Building restor-octo-giggle for all platforms...
echo.

cd /d "%~dp0\..\rust_samples"

REM Build Windows target
echo Building for Windows (x86_64)...
cargo build --release --target x86_64-pc-windows-msvc

if %ERRORLEVEL% EQU 0 (
    echo ✓ Windows build successful
) else (
    echo ❌ Windows build failed
    exit /b 1
)

echo.
echo ✅ Build completed!
echo.
echo Build artifacts are in:
echo   target\x86_64-pc-windows-msvc\release\
