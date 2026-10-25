#!/bin/bash

# Install script for Simple Calendar Linux desktop integration

set -e

APP_NAME="simple-calendar"
DESKTOP_FILE="simple-calendar.desktop"
ICON_FILE="simple-calendar.svg"

# Determine install locations
if [ -d "$HOME/.local/share/applications" ]; then
    DESKTOP_DIR="$HOME/.local/share/applications"
else
    DESKTOP_DIR="$HOME/.local/share/applications"
    mkdir -p "$DESKTOP_DIR"
fi

if [ -d "$HOME/.local/share/icons/hicolor/scalable/apps" ]; then
    ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"
else
    ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"
    mkdir -p "$ICON_DIR"
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Copy desktop file
echo "Installing desktop file..."
cp "$SCRIPT_DIR/$DESKTOP_FILE" "$DESKTOP_DIR/"

# Update Exec path to point to the actual binary location
# Assuming binary is in the same directory as this script's parent
sed -i "s|Exec=simple-calendar|Exec=$SCRIPT_DIR/../simple-calendar|" "$DESKTOP_DIR/$DESKTOP_FILE"

# Copy icon
echo "Installing icon..."
cp "$SCRIPT_DIR/$ICON_FILE" "$ICON_DIR/${APP_NAME}.svg"

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    echo "Updating desktop database..."
    update-desktop-database "$DESKTOP_DIR"
fi

echo "Installation complete!"
echo "You can now find Simple Calendar in your applications menu."
