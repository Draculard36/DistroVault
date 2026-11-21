# DistroVault Desktop — Tauri starter (Windows + Linux)

This is the **desktop-development project**, not the working browser/Python release.
It contains the DistroVault v13 interface, Favorites, themes, distro library, and icon assets adapted for Tauri 2.

**Keep DistroVault v13 unchanged.** First export its collection using Settings → Export Backup and store that JSON safely.

## Windows — first run

1. Install Node.js LTS, Rust (MSVC toolchain), and Visual Studio C++ Build Tools with a Windows SDK.
2. Open a Command Prompt in this folder (where `package.json` is located).
3. Run `npm install`.
4. Run `npm run desktop:dev`.
5. The desktop app should open in its own window. Use Settings → Import Backup to import your v13 JSON backup.

To build a Windows installer: `npm run desktop:windows`.
Output should be under `src-tauri/target/release/bundle/nsis`.

## Linux

Install the system libraries required by Tauri (WebKitGTK and build dependencies), plus Node.js and Rust.
Run `npm install` and `npm run desktop:dev` in the project directory.
Run `npm run desktop:linux` for AppImage/deb packages (dependencies may vary by distro).

## Data & privacy

Desktop saves use native application data directories and `DistroVault-data.json`.
The project does **not** contain your personal collection or backups; the first-run collection starts empty.
The app creates a backup of the previous save as `DistroVault-data.previous.json`.

## Status

This is **source code**, not a prebuilt installer. The Rust/Tauri build still needs to be compiled and tested on your Windows PC.
Do not publish publicly before testing and choosing a license.
