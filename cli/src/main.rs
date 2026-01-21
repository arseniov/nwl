use clap::{Parser, Subcommand};
use nwl_compiler::build_project;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const GITHUB_REPO_OWNER: &str = "anomaly";
const GITHUB_REPO_NAME: &str = "nwl";
const GITHUB_BASE_URL: &str = "https://raw.githubusercontent.com";

#[derive(Parser, Debug)]
#[command(name = "nwl")]
#[command(author = "NWL Team")]
#[command(version = "0.2.0")]
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
        #[arg(short, long)]
        watch: bool,
        #[arg(long)]
        no_test: bool,
    },
    #[command(name = "compile")]
    Compile {
        #[arg(short, long)]
        output: Option<PathBuf>,
        file: PathBuf,
    },
    #[command(name = "new")]
    New {
        name: String,
        #[arg(short, long)]
        location: Option<PathBuf>,
        #[arg(short, long, default_value = "blank")]
        template: String,
    },
    #[command(name = "dev")]
    Dev {
        #[arg(default_value = ".")]
        input: PathBuf,
        #[arg(short, long, default_value = "5173")]
        port: String,
        #[arg(long, default_value = "localhost")]
        host: String,
        #[arg(long)]
        no_open: bool,
        #[arg(short, long)]
        watch: bool,
        #[arg(long)]
        no_test: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Build {
            input,
            watch,
            no_test,
        } => {
            if watch {
                run_watch_build(input, no_test);
            } else {
                run_build(input, no_test);
            }
        }
        Commands::Compile { output, file } => {
            run_compile(output, file);
        }
        Commands::New {
            name,
            location,
            template,
        } => {
            run_new_project(name, location, template);
        }
        Commands::Dev {
            input,
            port,
            host,
            no_open,
            watch,
            no_test,
        } => {
            run_dev_server(input, port, host, no_open, watch, no_test);
        }
    }
}

