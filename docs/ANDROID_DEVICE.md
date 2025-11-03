# Installing on Physical Android Device

This guide explains how to install and run the Tauri Hello World app on your physical Android phone.

## Method 1: USB Installation (Recommended)

### Step 1: Enable Developer Options on Your Phone

1. **Open Settings** on your Android phone
2. **Navigate to About Phone** (usually under Settings > About Phone or Settings > System > About Phone)
3. **Find "Build Number"** (may be under Software Information)
4. **Tap "Build Number" 7 times**
   - You'll see a message: "You are now a developer!"
5. **Go back** to Settings main menu

### Step 2: Enable USB Debugging

1. **Open Settings** > **System** > **Developer Options**
   - On some devices: Settings > Developer Options (directly)
2. **Toggle "Developer Options" ON** (if not already enabled)
3. **Enable "USB Debugging"**
4. **Enable "Install via USB"** (if available - some devices have this option)
5. **(Optional) Enable "USB Debugging (Security Settings)"** for first-time installation

### Step 3: Connect Your Phone to Computer

1. **Connect your Android phone** to your Mac via USB cable
2. **Unlock your phone**
3. **Tap "Allow" or "OK"** when the "Allow USB debugging?" prompt appears
   - Check "Always allow from this computer" for convenience
4. **Select "File Transfer" or "MTP"** mode if prompted (swipe down notification panel)

### Step 4: Verify Connection

```bash
# Check if device is detected
adb devices

# Expected output:
# List of devices attached
# ABC123XYZ    device
```

If your device shows as "unauthorized", check your phone for the authorization prompt.

### Step 5: Install the APK

Once the build completes, install the APK:

```bash
# Install the universal APK (works on all devices)
adb install src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk

# Note: For production builds with signing, use the signed APK instead
```

### Step 6: Launch the App

**Option A: From Phone**
1. Look for "Tauri Hello World" app icon on your home screen or app drawer
2. Tap the icon to launch

**Option B: From Command Line**
```bash
adb shell am start -n com.raibid.tauri.hello/.MainActivity
```

---

## Method 2: Wireless Installation (No Cable)

### Prerequisites

- Android 11 or higher
- Phone and computer on the same WiFi network

### Step 1: Enable Wireless Debugging

1. **Open Settings** > **Developer Options**
2. **Enable "Wireless Debugging"**
3. **Tap "Wireless Debugging"** to enter settings
4. **Note the IP address and port** (e.g., 192.168.1.100:45678)

### Step 2: Pair Computer with Phone

```bash
# Method A: Using pairing code
adb pair <IP_ADDRESS>:<PORT>
# Enter the 6-digit pairing code shown on your phone

# Method B: Using QR code (if available)
# Scan the QR code shown in Wireless Debugging settings
```

### Step 3: Connect Wirelessly

```bash
# Connect to the device
adb connect <IP_ADDRESS>:<PORT>

# Verify connection
adb devices
```

### Step 4: Install APK Wirelessly

```bash
adb install src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
```

---

## Method 3: Direct APK Transfer

### Step 1: Transfer APK to Phone

**Option A: Email**
1. Email the APK file to yourself
2. Open email on your phone
3. Download the APK attachment

**Option B: Cloud Storage**
1. Upload APK to Google Drive, Dropbox, etc.
2. Download from cloud storage app on your phone

**Option C: USB File Transfer**
1. Connect phone via USB
2. Select "File Transfer" mode
3. Copy APK to Downloads folder

### Step 2: Install from File Manager

1. **Open Files** or **Downloads** app on your phone
2. **Tap the APK file**
3. **Tap "Install"**
4. If you see "For security, your phone is not allowed to install unknown apps from this source":
   - Tap "Settings"
   - Enable "Allow from this source"
   - Go back and tap "Install" again
5. Wait for installation to complete
6. Tap "Open" to launch the app

---

## Method 4: Development Mode (Live Reload)

For active development with live reload:

