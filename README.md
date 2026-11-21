<div align="center">
  <img src="frontend/assets/DistroVault-wordmark.png" alt="DistroVault logo" width="460">

  # DistroVault

  **Your machines. Your distros. Your vault.**

  A private-by-default desktop app for organizing computers, operating systems, and distro experiments.

  **Windows · Linux · Offline-first · Local data**
</div>

---

## What is DistroVault?

DistroVault is a personal workspace for people who maintain more than one computer, experiment with Linux distributions, or simply want to remember what's running where.

Keep your hardware collection, installed operating systems, past experiments, favorites, and future test plans together in one place—without creating an account or sending your collection to a cloud service.

## Features

- **Computer inventory** — Record desktops and laptops, including CPU, RAM, graphics, storage, display, and notes.
- **Operating system tracking** — Track Linux, Windows, macOS, and independent operating systems for each computer.
- **Distro library** — Search a built-in catalogue of distro names or enter your own.
- **Test queue** — Keep a list of distributions you want to try next.
- **Favorites** — Star your favorite computers and distros.
- **Custom themes** — Choose a theme or personalize the app's colors.
- **Backup and restore** — Export and import your collection as JSON.

## Your collection. Your data. Your control. 🔒

DistroVault is **private by default**:

- No account is needed.
- Your collection is saved locally on your device.
- The app can be used offline.
- Export and import backups whenever you choose.
- New installations start with an empty collection—no personal hardware records are bundled with the source.

DistroVault does not provide cloud synchronization, so collections on different computers remain separate unless you transfer a backup.

## Platforms

| Platform | Package | Tested status |
| --- | --- | --- |
| Windows | NSIS `.exe` installer | Built, installed, and tested |
| Fedora KDE | `.rpm` | Built, installed, and tested |
| Linux | `.AppImage` | Built and launched on Fedora; other distributions may need testing |

The AppImage is intended for broader Linux compatibility, but it is not guaranteed to work on every distribution or system configuration.

## Build from source

DistroVault is built using **Tauri 2**, **Rust**, and **HTML/CSS/JavaScript**.

### Prerequisites

Install [Node.js](https://nodejs.org/), [Rust](https://rustup.rs/), and the applicable [Tauri system prerequisites](https://v2.tauri.app/start/prerequisites/). Windows builds require the Microsoft C++ build tools and Windows SDK; Linux builds require the appropriate WebKitGTK and system development libraries.

From the directory containing `package.json`:

```bash
npm install
npm run desktop:dev
```

### Windows installer

On Windows, run:

```powershell
npm run desktop:windows
```

The NSIS installer is generated under `src-tauri/target/release/bundle/nsis/`.

### Fedora RPM

On Fedora, run:

```bash
npx tauri build --bundles rpm
```

Find the RPM under `src-tauri/target/release/bundle/rpm/`.

### Linux AppImage

On Linux, run:

```bash
npx tauri build --bundles appimage
```

Find the AppImage under `src-tauri/target/release/bundle/appimage/`. Make the file executable before launching it if your file manager requires that.

## Backups and updates

DistroVault stores its desktop data in the operating system's application-data location rather than inside the application installer. Use **Backup & Settings → Export Backup** to create a portable JSON copy of your collection.

When upgrading, keep a backup and **do not choose to delete application data** if the installer offers that option.

## Project structure

```text
frontend/        App interface, themes, and graphics
src-tauri/       Rust backend, Tauri configuration, and desktop icons
package.json     Development and build commands
README.md        Project documentation
```

## Feedback and contributions

Have an idea, found a bug, or want to improve the distro catalogue? GitHub Issues are a good place to discuss it when access to the repository is available.

---

