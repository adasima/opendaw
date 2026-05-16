# Build script for INDAW distribution
$ErrorActionPreference = "Stop"

Write-Host "🚧 Starting Build Process for INDAW..."
Write-Host "   Cleaning dist folder..."

# 1. Clean Dist Folder
if (Test-Path "dist") {
    Remove-Item -Recurse -Force "dist"
}
New-Item -ItemType Directory "dist" | Out-Null
New-Item -ItemType Directory "dist/Portable_Folder" | Out-Null
New-Item -ItemType Directory "dist/Single_Exe" | Out-Null

Write-Host "   Running Tauri Build..."

# 2. Run Build
# Note: Ensure you are in the project root
npm run tauri build

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
}

Write-Host "📦 Packaging Artifacts..."

# 3. Define Paths
$targetDir = "src-tauri/target/release"
$bundleDir = "$targetDir/bundle/nsis"

# 4. Copy Portable Build (Folder Version)
# Assuming 'indaw.exe' and potentially 'WebView2Loader.dll' or resources are needed.
# Since frontend assets are bundled into the binary, we mostly need the exe.
Write-Host "   Creating Portable Folder version..."
Copy-Item "$targetDir/tauri-app.exe" -Destination "dist/Portable_Folder/INDAW.exe"
# Check for WebView2Loader.dll just in case (usually statically linked or system provided, but if present copy it)
if (Test-Path "$targetDir/WebView2Loader.dll") {
    Copy-Item "$targetDir/WebView2Loader.dll" -Destination "dist/Portable_Folder/"
}
# Copy any sidecars or resources if they exist (example check)
if (Test-Path "$targetDir/resources") {
    Copy-Item -Recurse "$targetDir/resources" -Destination "dist/Portable_Folder/"
}

# 5. Zip the portable folder
Compress-Archive -Path "dist/Portable_Folder/*" -DestinationPath "dist/INDAW_Portable_Folder.zip"

Write-Host "✅ Build & Packaging Complete!"
Write-Host "   output: ./dist/Portable_Folder/"
