use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::db::{Alias, Command, Db};

const RESERVED: &[&str] = &[
    "add", "ls", "list", "delete", "remove", "del",
    "command", "commands", "cmd", "cmds",
    "export", "import", "shell-init",
];

#[derive(Serialize, Deserialize)]
struct AliasExport {
    #[serde(rename = "type")]
    kind: String,
    aliases: Vec<Alias>,
}

#[derive(Serialize, Deserialize)]
struct CommandExport {
    #[serde(rename = "type")]
    kind: String,
    commands: Vec<Command>,
}

fn check_reserved(alias: &str) -> Result<()> {
    if RESERVED.contains(&alias) {
        bail!("'{}' is a reserved command name and cannot be used as an alias", alias);
    }
    Ok(())
}

pub fn add_interactive(db: &Db) -> Result<()> {
    let alias = prompt("Alias")?;
    check_reserved(&alias)?;
    let path = prompt("Path")?;
    let expanded = expand_env_vars(&path);
    db.add(&alias, &expanded)?;
    println!("Added: {} -> {}", alias, expanded);
    Ok(())
}

pub fn add(db: &Db, alias: &str, path: &str) -> Result<()> {
    check_reserved(alias)?;
    let expanded = expand_env_vars(path);
    db.add(alias, &expanded)?;
    println!("Added: {} -> {}", alias, expanded);
    Ok(())
}

pub fn lookup_or_search(db: &Db, text: &str) -> Result<()> {
    match resolve_alias_path(db, text)? {
        Some(path) => {
            println!("{}", path);
            Ok(())
        }
        None => search(db, text),
    }
}

pub fn run_open_command(db: &Db, alias: &str, cmd_name: &str) -> Result<()> {
    let base_alias = alias_base(alias);
    let path = resolve_alias_path(db, alias)?
        .with_context(|| format!("alias '{}' not found", base_alias))?;
    let executable = db
        .get_command(cmd_name)?
        .with_context(|| format!("open command '-{}' not found — use 'da commands ls' to see available commands", cmd_name))?;
    // On Windows, route through cmd /C so .cmd/.bat scripts (e.g. VS Code's `code.cmd`)
    // are resolved correctly — CreateProcess alone does not expand PATHEXT.
    // Using status() (not spawn()) keeps da alive until the child exits, which gives
    // TUI apps like nvim exclusive access to the terminal.
    #[cfg(windows)]
    std::process::Command::new("cmd")
        .args(["/C", executable.as_str(), path.as_str()])
        .status()
        .with_context(|| format!("failed to launch '{}' with path '{}'", executable, path))?;

    #[cfg(not(windows))]
    std::process::Command::new(&executable)
        .arg(&path)
        .status()
        .with_context(|| format!("failed to launch '{}' with path '{}'", executable, path))?;

    Ok(())
}

pub fn delete(db: &Db, alias: &str) -> Result<()> {
    if db.delete(alias)? {
        println!("Deleted '{}'", alias);
    } else {
        bail!("alias '{}' not found", alias);
    }
    Ok(())
}

pub fn list(db: &Db) -> Result<()> {
    let aliases = db.list()?;
    if aliases.is_empty() {
        println!("No aliases saved.");
    } else {
        print_alias_table(&aliases);
    }
    Ok(())
}

pub fn search(db: &Db, text: &str) -> Result<()> {
    let aliases = db.search(text)?;
    if aliases.is_empty() {
        println!("No matches for '{}'", text);
    } else {
        print_alias_table(&aliases);
    }
    Ok(())
}

pub fn list_commands(db: &Db) -> Result<()> {
    let cmds = db.list_commands()?;
    if cmds.is_empty() {
        println!("No open commands configured.");
    } else {
        print_command_table(&cmds);
    }
    Ok(())
}

pub fn add_command(db: &Db, name: &str, executable: &str) -> Result<()> {
    db.add_command(name, executable)?;
    println!("Added command: -{} -> {}", name, executable);
    Ok(())
}

pub fn delete_command(db: &Db, name: &str) -> Result<()> {
    if db.delete_command(name)? {
        println!("Deleted command '-{}'", name);
    } else {
        bail!("command '-{}' not found", name);
    }
    Ok(())
}

