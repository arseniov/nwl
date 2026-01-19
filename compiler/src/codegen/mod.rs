use nwl_shared::{
    ButtonElement, CardElement, ContainerElement, Document, HeadingElement, ImageElement,
    InputElement, LayoutElement, LayoutType, ListElement, PageData, SpacerElement, TextElement,
};

#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("Unsupported element type")]
    UnsupportedElement,
}

pub struct ReactGenerator;

impl ReactGenerator {
    pub fn generate(document: &Document) -> Result<String, CodegenError> {
        let mut output = String::new();

        for page in &document.pages {
            output.push_str(&Self::generate_page(&page.page_data)?);
            output.push('\n');
        }

        Ok(output)
    }

    fn generate_page(page: &PageData) -> Result<String, CodegenError> {
        let mut output = String::new();

        let component_name = "App";

        output.push_str(&format!(
            "export default function {}() {{\n",
            component_name
        ));

        output.push_str("  return (\n");
        output.push_str("    <>\n");

        if let Some(layout) = &page.layout {
            let page_style = Self::format_style(&page.style);
            let page_style_attr = if !page_style.is_empty() {
                format!(" className=\"{}\"", page_style)
            } else {
                String::new()
            };
            let layout_classes = Self::format_layout_props(layout);
            output.push_str(&format!("      <div{}>\n", page_style_attr));
            for child in &page.children {
                output.push_str(&format!("        {}\n", Self::generate_element(child, 2)?));
            }
            output.push_str("      </div>\n");
        } else {
            for child in &page.children {
                output.push_str(&format!("      {}\n", Self::generate_element(child, 1)?));
            }
        }

        output.push_str("    </>\n");
        output.push_str("  );\n");
        output.push_str("}\n");

        Ok(output)
    }

    fn generate_element(
        element: &nwl_shared::Element,
        indent: usize,
    ) -> Result<String, CodegenError> {
        match element {
            nwl_shared::Element::Heading(heading) => Self::generate_heading(heading, indent),
            nwl_shared::Element::Text(text) => Self::generate_text(text, indent),
            nwl_shared::Element::Button(button) => Self::generate_button(button, indent),
            nwl_shared::Element::Card(card) => Self::generate_card(card, indent),
            nwl_shared::Element::List(list) => Self::generate_list(list, indent),
            nwl_shared::Element::Layout(layout) => Self::generate_layout(layout, indent),
            nwl_shared::Element::Input(input) => Self::generate_input(input, indent),
            nwl_shared::Element::Image(image) => Self::generate_image(image, indent),
            nwl_shared::Element::Spacer(spacer) => Self::generate_spacer(spacer, indent),
            nwl_shared::Element::Container(container) => {
                Self::generate_container(container, indent)
            }
        }
    }

    fn generate_heading(heading: &HeadingElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&heading.style);

