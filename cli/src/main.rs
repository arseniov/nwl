use clap::{Parser, Subcommand};
use nwl_compiler::compile_file;
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
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(default_value = "pages")]
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
        Commands::Build { output, input } => {
            println!("Building from {}", input.display());
            if !input.exists() {
                eprintln!("Error: Input directory not found: {}", input.display());
                std::process::exit(1);
            }
            println!("Build command ready - compiles YAML pages to React");
        }
        Commands::Compile { output, file } => {
            if !file.exists() {
                eprintln!("Error: File not found: {}", file.display());
                std::process::exit(1);
            }

            match compile_file(file.clone()) {
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
