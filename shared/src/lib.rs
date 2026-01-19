use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutType {
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "grid")]
    Grid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layout {
    #[serde(rename = "type")]
    pub layout_type: LayoutType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "element")]
pub enum Element {
    #[serde(rename = "heading")]
    Heading(HeadingElement),
    #[serde(rename = "text")]
    Text(TextElement),
    #[serde(rename = "button")]
    Button(ButtonElement),
    #[serde(rename = "card")]
    Card(CardElement),
    #[serde(rename = "list")]
    List(ListElement),
    #[serde(rename = "layout")]
    Layout(LayoutElement),
    #[serde(rename = "input")]
    Input(InputElement),
    #[serde(rename = "image")]
    Image(ImageElement),
    #[serde(rename = "spacer")]
    Spacer(SpacerElement),
    #[serde(rename = "container")]
    Container(ContainerElement),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onClick: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutElement {
    pub layout: Layout,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpacerElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "page")]
    pub page_data: PageData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageData {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateFile {
    #[serde(rename = "state")]
    pub states: Vec<StateDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDefinition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial: Option<serde_yaml::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionFile {
    #[serde(rename = "action")]
    pub actions: Vec<ActionDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionDefinition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub handler: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    pub pages: Vec<Page>,
}

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("YAML parsing error: {0}")]
    Parse(String),
    #[error("Semantic error: {0}")]
    Semantic(String),
    #[error("Code generation error: {0}")]
    Codegen(String),
    #[error("IO error: {0}")]
    IO(String),
}

impl fmt::Display for LayoutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutType::Column => write!(f, "column"),
            LayoutType::Row => write!(f, "row"),
            LayoutType::Stack => write!(f, "stack"),
            LayoutType::Grid => write!(f, "grid"),
        }
    }
}
