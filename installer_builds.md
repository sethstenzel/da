# Building the Installer

## Prerequisites

### 1. NSIS 3.x
Install via winget (no admin required):
```powershell
winget install NSIS.NSIS
```

### 2. Release binary
Build `da.exe` from the `da/` directory:
```powershell
cd da
cargo build --release
cd ..
```

---

## Build the installer

From the repo root:
```powershell
cd installer
& "C:\Program Files (x86)\NSIS\makensis.exe" da.nsi
```

This produces `installer\da-<version>-installer.exe`.

> `makensis` is not added to PATH by default — use the full path above.

---

## What the installer does

- Installs `da.exe`, `path_add.ps1`, and `path_remove.ps1` to `%LOCALAPPDATA%\Programs\da\`
- Appends that directory to the **user PATH** via PowerShell (no admin required, no 1024-char limit)
- Writes an entry to **Add/Remove Programs** under the current user

## What the uninstaller does

- Removes the install directory from the user PATH
- Deletes all installed files and the install directory
- Removes the Add/Remove Programs entry

---

## Updating the version

Update the version in both files to keep them in sync:

1. `da/Cargo.toml` — `version = "x.y.z"`
2. `installer/da.nsi` — `!define VERSION "x.y.z"`