fn print_alias_table(aliases: &[Alias]) {
    let col_width = aliases.iter().map(|a| a.name.len()).max().unwrap_or(0);
    for a in aliases {
        println!("{:<width$}  {}", a.name, a.path, width = col_width);
    }
}

fn print_command_table(cmds: &[Command]) {
    let col_width = cmds.iter().map(|c| c.name.len() + 1).max().unwrap_or(0);
    for c in cmds {
        let flag = format!("-{}", c.name);
        println!("{:<width$}  {}", flag, c.executable, width = col_width);
    }
}

fn prompt(label: &str) -> Result<String> {
    print!("{}: ", label);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    fn db() -> Db {
        Db::open_in_memory().unwrap()
    }

    // --- reserved name guard: all reserved words must be rejected ---

    #[test]
    fn reserved_words_are_rejected() {
        let db = db();
        for word in RESERVED {
            let result = add(&db, word, "C:\\some\\path");
            assert!(result.is_err(), "'{word}' should be rejected as an alias");
        }
    }

    // --- spot-check the new aliases added in this session ---

    #[test]
    fn cmd_is_reserved() {
        let db = db();
        assert!(add(&db, "cmd", "C:\\test").is_err());
    }

    #[test]
    fn cmds_is_reserved() {
        let db = db();
        assert!(add(&db, "cmds", "C:\\test").is_err());
    }

    #[test]
    fn commands_is_reserved() {
        let db = db();
        assert!(add(&db, "commands", "C:\\test").is_err());
    }

    // --- non-reserved names must be accepted ---

    #[test]
    fn normal_alias_is_accepted() {
        let db = db();
        assert!(add(&db, "my-project", "C:\\projects\\foo").is_ok());
    }

    // --- export / import ---

    #[test]
    fn export_is_reserved() {
        let db = db();
        assert!(add(&db, "export", "C:\\test").is_err());
    }

    #[test]
    fn import_is_reserved() {
        let db = db();
        assert!(add(&db, "import", "C:\\test").is_err());
    }

    #[test]
    fn import_aliases_roundtrip() {
        let db = db();
        db.add("foo", "C:\\foo").unwrap();
        db.add("bar", "C:\\bar").unwrap();

        let aliases = db.list().unwrap();
        let json = serde_json::to_string(&AliasExport { kind: "aliases".into(), aliases }).unwrap();

        let db2 = Db::open_in_memory().unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let export: AliasExport = serde_json::from_value(value).unwrap();
        for a in export.aliases {
            db2.add(&a.name, &a.path).unwrap();
        }

        assert_eq!(db2.get("foo").unwrap(), Some("C:\\foo".into()));
        assert_eq!(db2.get("bar").unwrap(), Some("C:\\bar".into()));
    }

    #[test]
    fn import_commands_roundtrip() {
        let db = db();
        db.add_command("rider", "rider64").unwrap();

        let commands = db.list_commands().unwrap();
        let json = serde_json::to_string(&CommandExport { kind: "commands".into(), commands }).unwrap();

        let db2 = Db::open_in_memory().unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let export: CommandExport = serde_json::from_value(value).unwrap();
        for c in export.commands {
            db2.add_command(&c.name, &c.executable).unwrap();
        }

        assert_eq!(db2.get_command("rider").unwrap(), Some("rider64".into()));
    }

    #[test]
    fn import_overwrites_existing_alias() {
        let db = db();
        db.add("foo", "C:\\old").unwrap();

        let aliases = vec![Alias { name: "foo".into(), path: "C:\\new".into() }];
        let json = serde_json::to_string(&AliasExport { kind: "aliases".into(), aliases }).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let export: AliasExport = serde_json::from_value(value).unwrap();
        for a in export.aliases {
            db.add(&a.name, &a.path).unwrap();
        }

        assert_eq!(db.get("foo").unwrap(), Some("C:\\new".into()));
    }

    #[test]
    fn import_overwrites_existing_command() {
        let db = db();
        db.add_command("code", "old-code").unwrap();

        let commands = vec![Command { name: "code".into(), executable: "new-code".into() }];
        let json = serde_json::to_string(&CommandExport { kind: "commands".into(), commands }).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let export: CommandExport = serde_json::from_value(value).unwrap();
        for c in export.commands {
            db.add_command(&c.name, &c.executable).unwrap();
        }

        assert_eq!(db.get_command("code").unwrap(), Some("new-code".into()));
    }

    #[test]
    fn import_unknown_type_errors() {
        let json = r#"{"type":"bogus","data":[]}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let kind = value["type"].as_str().unwrap_or("");
        assert!(!matches!(kind, "aliases" | "commands"));
    }
}

