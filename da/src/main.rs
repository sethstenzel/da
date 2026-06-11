mod commands;
mod db;

use anyhow::Result;
use db::Db;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let db = Db::open()?;

    match args.as_slice() {
        [] => print_usage(),

        [cmd] if cmd == "add" => commands::add_interactive(&db)?,

        [cmd, alias, path] if cmd == "add" => commands::add(&db, alias, path)?,

        [cmd] if cmd == "ls" || cmd == "list" => commands::list(&db)?,

        [cmd, alias] if cmd == "delete" || cmd == "remove" || cmd == "del" => {
            commands::delete(&db, alias)?
        }

        [cmd, sub] if matches!(cmd.as_str(), "command" | "commands" | "cmd" | "cmds") && sub == "ls" => {
            commands::list_commands(&db)?
        }

        [cmd, sub, name, exe] if matches!(cmd.as_str(), "command" | "commands" | "cmd" | "cmds") && sub == "add" => {
            commands::add_command(&db, name, exe)?
        }

        [cmd, sub, name] if matches!(cmd.as_str(), "command" | "commands" | "cmd" | "cmds") && sub == "delete" => {
            commands::delete_command(&db, name)?
        }

        // da <alias> -<command>  — look up command from the database
        [alias, flag] if flag.starts_with('-') && flag.len() > 1 => {
            commands::run_open_command(&db, alias, &flag[1..])?
        }

        // da cmds / da cmd alone lists all open commands
        [cmd] if matches!(cmd.as_str(), "cmds" | "cmd") => commands::list_commands(&db)?,

        [cmd] if cmd == "shell-init" => commands::shell_init()?,

        [cmd] if cmd == "export" => commands::export(&db)?,

        [cmd, file] if cmd == "import" => commands::import(&db, file)?,

        // exact match prints the path; anything else is treated as a fuzzy search
        [text] => commands::lookup_or_search(&db, text)?,

        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  da add                            # add alias (interactive)");
    println!("  da add <alias> <path>             # add alias");
    println!("  da <alias>                        # print mapped path");
    println!("  da <alias> -<command>             # open path with a command");
    println!("  da delete|remove|del <alias>      # delete alias");
    println!("  da ls                             # list all aliases  (also: list)");
    println!("  da <text>                         # fuzzy search");
    println!("  da command ls                     # list open commands  (also: cmd, cmds, commands)");
    println!("  da command add <name> <exe>       # add/update a command");
    println!("  da command delete <name>          # delete a command");
    println!("  da export                         # export aliases.json and commands.json to current dir");
    println!("  da import <file>                  # import aliases or commands from a .json export file");
    println!("  da shell-init                     # install 'dacd' function into your shell profile(s)");
}
