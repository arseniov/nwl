pub mod codegen;

use crate::codegen::{generate_react, generate_router, CodegenError};
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
    compile(&content)
}

pub fn compile(input: &str) -> Result<String, CompilerError> {
    let document = parse_yaml(input)?;
    let output = generate_react(&document)?;
    Ok(output)
}

pub fn compile_page(input: &str) -> Result<String, CompilerError> {
    let page: Page = serde_yaml::from_str(input)?;
    let document = Document { pages: vec![page] };
    let output = generate_react(&document)?;
    Ok(output)
}

pub fn parse_document(input: &str) -> Result<Document, serde_yaml::Error> {
    serde_yaml::from_str(input)
}

pub fn parse_page(input: &str) -> Result<Page, serde_yaml::Error> {
    serde_yaml::from_str(input)
}

pub fn parse_yaml(input: &str) -> Result<Document, CompilerError> {
    if let Ok(page) = serde_yaml::from_str::<Page>(input) {
        return Ok(Document { pages: vec![page] });
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
        let document = Document { pages: vec![page] };
        let component_code = generate_react(&document)?;

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

    let router_code = generate_router(&config.name, &import_statements, &route_entries);
    let main_tsx_path = project_dir.join("src").join("main.tsx");
    fs::write(&main_tsx_path, &router_code)?;

    Ok(())
}
