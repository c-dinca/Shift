use shift_detect;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Subcommand)]
#[derive(Clone)]
enum Commands {
    Status{
        #[arg(short, long, default_value = ".")]
        path: String,
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
fn main() {

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Status { path }) =>
            match shift_detect::detect(String::from(path))
            {
                Ok(stack) => {
                    println!("{}", "Detected stack:".bold().cyan());

                    match stack.node_version {
                        Some(version) => println!("{}", format!("Node version: {}", version).green()),
                        None => println!("{}", "Node version: not specified".red()),
                    }

                    let important = ["react", "next", "typescript", "express",
                        "vue", "angular", "tailwindcss", "vite"];

                    for pkg in &stack.packages {
                       if important.contains(&pkg.name.as_str()) {
                           println!("  {} {}@{}", "▶".cyan(), pkg.name.bold().green(), pkg.version.yellow() )
                       }else {
                           println!("  - {}@{}", pkg.name, pkg.version.dimmed());
                       }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        None => {println!("{}", "No command specified".red())},
    }
}
