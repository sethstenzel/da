# Release Process

## 1. Bump the version

Update the version number in all four places:

| File | Field |
|---|---|
| `da/Cargo.toml` | `version = "x.y.z"` |
| `installer/da.nsi` | `!define VERSION "x.y.z"` |
| `installer/arch/PKGBUILD` | `pkgver=x.y.z` |
| `installer/homebrew/da.rb` | version in `url` |

---

## 2. Run tests

```powershell
cd da
cargo test
```

All tests must pass before proceeding.

---

## 3. Build the Windows installer

```powershell
.\build_installer.ps1
```

Produces `installer\da-<version>-installer.exe`.

---

## 4. Commit and tag

```powershell
git add -A
git commit -m "release v<version>"
git tag v<version>
git push origin main --tags
```

---

## 5. Create the GitHub release

```powershell
gh release create v<version> `
    installer/da-<version>-installer.exe `
    --title "v<version>" `
    --notes "Release notes here."
```

Or use the GitHub web UI at https://github.com/sethstenzel/da/releases/new.

---

## 6. Get the SHA256 for the Homebrew formula

After the release is published, compute the checksum of the source tarball:

**PowerShell:**
```powershell
$version = "x.y.z"
$url = "https://github.com/sethstenzel/da/archive/v$version.tar.gz"
$tmp = "$env:TEMP\da-$version.tar.gz"
Invoke-WebRequest $url -OutFile $tmp
(Get-FileHash $tmp -Algorithm SHA256).Hash.ToLower()
Remove-Item $tmp
```

**bash (Linux / macOS):**
```bash
version="x.y.z"
curl -sL "https://github.com/sethstenzel/da/archive/v$version.tar.gz" | shasum -a 256
```

Copy the resulting hash into `installer/homebrew/da.rb`:
```ruby
sha256 "paste_hash_here"
```

Then update the `url` line version number to match:
```ruby
url "https://github.com/sethstenzel/da/archive/vX.Y.Z.tar.gz"
```

---

## 7. Update the Arch PKGBUILD checksum

The PKGBUILD `sha256sums` field should also be updated with the same hash:

```
sha256sums=('paste_hash_here')
```

Then submit the updated PKGBUILD to the AUR if it is published there.

---

## 8. Publish the Homebrew formula

If you maintain a Homebrew tap at `https://github.com/sethstenzel/homebrew-da`:

```bash
# In the homebrew-da tap repo
cp /path/to/da/installer/homebrew/da.rb Formula/da.rb
git add Formula/da.rb
git commit -m "da x.y.z"
git push
```

Users can then install with:
```bash
brew tap sethstenzel/da
brew install da
```