```bash
# Ensure device is connected via USB or WiFi
export ANDROID_HOME=~/Library/Android/sdk
export NDK_HOME=$ANDROID_HOME/ndk/27.2.12479018
export PATH="/opt/homebrew/opt/openjdk@21/bin:$PATH"
export JAVA_HOME="/opt/homebrew/opt/openjdk@21"

# Run development build
npm run tauri:android
```

This will:
- Build the app
- Install it on your device
- Launch it automatically
- Hot-reload when you make code changes

---

## Updating the App

### Update via ADB

```bash
# Uninstall old version (optional - not required)
adb uninstall com.raibid.tauri.hello

# Install new version
adb install -r src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
```

The `-r` flag reinstalls the app, preserving data.

### Update via Manual Install

1. Build new APK
2. Transfer to phone
3. Tap to install - it will update the existing app

---

## Troubleshooting

### Device Not Detected

```bash
# Restart ADB server
adb kill-server
adb start-server

# Check device list
adb devices
```

**If still not detected:**
1. Try a different USB cable (data cable, not charge-only)
2. Try different USB port on your computer
3. Restart your phone
4. Reinstall USB drivers (if on Windows)
5. Check if USB debugging is still enabled

### "Device Unauthorized"

1. Disconnect and reconnect USB cable
2. Check your phone for authorization prompt
3. Revoke USB debugging authorizations: Developer Options > Revoke USB debugging authorizations
4. Reconnect and authorize again

### "Installation Blocked"

**Error: "App not installed"**
- Free up storage space on your phone
- Uninstall previous version: `adb uninstall com.raibid.tauri.hello`
- Clear app installer cache: Settings > Apps > Package installer > Clear cache

**Error: "Install unknown apps"**
- Enable "Install unknown apps" for your file manager or browser
- Settings > Apps > Special access > Install unknown apps

### "Signature Conflict"

If updating an existing app with different signature:

```bash
# Uninstall old version completely
adb uninstall com.raibid.tauri.hello

# Install new version
adb install app-universal-release.apk
```

### App Crashes on Launch

```bash
# View crash logs
adb logcat | grep -i "tauri"

# OR view all errors
adb logcat *:E
```

Check logs for specific error messages.

### Wireless Debugging Connection Lost

```bash
# Reconnect
adb connect <IP_ADDRESS>:<PORT>

# If IP changed (common on WiFi)
# Check new IP in phone's Wireless Debugging settings
```

---

## Finding Your Device's Architecture

To install the smallest APK for your device:

```bash
# Connect device via ADB
adb devices

# Check CPU architecture
adb shell getprop ro.product.cpu.abi

# Common outputs:
# arm64-v8a   → Use app-arm64-v8a-release.apk
# armeabi-v7a → Use app-armeabi-v7a-release.apk
# x86_64      → Use app-x86_64-release.apk
# x86         → Use app-x86-release.apk
```

**Universal APK** works on all architectures but is larger (~12MB vs ~8MB).

---

## Uninstalling the App

### From Phone

1. Long-press the app icon
2. Tap "Uninstall" or drag to "Uninstall"
3. Confirm

### Via ADB

```bash
adb uninstall com.raibid.tauri.hello
```

---

## Security Considerations

- **Developer Options** expose advanced settings - be careful what you change
- **USB Debugging** allows full device access - only authorize trusted computers
- **Install Unknown Apps** can be a security risk - only install APKs from trusted sources
- **Disable USB Debugging** when not developing to enhance security

---

## Quick Reference

```bash
# Check device connection
adb devices

# Install APK
adb install path/to/app.apk

# Install APK (force reinstall)
adb install -r src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk

# Uninstall app
adb uninstall com.raibid.tauri.hello

# Launch app
adb shell am start -n com.raibid.tauri.hello/.MainActivity

# View logs
adb logcat | grep Tauri

# Check architecture
adb shell getprop ro.product.cpu.abi
```

---

## Additional Resources

- [Android Developer Guide - ADB](https://developer.android.com/tools/adb)
- [Tauri Mobile Documentation](https://tauri.app/v1/guides/building/android)
- [USB Debugging Guide](https://developer.android.com/studio/debug/dev-options)
