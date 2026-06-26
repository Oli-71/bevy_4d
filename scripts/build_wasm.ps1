param(
    [string]$Profile = "wasm-release",
    [switch]$NoCopy,
    [switch]$CopyOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptRoot
Set-Location $repoRoot

$bindgenOutput = Join-Path $repoRoot "out"
$docsOut = Join-Path $repoRoot "docs\out"
$docsAssets = Join-Path $repoRoot "docs\assets"

function Copy-ToDocs {
    param(
        [string]$SourceOut,
        [string]$SourceAssets
    )

    Write-Host "[build_wasm] Copying generated output into docs..."

    if (Test-Path $docsOut) { Remove-Item $docsOut -Recurse -Force }
    if (Test-Path $docsAssets) { Remove-Item $docsAssets -Recurse -Force }

    New-Item -ItemType Directory -Path $docsOut | Out-Null
    Copy-Item -Path "$SourceOut\*" -Destination $docsOut -Recurse -Force

    New-Item -ItemType Directory -Path $docsAssets | Out-Null
    Copy-Item -Path "$SourceAssets\*" -Destination $docsAssets -Recurse -Force
}

if ($CopyOnly) {
    if (-not (Test-Path $bindgenOutput)) {
        throw "Generated output directory not found: $bindgenOutput. Run the build step first."
    }

    Copy-ToDocs -SourceOut $bindgenOutput -SourceAssets (Join-Path $repoRoot "assets")
    Write-Host "[build_wasm] Done."
    return
}

Write-Host "`n[build_wasm] Building WebAssembly target using profile '$Profile'..."
cargo build --profile $Profile --target wasm32-unknown-unknown

$wasmPath = Join-Path $repoRoot "target\wasm32-unknown-unknown\release\bevy_4d.wasm"
if (-not (Test-Path $wasmPath)) {
    throw "WebAssembly file not found: $wasmPath"
}

Write-Host "[build_wasm] Running wasm-bindgen -> $bindgenOutput"
$wasmBindgen = Get-Command wasm-bindgen -ErrorAction SilentlyContinue
if (-not $wasmBindgen) {
    throw "wasm-bindgen CLI not found. Install it with `"cargo install wasm-bindgen-cli`"."
}
& $wasmBindgen.Path --out-dir $bindgenOutput --target web $wasmPath

if (-not $NoCopy) {
    Copy-ToDocs -SourceOut $bindgenOutput -SourceAssets (Join-Path $repoRoot "assets")
}

Write-Host "[build_wasm] Done."
