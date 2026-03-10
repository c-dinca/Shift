use shift_detect;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
#[derive(Clone)]
enum Commands {
    Status,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
fn main() {

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Status) =>
            match shift_detect::detect(String::from("./package.json"))
            {
                Ok(stack) => {
                    println!("Detected stack:");

                    match stack.node_version {
                        Some(version) => println!("Node version: {}", version),
                        None => println!("Node version: not specified"),
                    }
                    println!("Packages: {}", stack.packages.join(","));
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        None => {println!("No command specified")},
    }
}
