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

        let has_state = document.pages.iter().any(|p| !p.page_data.state.is_empty());

        if has_state {
            output.push_str("import React, { useState } from 'react';\n\n");
        } else {
            output.push_str("import React from 'react';\n\n");
        }

        for page in &document.pages {
            output.push_str(&Self::generate_page(&page.page_data)?);
            output.push('\n');
        }

        Ok(output)
    }

    fn generate_page(page: &PageData) -> Result<String, CodegenError> {
        let mut output = String::new();

        let component_name = Self::to_pascal_case(&page.name);

        output.push_str(&format!(
            "export default function {}() {{\n",
            component_name
        ));

        if !page.state.is_empty() {
            output.push_str("  const ");
            let mut first = true;
            for state in &page.state {
                if !first {
                    output.push_str(", ");
                }
                let camel_name = Self::to_camel_case(&state.name);
                let setter_name = format!("set{}", Self::to_pascal_case(&state.name));
                let initial = match &state.initial {
                    Some(serde_yaml::Value::Number(n)) => n.to_string(),
                    Some(serde_yaml::Value::String(s)) => format!("\"{}\"", s),
                    Some(serde_yaml::Value::Bool(b)) => b.to_string(),
                    Some(serde_yaml::Value::Null) => "null".to_string(),
                    _ => "null".to_string(),
                };
                output.push_str(&format!(
                    "[{}, {}] = useState({})",
                    camel_name, setter_name, initial
                ));
                first = false;
            }
            output.push_str(";\n");
        }

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
            nwl_shared::Element::Checkbox(checkbox) => Self::generate_checkbox(checkbox, indent),
            nwl_shared::Element::Slider(slider) => Self::generate_slider(slider, indent),
            nwl_shared::Element::Select(select) => Self::generate_select(select, indent),
            nwl_shared::Element::RadioGroup(radio) => Self::generate_radio_group(radio, indent),
            nwl_shared::Element::Textarea(textarea) => Self::generate_textarea(textarea, indent),
            nwl_shared::Element::Form(form) => Self::generate_form(form, indent),
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

        if let Some(bind) = &input.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        } else if let Some(value) = &input.value {
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
        Ok(format!("{}<div className=\"h-6\" />", indent_str))
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

    fn generate_checkbox(
        checkbox: &nwl_shared::CheckboxElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&checkbox.style);

        let mut props = Vec::new();
        props.push("type=\"checkbox\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }

        if let Some(bind) = &checkbox.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("checked={{{}}}", camel_name));
            props.push(format!(
                "onChange={{() => {}(!{})}}",
                setter_name, camel_name
            ));
        }

        if let Some(onChange) = &checkbox.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        if let Some(label) = &checkbox.label {
            Ok(format!(
                "{}<label className=\"flex items-center gap-2\"><input{} />{}</label>",
                indent_str, props_str, label
            ))
        } else {
            Ok(format!("{}<input{} />", indent_str, props_str))
        }
    }

    fn generate_slider(
        slider: &nwl_shared::SliderElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&slider.style);

        let mut props = Vec::new();
        props.push("type=\"range\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(min) = slider.min {
            props.push(format!("min=\"{}\"", min));
        }
        if let Some(max) = slider.max {
            props.push(format!("max=\"{}\"", max));
        }
        if let Some(step) = slider.step {
            props.push(format!("step=\"{}\"", step));
        }

        if let Some(bind) = &slider.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(Number(e.target.value))}}",
                setter_name
            ));
        }

        if let Some(onChange) = &slider.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        if let Some(label) = &slider.label {
            Ok(format!(
                "{}<label className=\"flex flex-col gap-1\"><span>{}</span><input{} /></label>",
                indent_str, label, props_str
            ))
        } else {
            Ok(format!("{}<input{} />", indent_str, props_str))
        }
    }

    fn generate_select(
        select: &nwl_shared::SelectElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&select.style);

        let mut props = Vec::new();
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }

        if let Some(bind) = &select.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        if let Some(onChange) = &select.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        let mut options = String::new();
        if let Some(placeholder) = &select.placeholder {
            options.push_str(&format!(
                "{}<option value=\"\" disabled>{}</option>\n",
                indent_str, placeholder
            ));
        }
        for opt in &select.options {
            let label = opt.label.as_ref().unwrap_or(&opt.value);
            options.push_str(&format!(
                "{}<option value=\"{}\">{}</option>\n",
                indent_str, opt.value, label
            ));
        }

        Ok(format!(
            "{}<select{}>\n{}{}</select>",
            indent_str, props_str, options, indent_str
        ))
    }

    fn generate_radio_group(
        radio: &nwl_shared::RadioGroupElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&radio.style);

        let mut output = String::new();
        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        output.push_str(&format!("{}<div{}>\n", indent_str, wrapper_class));

        if let Some(label) = &radio.label {
            output.push_str(&format!(
                "{}  <p className=\"font-semibold mb-2\">{}</p>\n",
                indent_str, label
            ));
        }

        for opt in &radio.options {
            let label = opt.label.as_ref().unwrap_or(&opt.value);
            let option_class = format!("flex items-center gap-2");

            let mut input_props = Vec::new();
            input_props.push("type=\"radio\"".to_string());
            input_props.push(format!("value=\"{}\"", opt.value));

            if let Some(bind) = &radio.bind {
                let camel_name = Self::to_camel_case(bind);
                input_props.push(format!("checked={{{} === \"{}\"}}", camel_name, opt.value));
                let setter_name = format!("set{}", Self::to_pascal_case(bind));
                input_props.push(format!(
                    "onChange={{() => {}(\"{}\")}}",
                    setter_name, opt.value
                ));
            }

            if let Some(onChange) = &radio.onChange {
                input_props.push(format!("onChange={{(e) => {}}}", onChange));
            }

            let input_str = format!(
                "<label className=\"{}\"><input {} />{}</label>",
                option_class,
                input_props.join(" "),
                label
            );
            output.push_str(&format!("{}  {}\n", indent_str, input_str));
        }

        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn generate_textarea(
        textarea: &nwl_shared::TextareaElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&textarea.style);

        let mut props = Vec::new();
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(placeholder) = &textarea.placeholder {
            props.push(format!("placeholder=\"{}\"", placeholder));
        }
        if let Some(rows) = textarea.rows {
            props.push(format!("rows=\"{}\"", rows));
        }

        if let Some(bind) = &textarea.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        if let Some(onChange) = &textarea.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<textarea{} />", indent_str, props_str))
    }

    fn generate_form(
        form: &nwl_shared::FormElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&form.style);

        let mut output = String::new();
        let props = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        let on_submit = if let Some(handler) = &form.onSubmit {
            format!("onSubmit={{(e) => {{ e.preventDefault(); {} }}}}", handler)
        } else {
            "onSubmit={(e) => e.preventDefault()}".to_string()
        };

        output.push_str(&format!("{}<form{} {}>\n", indent_str, props, on_submit));

        for child in &form.children {
            output.push_str(&format!(
                "{}  {}\n",
                indent_str,
                Self::generate_element(child, indent + 1)?
            ));
        }

        output.push_str(&format!("{}</form>\n", indent_str));

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

    fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = false;

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

pub fn generate_router(name: &str, imports: &str, routes: &str) -> String {
    format!(
        r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import {{ BrowserRouter, Routes, Route }} from 'react-router-dom'
import './index.css'

{}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
{}
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
)
"#,
        imports, routes
    )
}

impl From<CodegenError> for nwl_shared::CompileError {
    fn from(e: CodegenError) -> Self {
        nwl_shared::CompileError::Codegen(e.to_string())
    }
}
