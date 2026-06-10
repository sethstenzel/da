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

        [cmd] if cmd == "ls" => commands::list(&db)?,

        [cmd, alias] if cmd == "delete" || cmd == "remove" || cmd == "del" => {
            commands::delete(&db, alias)?
        }

        [cmd, sub] if cmd == "commands" && sub == "ls" => commands::list_commands(&db)?,

        [cmd, sub, name, exe] if cmd == "commands" && sub == "add" => {
            commands::add_command(&db, name, exe)?
        }

        [cmd, sub, name] if cmd == "commands" && sub == "delete" => {
            commands::delete_command(&db, name)?
        }

        // da <alias> -<command>  — look up command from the database
        [alias, flag] if flag.starts_with('-') && flag.len() > 1 => {
            commands::run_open_command(&db, alias, &flag[1..])?
        }

        // exact match prints the path; anything else is treated as a fuzzy search
        [text] => commands::lookup_or_search(&db, text)?,

        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  da add                            # interactive");
    println!("  da add <alias> <path>             # non-interactive");
    println!("  da <alias>                        # print mapped path");
    println!("  da <alias> -<command>             # open path with a command");
    println!("  da delete <alias>                 # delete alias");
    println!("  da remove <alias>                 # delete alias");
    println!("  da del <alias>                    # delete alias");
    println!("  da ls                             # list all aliases");
    println!("  da <text>                         # fuzzy search");
    println!("  da commands ls                    # list open commands");
    println!("  da commands add <name> <exe>      # add/update an open command");
    println!("  da commands delete <name>         # delete an open command");
}