fn run_build(input: PathBuf, no_test: bool) {
    println!("Building NWL project in {}", input.display());
    if !input.exists() {
        eprintln!("Error: Directory not found: {}", input.display());
        std::process::exit(1);
    }
    match build_project(input.clone()) {
        Ok(()) => {
            println!("Build successful! Routes generated automatically.");
            if !no_test {
                run_jsx_validation(&input);
            }
        }
        Err(e) => {
            eprintln!("Build error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_watch_build(input: PathBuf, no_test: bool) {
    println!("Watching for changes in {}...", input.display());
    println!("Press Ctrl+C to stop.");

    let mut paths = Vec::new();
    collect_yaml_files(&input, &mut paths);

    if paths.is_empty() {
        eprintln!("No NWL files found in {}", input.display());
        std::process::exit(1);
    }

    println!("Watching {} file(s)", paths.len());

    // Initial build
    run_build(input.clone(), no_test);

    // Simple polling watcher
    let mut last_modified: Vec<(PathBuf, std::time::SystemTime)> = paths
        .iter()
        .filter_map(|p| {
            p.metadata()
                .ok()
                .and_then(|m| m.modified().ok().map(|m| (p.clone(), m)))
        })
        .collect();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        for path in &paths {
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Some((_, last_mod)) = last_modified.iter().find(|(p, _)| p == path) {
                        if modified > *last_mod {
                            println!("\nDetected change in: {}", path.display());
                            run_build(input.clone(), no_test);

                            if let Ok(m) = path.metadata() {
                                if let Ok(mtime) = m.modified() {
                                    if let Some(entry) =
                                        last_modified.iter_mut().find(|(p, _)| p == path)
                                    {
                                        entry.1 = mtime;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn collect_yaml_files(dir: &PathBuf, paths: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_yaml_files(&path, paths);
            } else if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    paths.push(path);
                }
            }
        }
    }
}

fn run_compile(output: Option<PathBuf>, file: PathBuf) {
    if !file.exists() {
        eprintln!("Error: File not found: {}", file.display());
        std::process::exit(1);
    }

    let content = fs::read_to_string(&file).expect("Failed to read file");
    match nwl_compiler::compile(&content) {
        Ok(output_code) => {
            if let Some(output_path) = output {
                fs::write(&output_path, &output_code).expect("Failed to write output");
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

fn run_new_project(name: String, location: Option<PathBuf>, template: String) {
    // Determine project path
    let project_path = match location {
        Some(loc) => loc.join(&name),
        None => PathBuf::from(&name),
    };

    // Check if directory exists
    if project_path.exists() {
        if let Ok(entries) = fs::read_dir(&project_path) {
            let count = entries.count();
            if count > 0 {
                eprintln!(
                    "Error: Directory '{}' already exists and is not empty.",
                    project_path.display()
                );
                eprintln!("Please choose a different location or remove the existing directory.");
                std::process::exit(1);
            }
        }
    }

    println!("Creating new NWL project: {}", name);
    println!("Location: {}", project_path.display());
    println!("Template: {}", template);

    // Create project directory
    if let Err(e) = fs::create_dir_all(&project_path) {
        eprintln!("Error creating project directory: {}", e);
        std::process::exit(1);
    }

    // Try to fetch template from GitHub, fall back to local templates
    let template_url = format!(
        "{}/{}/{}/main/templates/{}/template.zip",
        GITHUB_BASE_URL, GITHUB_REPO_OWNER, GITHUB_REPO_NAME, template
    );

    println!("\nFetching template from GitHub...");

    match fetch_and_extract_template(&template_url, &project_path, &template, &name) {
        Ok(()) => {
            println!("\nProject created successfully!");
            print_next_steps(&project_path);
        }
        Err(e) => {
            eprintln!("Failed to fetch template from GitHub: {}", e);
            eprintln!("Make sure you have an internet connection or try again later.");
            std::process::exit(1);
        }
    }
}

fn fetch_and_extract_template(
    url: &str,
    project_path: &PathBuf,
    template: &str,
    project_name: &str,
) -> Result<(), String> {
    // For now, use local templates as fallback since we don't have GitHub templates yet
    // In production, this would fetch from GitHub and extract the zip

    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = match current_exe.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };

    // Try local templates directory
    let local_templates = exe_dir.join("templates");
    let template_path = local_templates.join(template);

    if !template_path.exists() {
        // Try relative to current working directory
        let cwd_templates = PathBuf::from("templates");
        let cwd_template_path = cwd_templates.join(template);
        if cwd_template_path.exists() {
            copy_template(&cwd_template_path, project_path, project_name)?;
            return Ok(());
        }

        // Try workspace relative paths
        let workspace_templates = PathBuf::from("/home/giacomo/coding/nwl/templates");
        let workspace_template_path = workspace_templates.join(template);
        if workspace_template_path.exists() {
            copy_template(&workspace_template_path, project_path, project_name)?;
            return Ok(());
        }

        return Err(format!("Template '{}' not found locally", template));
    }

    copy_template(&template_path, project_path, project_name)?;
    Ok(())
}

fn copy_template(
    template_path: &PathBuf,
    project_path: &PathBuf,
    project_name: &str,
) -> Result<(), String> {
    println!("Using local template: {}", template_path.display());

    // Copy all files from template
    if let Ok(entries) = fs::read_dir(template_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();

            if path.is_dir() {
                // Recursively copy directory
                let dest_dir = project_path.join(&file_name);
                fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
                copy_directory_recursive(&path, &dest_dir, project_name)?;
            } else {
                // Copy file
                let dest = project_path.join(&file_name);
                if let Ok(content) = fs::read_to_string(&path) {
                    // Replace placeholder in nwl.yaml
                    let processed = if file_name == "nwl.yaml" {
                        content.replace("{PROJECT_NAME}", project_name)
                    } else {
                        content
                    };
                    write_file(&dest, &processed);
                }
            }
        }
    }

    Ok(())
}

fn copy_directory_recursive(
    from: &PathBuf,
    to: &PathBuf,
    project_name: &str,
) -> Result<(), String> {
    if let Ok(entries) = fs::read_dir(from) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();

            let dest = to.join(&file_name);

            if path.is_dir() {
                fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
                copy_directory_recursive(&path, &dest, project_name)?;
            } else {
                if let Ok(content) = fs::read_to_string(&path) {
                    let processed = if file_name == "nwl.yaml" {
                        content.replace("{PROJECT_NAME}", project_name)
                    } else {
                        content
                    };
                    write_file(&dest, &processed);
                }
            }
        }
    }
    Ok(())
}

fn print_next_steps(project_path: &PathBuf) {
    println!("\nNext steps:");
    println!("  cd {}", project_path.display());
    println!("  npm install");
    println!("  nwl dev");
}

fn write_file(path: &PathBuf, content: &str) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).ok();
        }
    }
    if let Err(e) = fs::write(path, content) {
        eprintln!("Error writing to {}: {}", path.display(), e);
        std::process::exit(1);
    }
}

fn run_jsx_validation(input: &PathBuf) {
    println!("\nRunning JSX validation tests...");

    let demo_src_path = input.join("src");
    let test_file_path = demo_src_path.join("compiled-jsx.test.ts");

    if !test_file_path.exists() {
        println!("JSX test file not found, skipping validation.");
        return;
    }

    let status = std::process::Command::new("npx")
        .args(&["vitest", "run", "src/compiled-jsx.test.ts"])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .current_dir(input)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("JSX validation passed!");
        }
        Ok(_) => {
            eprintln!("\nJSX validation failed! Please check the errors above.");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to run JSX validation: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_dev_server(
    input: PathBuf,
    port: String,
    host: String,
    _no_open: bool,
    watch: bool,
    no_test: bool,
) {
    println!("Starting NWL dev server...");
    println!("Port: {}", port);
    println!("Host: {}", host);

    if watch {
        println!("Watch mode: enabled");
    }

    // Build first
    println!("\nBuilding project...");
    run_build(input.clone(), no_test);

    if watch {
        println!("\nStarting Vite dev server with watch mode...");
        println!("Press Ctrl+C to stop.\n");

        std::env::set_current_dir(&input).ok();

        // Start watch build in background thread
        let watch_input = input.clone();
        let watch_no_test = no_test;
        std::thread::spawn(move || {
            run_watch_build(watch_input, watch_no_test);
        });

        // Start Vite dev server
        let status = std::process::Command::new("npx")
            .args(&["vite", "--port", &port, "--host", &host])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(_) => {
                eprintln!("Vite dev server exited with error");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Failed to start dev server: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Start dev server without watch
        println!("\nStarting Vite dev server...");
        println!("Press Ctrl+C to stop.\n");

        std::env::set_current_dir(&input).ok();

        let status = std::process::Command::new("npx")
            .args(&["vite", "--port", &port, "--host", &host])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(_) => {
                eprintln!("Vite dev server exited with error");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Failed to start dev server: {}", e);
                std::process::exit(1);
            }
        }
    }
}
