$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$kernelElf = Join-Path $root "target\aarch64-unknown-none\release\kernel"
$outDir = Join-Path $root "target\raspi5"
$kernelImg = Join-Path $outDir "kernel_2712.img"
$kernel8Img = Join-Path $outDir "kernel8.img"

Push-Location $root
try {
    cargo build -p kernel --release

    New-Item -ItemType Directory -Force -Path $outDir | Out-Null

    $sysroot = rustc --print sysroot
    $objcopy = Join-Path $sysroot "lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-objcopy.exe"
    if (-not (Test-Path $objcopy)) {
        throw "llvm-objcopy.exe not found. Install it with: rustup component add llvm-tools-preview"
    }

    & $objcopy -O binary $kernelElf $kernelImg
    Copy-Item -Force $kernelImg $kernel8Img

    Write-Host "Generated:"
    Write-Host "  $kernelImg"
    Write-Host "  $kernel8Img"
}
finally {
    Pop-Location
}