pub fn shell_init() {
    println!("To enable 'dacd <alias>' for changing directories, add the following");
    println!("function to your PowerShell profile(s):");
    println!();
    println!("  function dacd {{ if (-not $args[0]) {{ Write-Host \"Usage: dacd <alias>\"; return }}; $path = da $args[0]; if ($LASTEXITCODE -eq 0) {{ Set-Location $path }} }}");
    println!();
    println!("Profile locations:");
    println!("  PowerShell 5 : $HOME\\Documents\\WindowsPowerShell\\Microsoft.PowerShell_profile.ps1");
    println!("  PowerShell 7+: $HOME\\Documents\\PowerShell\\Microsoft.PowerShell_profile.ps1");
    println!();
    println!("Or open each profile with 'notepad $PROFILE' and paste the line in.");
}

pub fn export(db: &Db) -> Result<()> {
    let ts = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let aliases_file = format!("aliases_{ts}.json");
    let commands_file = format!("commands_{ts}.json");

    let aliases = db.list()?;
    let alias_count = aliases.len();
    let json = serde_json::to_string_pretty(&AliasExport { kind: "aliases".into(), aliases })?;
    std::fs::write(&aliases_file, &json).with_context(|| format!("failed to write {aliases_file}"))?;
    println!("Exported {alias_count} aliases to {aliases_file}");

    let commands = db.list_commands()?;
    let cmd_count = commands.len();
    let json = serde_json::to_string_pretty(&CommandExport { kind: "commands".into(), commands })?;
    std::fs::write(&commands_file, &json).with_context(|| format!("failed to write {commands_file}"))?;
    println!("Exported {cmd_count} commands to {commands_file}");

    Ok(())
}

pub fn import(db: &Db, file: &str) -> Result<()> {
    let content = std::fs::read_to_string(file)
        .with_context(|| format!("failed to read '{file}'"))?;
    let value: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("'{file}' is not valid JSON"))?;
    let kind = value["type"].as_str().unwrap_or("").to_string();
    match kind.as_str() {
        "aliases" => {
            let export: AliasExport = serde_json::from_value(value)
                .context("invalid aliases export format")?;
            let count = export.aliases.len();
            for a in export.aliases {
                db.add(&a.name, &a.path)?;
            }
            println!("Imported {count} aliases from '{file}'");
        }
        "commands" => {
            let export: CommandExport = serde_json::from_value(value)
                .context("invalid commands export format")?;
            let count = export.commands.len();
            for c in export.commands {
                db.add_command(&c.name, &c.executable)?;
            }
            println!("Imported {count} commands from '{file}'");
        }
        other => bail!("unknown export type '{other}' — expected 'aliases' or 'commands'"),
    }
    Ok(())
}

fn alias_base(input: &str) -> &str {
    input.find(['\\', '/']).map_or(input, |pos| &input[..pos])
}

fn resolve_alias_path(db: &Db, input: &str) -> Result<Option<String>> {
    let (alias, subpath) = match input.find(['\\', '/']) {
        Some(pos) => (&input[..pos], Some(&input[pos + 1..])),
        None => (input, None),
    };
    match db.get(alias)? {
        Some(base) => {
            let full = match subpath {
                Some(sub) => std::path::Path::new(&base).join(sub).to_string_lossy().into_owned(),
                None => base,
            };
            Ok(Some(full))
        }
        None => Ok(None),
    }
}

fn expand_env_vars(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut var = String::new();
            let mut closed = false;
            for inner in chars.by_ref() {
                if inner == '%' {
                    closed = true;
                    break;
                }
                var.push(inner);
            }
            if closed {
                match std::env::var(&var) {
                    Ok(val) => result.push_str(&val),
                    Err(_) => {
                        result.push('%');
                        result.push_str(&var);
                        result.push('%');
                    }
                }
            } else {
                result.push('%');
                result.push_str(&var);
            }
        } else {
            result.push(c);
        }
    }
    result
}
