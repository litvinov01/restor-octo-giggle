# Build script for all platforms (Windows PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Building restor-octo-giggle for all platforms..." -ForegroundColor Cyan
Write-Host ""

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$RustDir = Join-Path $ProjectRoot "rust_samples"

Set-Location $RustDir

# Check if cross is installed
$UseCross = $false
if (Get-Command cross -ErrorAction SilentlyContinue) {
    $UseCross = $true
} else {
    Write-Host "⚠️  'cross' not found. Install with: cargo install cross" -ForegroundColor Yellow
}

# Function to build target
function Build-Target {
    param(
        [string]$Target,
        [string]$Name
    )
    
    Write-Host "Building for $Name..." -ForegroundColor Blue
    
    if ($UseCross) {
        cross build --release --target $Target
    } else {
        cargo build --release --target $Target
    }
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ $Name build successful" -ForegroundColor Green
    } else {
        Write-Host "❌ $Name build failed" -ForegroundColor Red
        return $false
    }
    Write-Host ""
    return $true
}

# Detect Windows platform
Write-Host "Detected Windows platform" -ForegroundColor Cyan

# Add targets
rustup target add x86_64-pc-windows-msvc 2>$null
rustup target add x86_64-unknown-linux-gnu 2>$null

# Build Windows
Build-Target -Target "x86_64-pc-windows-msvc" -Name "Windows (x86_64)"

# Linux cross-compile requires mingw or Docker
# Build-Target -Target "x86_64-unknown-linux-gnu" -Name "Linux (x86_64)"

Write-Host "✅ Build completed!" -ForegroundColor Green
Write-Host ""
Write-Host "Build artifacts are in:"
Write-Host "  $RustDir\target\x86_64-pc-windows-msvc\release\"
