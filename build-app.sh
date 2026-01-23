#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Creating app bundle..."
APP_NAME="Humanboard"
APP_DIR="$APP_NAME.app"

rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy the binary
cp target/release/humanboard "$APP_DIR/Contents/MacOS/Humanboard"

# Copy the icon
cp assets/AppIcon.icns "$APP_DIR/Contents/Resources/AppIcon.icns"

# Copy assets folder (with icons)
cp -r assets "$APP_DIR/Contents/Resources/assets"

# Copy themes folder
cp -r themes "$APP_DIR/Contents/Resources/themes"

# Copy lib folder (for PDFium)
cp -r lib "$APP_DIR/Contents/Resources/lib"

# Create Info.plist
cat > "$APP_DIR/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Humanboard</string>
    <key>CFBundleDisplayName</key>
    <string>Humanboard</string>
    <key>CFBundleIdentifier</key>
    <string>com.humancorp.humanboard</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>Humanboard</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.productivity</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2025 Humancorp</string>
</dict>
</plist>
EOF

echo "Creating DMG..."
rm -f Humanboard.dmg
rm -rf dmg_temp
mkdir dmg_temp
cp -r Humanboard.app dmg_temp/
ln -s /Applications dmg_temp/Applications
hdiutil create -volname "Humanboard" -srcfolder dmg_temp -ov -format UDZO Humanboard.dmg
rm -rf dmg_temp

echo ""
echo "Done! Created:"
ls -lh Humanboard.app Humanboard.dmg
