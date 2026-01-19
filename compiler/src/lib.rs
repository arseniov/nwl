pub mod codegen;

use crate::codegen::{generate_react, CodegenError};
use nwl_shared::{Document, Page};
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
