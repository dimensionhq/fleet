echo Downloading Fleet installer...
# Download the installer
curl https://cdn.dimension.dev/dimension/fleet/bin/installer.bin -o installer.bin
# Make it an executable
chmod +x installer.bin
# Run the installer
./installer.bin