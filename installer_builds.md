# Building the Installer

## Prerequisites

### Rust toolchain
Required on all platforms:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
```

---

## Windows

### Additional prerequisites
- NSIS 3.x: `winget install NSIS.NSIS`

### Build
```powershell
.\build_installer.ps1
```
Produces `installer\da-<version>-installer.exe`.

> `makensis` is not on PATH by default — the script uses the full path automatically.

### What the installer does
- Installs `da.exe` to `%LOCALAPPDATA%\Programs\da\`
- Appends that directory to the user PATH via PowerShell (no admin, no 1024-char limit)
- Optionally installs the `dacd` shell function to PowerShell 5 and 7 profiles (checked by default)
- Writes an Add/Remove Programs entry

---

## Linux (Debian / Ubuntu)

### Build binary
```bash
chmod +x build_linux.sh
./build_linux.sh
```

### Build .deb package
```bash
cargo install cargo-deb   # one-time
./build_linux.sh          # will auto-build .deb if cargo-deb is installed
```

### Install .deb
```bash
sudo dpkg -i da/target/debian/da_<version>_amd64.deb
```

### Shell integration (dacd)
```bash
da shell-init
source ~/.bashrc   # or ~/.zshrc
```

---

## Linux (Arch)

The `installer/arch/PKGBUILD` is intended for AUR publication.

### Local build/install via makepkg
```bash
cd installer/arch
makepkg -si
```

### Shell integration (dacd)
```bash
da shell-init
source ~/.bashrc   # or ~/.zshrc
```

---

## macOS

### Build binary
```bash
chmod +x build_macos.sh
./build_macos.sh
```

### Homebrew tap (once published)
```bash
brew tap sethstenzel/da
brew install sethstenzel/da/da
```

The formula is at `installer/homebrew/da.rb`. Before publishing:
1. Update the `url` and `homepage` with the real GitHub repo
2. Replace `sha256 "PLACEHOLDER"` with the actual checksum: `curl -L <url> | shasum -a 256`

### Shell integration (dacd)
```bash
da shell-init
source ~/.zshrc   # or ~/.bashrc
```

---

## Updating the version

Update in both places to keep them in sync:

1. `da/Cargo.toml` — `version = "x.y.z"`
2. `installer/da.nsi` — `!define VERSION "x.y.z"` (Windows installer)
3. `installer/arch/PKGBUILD` — `pkgver=x.y.z`
4. `installer/homebrew/da.rb` — `url` version + `sha256`