        if class_name.is_empty() {
            Ok(format!("{}<h1>{}</h1>", indent_str, heading.content))
        } else {
            Ok(format!(
                "{}<h1 className=\"{}\">{}</h1>",
                indent_str, class_name, heading.content
            ))
        }
    }

    fn generate_text(text: &TextElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&text.style);

        if class_name.is_empty() {
            Ok(format!("{}<p>{}</p>", indent_str, text.content))
        } else {
            Ok(format!(
                "{}<p className=\"{}\">{}</p>",
                indent_str, class_name, text.content
            ))
        }
    }

    fn generate_button(button: &ButtonElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&button.style);

        let mut props = Vec::new();
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }

        if let Some(onClick) = &button.onClick {
            if onClick.starts_with('/') {
                props.push(format!(
                    "onClick={{() => window.location.href = \"{}\"}}",
                    onClick
                ));
            } else {
                props.push(format!("onClick={{() => {}}}", onClick));
            }
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!(
            "{}<button{}>{}</button>",
            indent_str, props_str, button.content
        ))
    }

    fn generate_card(card: &CardElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&card.style);

        let mut output = String::new();
        let props = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        output.push_str(&format!("{}<div{}>\n", indent_str, props));

        for child in &card.children {
            output.push_str(&format!(
                "{}  {}\n",
                indent_str,
                Self::generate_element(child, indent + 1)?
            ));
        }

        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn generate_list(list: &ListElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);

        let mut output = String::new();

        if let Some(data_var) = &list.data {
            output.push_str(&format!("{}{{ {}.map((item) => (\n", indent_str, data_var));

            output.push_str(&format!(
                "{}  <React.Fragment key={{item.id}}>\n",
                indent_str
            ));

            for child in &list.children {
                output.push_str(&format!(
                    "{}    {}\n",
                    indent_str,
                    Self::generate_element(child, indent + 2)?
                ));
            }

            output.push_str(&format!("{}  </React.Fragment>\n", indent_str));
            output.push_str(&format!("{}))}}\n", indent_str));
        } else {
            for child in &list.children {
                output.push_str(&format!(
                    "{}{}\n",
                    indent_str,
                    Self::generate_element(child, indent)?
                ));
            }
        }

        Ok(output)
    }

    fn generate_layout(layout: &LayoutElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let mut output = String::new();

        output.push_str(&format!(
            "{}<div{}>\n",
            indent_str,
            Self::format_layout_props(&layout.layout)
        ));

        for child in &layout.children {
            output.push_str(&format!(
                "{}  {}\n",
                indent_str,
                Self::generate_element(child, indent + 1)?
            ));
        }

        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn generate_input(input: &InputElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&input.style);

        let mut props = Vec::new();
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(placeholder) = &input.placeholder {
            props.push(format!("placeholder=\"{}\"", placeholder));
        }
        if let Some(value) = &input.value {
            props.push(format!("value=\"{}\"", value));
        }
        if let Some(onChange) = &input.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<input{} />", indent_str, props_str))
    }

    fn generate_image(image: &ImageElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&image.style);

        let mut props = Vec::new();
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(src) = &image.src {
            props.push(format!("src=\"{}\"", src));
        }
        if let Some(alt) = &image.alt {
            props.push(format!("alt=\"{}\"", alt));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<img{} />", indent_str, props_str))
    }

    fn generate_spacer(_spacer: &SpacerElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        Ok(format!("{}<div style={{height: '24px'}} />", indent_str))
    }

    fn generate_container(
        container: &ContainerElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&container.style);

        let mut output = String::new();
        let props = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        output.push_str(&format!("{}<div{}>\n", indent_str, props));

        for child in &container.children {
            output.push_str(&format!(
                "{}  {}\n",
                indent_str,
                Self::generate_element(child, indent + 1)?
            ));
        }

        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn format_layout_props(layout: &nwl_shared::Layout) -> String {
        let mut classes: Vec<String> = Vec::new();

        match layout.layout_type {
            LayoutType::Column => classes.push("flex flex-col".to_string()),
            LayoutType::Row => classes.push("flex flex-row".to_string()),
            LayoutType::Stack => classes.push("relative".to_string()),
            LayoutType::Grid => {
                classes.push("grid".to_string());
                if let Some(cols) = layout.columns {
                    classes.push(format!("grid-cols-{}", cols));
                }
            }
        }

        for prop in &layout.properties {
            classes.push(prop.clone());
        }

        if classes.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", classes.join(" "))
        }
    }

    fn format_style(styles: &[String]) -> String {
        styles.join(" ")
    }

    fn to_pascal_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = true;

        for c in s.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize = true;
            } else if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

pub fn generate_react(document: &Document) -> Result<String, CodegenError> {
    ReactGenerator::generate(document)
}

impl From<CodegenError> for nwl_shared::CompileError {
    fn from(e: CodegenError) -> Self {
        nwl_shared::CompileError::Codegen(e.to_string())
    }
}
