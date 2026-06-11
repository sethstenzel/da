# Building and Installing from Source

## Prerequisites (all platforms)

**Rust toolchain** — install via [rustup](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
```
On Windows you can also use: `winget install Rustlang.Rustup`

---

## Download the source

**Clone:**
```bash
git clone https://github.com/sethstenzel/da.git
cd da
```

**Or download a release tarball:**
```bash
# Replace x.y.z with the desired version
curl -L https://github.com/sethstenzel/da/archive/vx.y.z.tar.gz | tar xz
cd da-x.y.z
```

**Or use the GitHub CLI:**
```bash
gh repo clone sethstenzel/da
cd da
```

---

## Windows

### Additional prerequisites
- NSIS 3.x (only needed to build the installer): `winget install NSIS.NSIS`

### Option A — Build the installer
```powershell
.\build_installer.ps1
```
This produces `installer\da-<version>-installer.exe`. Run it to install `da.exe` to `%LOCALAPPDATA%\Programs\da\` and add it to your user PATH.

The installer optionally adds the `dacd` shell function to your PowerShell profiles (checked by default). You can also run this step manually at any time:
```powershell
da shell-init
```

### Option B — Build and install manually
```powershell
cd da
cargo build --release
```
Then copy the binary somewhere on your PATH:
```powershell
Copy-Item target\release\da.exe "$env:LOCALAPPDATA\Programs\da\da.exe"
```
Add `dacd` to your PowerShell profiles:
```powershell
da shell-init
```
Restart your terminal.

---

## Linux (Debian / Ubuntu)

### Build binary
```bash
chmod +x build_linux.sh
./build_linux.sh
```
The binary is at `da/target/release/da`.

### Install binary
```bash
sudo cp da/target/release/da /usr/local/bin/da
```

### Build and install a .deb package
```bash
cargo install cargo-deb        # one-time
./build_linux.sh               # auto-builds .deb if cargo-deb is present
sudo dpkg -i da/target/debian/da_*.deb
```

### Shell integration (dacd)
```bash
da shell-init
source ~/.bashrc    # or ~/.zshrc
```

---

## Linux (Arch)

### Build and install via makepkg
```bash
cd installer/arch
makepkg -si
```
This downloads the source, builds `da`, and installs it to `/usr/bin/da`.

### Shell integration (dacd)
```bash
da shell-init
source ~/.bashrc    # or ~/.zshrc
```

---

## macOS

### Build binary
```bash
chmod +x build_macos.sh
./build_macos.sh
```
The binary is at `da/target/release/da`.

### Install binary
```bash
sudo cp da/target/release/da /usr/local/bin/da
# or for Apple Silicon / Homebrew prefix:
sudo cp da/target/release/da /opt/homebrew/bin/da
```

### Shell integration (dacd)
```bash
da shell-init
source ~/.zshrc     # or ~/.bashrc
```

---

## Verify the installation

```bash
da
```
Should print the usage/help text. Then try:
```bash
da add myproject /path/to/project
da myproject        # prints the path
dacd myproject      # changes into the directory
```
