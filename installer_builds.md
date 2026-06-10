# Building the Installer

## Prerequisites

### 1. NSIS 3.x
Install via winget (no admin required):
```powershell
winget install NSIS.NSIS
```

### 2. Release binary
Build `da.exe` in release mode from the `da/` directory:
```powershell
cd da
cargo build --release
cd ..
```

---

## Building the installer

From the `installer/` directory, run:
```powershell
cd installer
makensis da.nsi
```

This produces `installer\da-0.1.0-installer.exe`.

---

## What the installer does

- Installs `da.exe` to `%LOCALAPPDATA%\Programs\da\`
- Appends that directory to the **user PATH** (no admin required)
- Broadcasts a `WM_WININICHANGE` message so running terminals pick up the PATH change
- Writes an entry to **Add/Remove Programs** under the current user

## What the uninstaller does

- Removes all occurrences of the install directory from the user PATH
- Deletes `da.exe` and the install directory
- Removes the Add/Remove Programs entry

---

## Updating the version

The version is defined once at the top of `installer/da.nsi`:
```nsis
!define VERSION "0.1.0"
```

Update this to match the version in `da/Cargo.toml` before building a new release.
