pub mod codegen;

use crate::codegen::{generate_react, generate_router, get_css_import_path, CodegenError};
use nwl_shared::{Document, Page, ProjectConfig};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Parse(#[from] serde_yaml::Error),
    #[error("Codegen error: {0}")]
    Codegen(#[from] CodegenError),
}

pub fn compile_file(path: PathBuf) -> Result<String, CompilerError> {
    let content = fs::read_to_string(path)?;
    compile_with_css(&content, &None, &None)
}

pub fn compile(input: &str) -> Result<String, CompilerError> {
    compile_with_css(input, &None, &None)
}

pub fn compile_with_css(
    input: &str,
    css_theme: &Option<String>,
    css_override: &Option<String>,
) -> Result<String, CompilerError> {
    let document = parse_yaml(input)?;
    if document.pages.len() == 1 {
        let output = generate_react(&document.pages[0], css_theme, css_override)?;
        Ok(output)
    } else {
        Err(CompilerError::from(CodegenError::UnsupportedElement))
    }
}

pub fn compile_page(input: &str) -> Result<String, CompilerError> {
    compile_page_with_css(input, &None, &None)
}

pub fn compile_page_with_css(
    input: &str,
    css_theme: &Option<String>,
    css_override: &Option<String>,
) -> Result<String, CompilerError> {
    let page: Page = serde_yaml::from_str(input)?;
    let output = generate_react(&page, css_theme, css_override)?;
    Ok(output)
}

pub fn parse_document(input: &str) -> Result<Document, serde_yaml::Error> {
    serde_yaml::from_str(input)
}

pub fn parse_page(input: &str) -> Result<Page, serde_yaml::Error> {
    serde_yaml::from_str(input)
}

pub fn parse_yaml(input: &str) -> Result<Document, CompilerError> {
    match serde_yaml::from_str::<Page>(input) {
        Ok(page) => return Ok(Document { pages: vec![page] }),
        Err(e) => eprintln!("DEBUG: Page parse error: {}", e),
    }
    serde_yaml::from_str::<Document>(input).map_err(Into::into)
}

pub fn build_project(project_dir: PathBuf) -> Result<(), CompilerError> {
    let config_path = project_dir.join("nwl.yaml");
    if !config_path.exists() {
        return Err(CompilerError::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "nwl.yaml not found in project directory",
        )));
    }

    let config_content = fs::read_to_string(&config_path)?;
    let config: ProjectConfig = serde_yaml::from_str(&config_content)?;

    // Log CSS configuration
    println!(
        "[NWL] CSS Theme: {:?}",
        config.css_theme.as_ref().unwrap_or(&"none".to_string())
    );
    println!(
        "[NWL] CSS Override: {:?}",
        config.css_override.as_ref().unwrap_or(&"none".to_string())
    );

    // Check if CSS should be imported based on project config
    let has_css_config = config.css_theme.is_some() || config.css_override.is_some();
    if has_css_config {
        println!("[NWL] CSS processing: theme and override styles will be imported");
    } else {
        println!("[NWL] CSS processing: no custom CSS configured");
    }

    let mut import_statements = String::new();
    let mut route_entries = String::new();

    for (index, route) in config.routes.iter().enumerate() {
        let page_path = project_dir.join(&route.page);
        if !page_path.exists() {
            return Err(CompilerError::IO(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Page file not found: {}", route.page),
            )));
        }

        let page_content = fs::read_to_string(&page_path)?;
        let page: Page = serde_yaml::from_str(&page_content)?;
        let component_name = page.page_data.name.clone();

        // Generate component with project-level CSS settings
        let component_code = generate_react(&page, &config.css_theme, &config.css_override)?;
        println!(
            "[NWL] Generated component: {} -> src/{}.tsx",
            component_name,
            component_name.to_lowercase()
        );

        let component_file = format!("{}.tsx", component_name.to_lowercase());
        let component_path = project_dir.join("src").join(&component_file);

        fs::write(&component_path, &component_code)?;

        import_statements.push_str(&format!(
            "import {} from './{}';\n",
            component_name,
            component_file.replace(".tsx", "")
        ));
        route_entries.push_str(&format!(
            "        <Route path=\"{}\" element={{<{} />}} />\n",
            route.path, component_name
        ));

        if index < config.routes.len() - 1 {
            route_entries.push_str("\n");
        }
    }

    // Create a full document for CSS processing (collect all pages)
    let mut all_pages = Vec::new();
    for route in &config.routes {
        let page_path = project_dir.join(&route.page);
        if page_path.exists() {
            let page_content = fs::read_to_string(&page_path)?;
            let page: Page = serde_yaml::from_str(&page_content)?;
            all_pages.push(page);
        }
    }
    let document = Document { pages: all_pages };

    // Generate processed CSS file if configured
    if has_css_config {
        let inline_styles = crate::codegen::css_processing::collect_inline_styles(&document);
        let processed = crate::codegen::css_processing::process_css(
            &project_dir,
            &config.css_theme,
            &config.css_override,
            &inline_styles,
        )
        .unwrap_or_else(|_| crate::codegen::css_processing::ProcessedCss {
            rules: Vec::new(),
            import_statements: Vec::new(),
        });
        let css_output = crate::codegen::css_processing::generate_css_output(&processed.rules);

        // Write processed CSS file
        let themes_dir = project_dir.join("themes");
        if !themes_dir.exists() {
            fs::create_dir_all(&themes_dir)?;
        }
        let processed_css_path = themes_dir.join("processed.css");
        fs::write(&processed_css_path, &css_output)?;
        println!("[NWL] Generated CSS: themes/processed.css");
    }

    // Generate router with CSS processing
    let router_code = generate_router(
        &config.name,
        &import_statements,
        &route_entries,
        &config.css_theme,
        &config.css_override,
        &document,
    );
    println!("[NWL] Generated router: src/main.tsx");

    let main_tsx_path = project_dir.join("src").join("main.tsx");
    fs::write(&main_tsx_path, &router_code)?;

    // Log summary
    println!(
        "[NWL] Build complete: {} routes, {} pages",
        config.routes.len(),
        config.routes.len()
    );
    if has_css_config {
        println!("[NWL] CSS files to be bundled: themes/theme.css, themes/theme.override.css");
    }

    Ok(())
}
