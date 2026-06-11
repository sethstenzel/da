# da

`da` is a fast, cross-platform directory alias manager. Map short names to long paths, print them on demand, open them with any tool, and change into them from any shell.

---

## Installation

### Windows
Download `da-<version>-installer.exe` from the [releases page](https://github.com/sethstenzel/da/releases) and run it.

The installer:
- Installs `da.exe` to `%LOCALAPPDATA%\Programs\da\`
- Adds that directory to your user PATH (no admin required)
- Optionally installs the `dacd` shell function to your PowerShell profiles

### Linux (Debian / Ubuntu)
```bash
sudo dpkg -i da_<version>_amd64.deb
```

### Linux (Arch)
```bash
cd installer/arch
makepkg -si
```

### macOS (Homebrew)
```bash
brew tap sethstenzel/da
brew install sethstenzel/da/da
```

### From source
See [build-install.md](build-install.md) for full instructions on all platforms.

---

## Quick start

```bash
da add gpd C:\s3711\git        # add an alias
da gpd                         # prints C:\s3711\git
da gpd/da                      # prints C:\s3711\git\da
dacd gpd                       # cd into C:\s3711\git
da gpd -e                      # open in file manager
da gpd -code                   # open in VS Code
```

---

## Commands

### Aliases

| Command | Description |
|---|---|
| `da add <alias> <path>` | Add or update an alias |
| `da add` | Add an alias interactively |
| `da <alias>` | Print the path for an alias |
| `da <alias>/<subpath>` | Print the path with a subpath appended |
| `da ls` / `da list` | List all aliases |
| `da delete <alias>` | Delete an alias (`remove` and `del` also work) |
| `da <text>` | Search aliases by name if no exact match is found |

Paths support environment variable expansion when adding:
- Windows style: `%USERPROFILE%\projects`
- Unix style: `$HOME/projects` or `${HOME}/projects`

### Open commands

Open commands let you launch a tool directly on an alias path using a `-flag` suffix.

| Command | Description |
|---|---|
| `da <alias> -<cmd>` | Open the alias path with the named command |
| `da cmds ls` | List all open commands |
| `da cmds add <name> <executable>` | Add or update an open command |
| `da cmds delete <name>` | Delete an open command |

**Default open commands:**

| Flag | Tool | Notes |
|---|---|---|
| `-e` | File manager | `explorer` (Windows), `open` (macOS), `xdg-open` (Linux) |
| `-code` | VS Code | `code` |
| `-nvim` | Neovim | `nvim` |

Add your own:
```bash
da cmds add rider rider64       # JetBrains Rider
da cmds add sub sublime_text    # Sublime Text
da myproject -rider             # open with Rider
```

### Export / Import

```bash
da export                       # writes aliases_<timestamp>.json and commands_<timestamp>.json
da import aliases_2024-01-01_12-00-00.json
da import commands_2024-01-01_12-00-00.json
```

Import overwrites any conflicting aliases or commands. The JSON files include a `"type"` field (`"aliases"` or `"commands"`) so `da import` knows how to handle each file.

### Shell integration

`dacd <alias>` changes your current directory to the alias path. Because `cd` must run in the current shell, `dacd` is a shell function rather than a binary.

**Install automatically (Windows):**
The installer adds `dacd` to your PowerShell profiles. To add it manually at any time:
```powershell
powershell -ExecutionPolicy Bypass -File installer\setup_shell.ps1
```

**Install automatically (Linux / macOS):**
```bash
bash installer/setup_shell.sh
source ~/.bashrc    # or ~/.zshrc
```

**See the function to add manually:**
```bash
da shell-init
```

---

## Subpath appending

Any path separator after the alias name appends a subpath to the stored path:

```bash
da add gpd C:\s3711\git
da gpd/da/da/src          # → C:\s3711\git\da\da\src
da gpd/da/da/src -nvim    # open that path in nvim
dacd gpd/da               # cd into C:\s3711\git\da
```

Both `\` and `/` work as separators on all platforms.

---

## Data storage

Aliases and commands are stored in a SQLite database:

| Platform | Path |
|---|---|
| Windows | `%LOCALAPPDATA%\da\a.db` |
| Linux | `~/.local/share/da/a.db` |
| macOS | `~/Library/Application Support/da/a.db` |

---

## Building

See [build-install.md](build-install.md) for build instructions on all platforms.  
See [RELEASING.md](RELEASING.md) for the release process.
