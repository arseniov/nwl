use clap::{Parser, Subcommand};
use nwl_compiler::build_project;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "nwl")]
#[command(author = "NWL Team")]
#[command(version = "0.1.0")]
#[command(about = "NWL compiler for React", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "build")]
    Build {
        #[arg(default_value = ".")]
        input: PathBuf,
    },
    #[command(name = "compile")]
    Compile {
        #[arg(short, long)]
        output: Option<PathBuf>,
        file: PathBuf,
    },
    #[command(name = "new")]
    New { name: String },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Build { input } => {
            println!("Building NWL project in {}", input.display());
            if !input.exists() {
                eprintln!("Error: Directory not found: {}", input.display());
                std::process::exit(1);
            }
            match build_project(input) {
                Ok(()) => {
                    println!("Build successful! Routes generated automatically.");
                }
                Err(e) => {
                    eprintln!("Build error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Compile { output, file } => {
            if !file.exists() {
                eprintln!("Error: File not found: {}", file.display());
                std::process::exit(1);
            }

            let content = std::fs::read_to_string(&file).expect("Failed to read file");
            match nwl_compiler::compile(&content) {
                Ok(output_code) => {
                    if let Some(output_path) = output {
                        std::fs::write(&output_path, &output_code).expect("Failed to write output");
                        println!("Compiled to {}", output_path.display());
                    } else {
                        println!("{}", output_code);
                    }
                }
                Err(e) => {
                    eprintln!("Compilation error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::New { name } => {
            println!("Creating new NWL project: {}", name);
        }
    }
}
