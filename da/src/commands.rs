use anyhow::{bail, Context, Result};
use std::io::{self, Write};

use crate::db::{Alias, Command, Db};

const RESERVED: &[&str] = &["add", "ls", "delete", "remove", "del", "commands"];

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
    match db.get(text)? {
        Some(path) => {
            println!("{}", path);
            Ok(())
        }
        None => search(db, text),
    }
}

pub fn run_open_command(db: &Db, alias: &str, cmd_name: &str) -> Result<()> {
    let path = db
        .get(alias)?
        .with_context(|| format!("alias '{}' not found", alias))?;
    let executable = db
        .get_command(cmd_name)?
        .with_context(|| format!("open command '-{}' not found — use 'da commands ls' to see available commands", cmd_name))?;
    std::process::Command::new(&executable)
        .arg(&path)
        .spawn()
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
