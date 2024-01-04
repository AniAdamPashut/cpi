use clap::{Arg, ArgAction, Command};
use modules::prelude::*;

fn main() {
    let matches = Command::new("cpi")
        .about("C Package Index utility")
        .version("0.2.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("benAmi")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .action(ArgAction::SetTrue)
                        .help("view package information"),
                )
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        // Remove command
        // 
        // Only a few of the args are implemented
        .subcommand(
            Command::new("remove")
                .short_flag('R')
                .long_flag("remove")
                .about("Removing packages.")
                .arg(
                    Arg::new("package")
                        .help("the packages to remove")
                        .action(ArgAction::Set)
                        .num_args(1..)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .filter_map(|it| Module::new(it.to_owned()).ok())
                .collect();


            if sync_matches.get_flag("info") {
                packages
                .iter()
                .filter_map(|module| module.get_info().ok())
                .for_each(|it| println!("\n=============================\n{}\n=============================\n", it.trim()));
            } else {
                packages
                .iter()
                .for_each(|module| module.install().unwrap_or(()));
            }
        }
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("info") {
                packages
                .map(|string| string.to_owned())
                .filter_map(|it| Module::new(it).ok())
                .filter_map(|module| module.get_info().ok())
                .for_each(|it| println!("\n=============================\n{}\n=============================\n", it.trim()));
            } else {
                println!("Displaying all locally installed packages...");
            }
        }

        Some(("remove", remove_matches)) => {
            let packages: Vec<_> = remove_matches
                .get_many::<String>("package")
                .expect("is present")
                .filter_map(|it| Module::new(it.to_owned()).ok())
                .collect();

            packages
            .iter()
            .for_each(|it| it.uninstall().unwrap_or(()))
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}