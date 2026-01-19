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
                    Some(serde_yaml::Value::Sequence(arr)) => {
                        let items: Vec<String> = arr
                            .iter()
                            .map(|v| match v {
                                serde_yaml::Value::String(s) => format!("\"{}\"", s),
                                serde_yaml::Value::Number(n) => n.to_string(),
                                serde_yaml::Value::Bool(b) => b.to_string(),
                                _ => "null".to_string(),
                            })
                            .collect();
                        format!("[{}]", items.join(", "))
                    }
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
            nwl_shared::Element::DateInput(date) => Self::generate_date_input(date, indent),
            nwl_shared::Element::TimeInput(time) => Self::generate_time_input(time, indent),
            nwl_shared::Element::DateTimeInput(datetime) => {
                Self::generate_datetime_input(datetime, indent)
            }
            nwl_shared::Element::ColorPicker(color) => Self::generate_color_picker(color, indent),
            nwl_shared::Element::FileUpload(file) => Self::generate_file_upload(file, indent),
            nwl_shared::Element::Progress(progress) => Self::generate_progress(progress, indent),
            nwl_shared::Element::Toggle(toggle) => Self::generate_toggle(toggle, indent),
            nwl_shared::Element::Tabs(tabs) => Self::generate_tabs(tabs, indent),
            nwl_shared::Element::Accordion(accordion) => {
                Self::generate_accordion(accordion, indent)
            }
            nwl_shared::Element::Modal(modal) => Self::generate_modal(modal, indent),
            nwl_shared::Element::Rating(rating) => Self::generate_rating(rating, indent),
            nwl_shared::Element::Badge(badge) => Self::generate_badge(badge, indent),
            nwl_shared::Element::Tag(tag) => Self::generate_tag(tag, indent),
            nwl_shared::Element::Alert(alert) => Self::generate_alert(alert, indent),
            nwl_shared::Element::Spinner(spinner) => Self::generate_spinner(spinner, indent),
            nwl_shared::Element::Counter(counter) => Self::generate_counter(counter, indent),
            nwl_shared::Element::SearchInput(search) => Self::generate_search_input(search, indent),
            nwl_shared::Element::CopyButton(copy) => Self::generate_copy_button(copy, indent),
            nwl_shared::Element::Pagination(pagination) => {
                Self::generate_pagination(pagination, indent)
            }
            nwl_shared::Element::Breadcrumb(breadcrumb) => {
                Self::generate_breadcrumb(breadcrumb, indent)
            }
            nwl_shared::Element::Avatar(avatar) => Self::generate_avatar(avatar, indent),
            nwl_shared::Element::ChipInput(chip) => Self::generate_chip_input(chip, indent),
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
            props.push(format!("rows={{{}}}", rows));
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

    fn generate_date_input(
        date: &nwl_shared::DateInputElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&date.style);

        let mut props = Vec::new();
        props.push("type=\"date\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(min) = &date.min {
            props.push(format!("min=\"{}\"", min));
        }
        if let Some(max) = &date.max {
            props.push(format!("max=\"{}\"", max));
        }
        if let Some(placeholder) = &date.placeholder {
            props.push(format!("placeholder=\"{}\"", placeholder));
        }

        if let Some(bind) = &date.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        if let Some(onChange) = &date.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<input{} />", indent_str, props_str))
    }

    fn generate_time_input(
        time: &nwl_shared::TimeInputElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&time.style);

        let mut props = Vec::new();
        props.push("type=\"time\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(min) = &time.min {
            props.push(format!("min=\"{}\"", min));
        }
        if let Some(max) = &time.max {
            props.push(format!("max=\"{}\"", max));
        }
        if let Some(step) = time.step {
            props.push(format!("step=\"{}\"", step));
        }

        if let Some(bind) = &time.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<input{} />", indent_str, props_str))
    }

    fn generate_datetime_input(
        datetime: &nwl_shared::DateTimeInputElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&datetime.style);

        let mut props = Vec::new();
        props.push("type=\"datetime-local\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(min) = &datetime.min {
            props.push(format!("min=\"{}\"", min));
        }
        if let Some(max) = &datetime.max {
            props.push(format!("max=\"{}\"", max));
        }

        if let Some(bind) = &datetime.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<input{} />", indent_str, props_str))
    }

    fn generate_color_picker(
        color: &nwl_shared::ColorPickerElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&color.style);

        let mut props = Vec::new();
        props.push("type=\"color\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }

        if let Some(bind) = &color.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        if let Some(onChange) = &color.onChange {
            props.push(format!("onChange={{(e) => {}}}", onChange));
        }

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        Ok(format!("{}<input{} />", indent_str, props_str))
    }

    fn generate_file_upload(
        file: &nwl_shared::FileUploadElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&file.style);

        let mut props = Vec::new();
        props.push("type=\"file\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(accept) = &file.accept {
            props.push(format!("accept=\"{}\"", accept));
        }
        if let Some(maxSize) = &file.maxSize {
            props.push(format!("data-max-size=\"{}\"", maxSize));
        }
        if file.multiple == Some(true) {
            props.push("multiple".to_string());
        }

        let mut output = String::new();
        output.push_str(&format!("{}<input", indent_str));
        for prop in &props {
            output.push_str(&format!(" {}", prop));
        }
        output.push_str(" />");
        Ok(output)
    }

    fn generate_progress(
        progress: &nwl_shared::ProgressElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&progress.style);

        let mut props = Vec::new();
        props.push("role=\"progressbar\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(max) = progress.max {
            props.push(format!("aria-valuemax=\"{}\"", max));
        }

        let (value_expr, display_value) = if let Some(bind) = &progress.bind {
            let camel_name = Self::to_camel_case(bind);
            (camel_name.clone(), camel_name)
        } else if let Some(value) = &progress.value {
            (value.clone(), value.clone())
        } else {
            ("0".to_string(), "0".to_string())
        };

        props.push(format!("aria-valuenow={{{}}}", value_expr));
        props.push(format!("style={{{{width: `${{{}}}%`}}}}", display_value));

        let props_str = if props.is_empty() {
            String::new()
        } else {
            format!(" {}", props.join(" "))
        };

        let label = if progress.showLabel == Some(true) {
            format!("<span className=\"text-sm ml-2\">{display_value}%</span>")
        } else {
            String::new()
        };

        Ok(format!(
            "{}{}{}",
            indent_str,
            format!("<div{} />", props_str),
            label
        ))
    }

    fn generate_toggle(
        toggle: &nwl_shared::ToggleElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&toggle.style);

        let on_color_default = "green".to_string();
        let on_color = toggle.onColor.as_ref().unwrap_or(&on_color_default);
        let off_color_default = "gray".to_string();
        let off_color = toggle.offColor.as_ref().unwrap_or(&off_color_default);

        let mut props = Vec::new();
        props.push("type=\"checkbox\"".to_string());
        props.push(format!("className=\"sr-only peer\""));

        if let Some(bind) = &toggle.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("checked={{{}}}", camel_name));
            props.push(format!(
                "onChange={{() => {}(!{})}}",
                setter_name, camel_name
            ));
        }

        let props_str = props.join(" ");

        let label_html = if let Some(label) = &toggle.label {
            format!(
                "<span className=\"ml-3 text-sm font-medium text-gray-900\">{}</span>",
                label
            )
        } else {
            String::new()
        };

        Ok(format!(
            "{}<label className=\"inline-flex relative items-center cursor-pointer\">{}<input {} /><div className=\"w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-{}-100\"></div>{}</label>",
            indent_str, label_html, props_str, on_color, label_html
        ))
    }

    fn generate_tabs(
        tabs: &nwl_shared::TabsElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&tabs.style);

        let mut output = String::new();
        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        output.push_str(&format!("{}<div{}>\n", indent_str, wrapper_class));

        if let Some(label) = &tabs.label {
            output.push_str(&format!(
                "{}  <p className=\"font-semibold mb-2\">{}</p>\n",
                indent_str, label
            ));
        }

        output.push_str(&format!(
            "{}  <div className=\"border-b border-gray-200\">\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}    <nav className=\"-mb-px flex space-x-8\">\n",
            indent_str
        ));

        for (index, tab) in tabs.options.iter().enumerate() {
            let tab_label = tab.label.as_ref().unwrap_or(&tab.value);
            let active_class = if index == 0 {
                "border-blue-500 text-blue-600"
            } else {
                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
            };

            let mut input_props = Vec::new();
            input_props.push("type=\"radio\"".to_string());
            input_props.push("name=\"tabs\"".to_string());
            input_props.push(format!("value=\"{}\"", tab.value));
            input_props.push(format!("className=\"sr-only peer\""));

            if let Some(bind) = &tabs.bind {
                let camel_name = Self::to_camel_case(bind);
                input_props.push(format!("checked={{{} === \"{}\"}}", camel_name, tab.value));
                let setter_name = format!("set{}", Self::to_pascal_case(bind));
                input_props.push(format!(
                    "onChange={{() => {}(\"{}\")}}",
                    setter_name, tab.value
                ));
            }

            output.push_str(&format!(
                "{}      <label className=\"{} cursor-pointer whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm\">{}<input {} /></label>\n",
                indent_str,
                active_class,
                tab_label,
                input_props.join(" ")
            ));
        }

        output.push_str(&format!("{}    </nav>\n", indent_str));
        output.push_str(&format!("{}  </div>\n", indent_str));
        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn generate_accordion(
        accordion: &nwl_shared::AccordionElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&accordion.style);

        let mut output = String::new();
        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        output.push_str(&format!("{}<div{}>\n", indent_str, wrapper_class));

        for item in &accordion.items {
            let item_title = &item.title;
            let item_content = &item.content;
            output.push_str(&format!(
                "{}  <details className=\"mb-2 border rounded-lg\">\n{}    <summary className=\"flex items-center justify-between p-4 font-medium cursor-pointer list-none\">{}      <svg className=\"w-5 h-5 ml-2 transition-transform\" fill=\"none\" stroke=\"currentColor\" viewBox=\"0 0 24 24\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M19 9l-7 7-7-7\"></path></svg>\n    </summary>{}    <div className=\"p-4 text-gray-600\">{}</div>\n  </details>\n",
                indent_str, indent_str, item_title, indent_str, item_content
            ));
        }

        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn generate_modal(
        modal: &nwl_shared::ModalElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);

        let show_prop = if let Some(bind) = &modal.bind {
            format!("{{{} && (", Self::to_camel_case(bind))
        } else {
            String::new()
        };

        let close_handler = if let Some(onClose) = &modal.onClose {
            format!("onClick={{() => {}}}", onClose)
        } else {
            String::new()
        };

        let mut output = String::new();
        output.push_str(&format!("{}<>{}\n", indent_str, show_prop));
        output.push_str(&format!("{}  <div className=\"fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50\" {}>\n", indent_str, close_handler));
        output.push_str(&format!(
            "{}    <div className=\"bg-white rounded-lg p-6 max-w-md w-full shadow-xl\">\n",
            indent_str
        ));

        if let Some(title) = &modal.title {
            output.push_str(&format!(
                "{}      <h2 className=\"text-xl font-bold mb-4\">{}</h2>\n",
                indent_str, title
            ));
        }

        for child in &modal.children {
            output.push_str(&format!(
                "{}        {}\n",
                indent_str,
                Self::generate_element(child, indent + 4)?
            ));
        }

        output.push_str(&format!("{}    </div>\n", indent_str));
        output.push_str(&format!("{}  </div>\n", indent_str));
        output.push_str(&format!("{})}}</>", indent_str));

        Ok(output)
    }

    fn generate_rating(
        rating: &nwl_shared::RatingElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let max = rating.max.unwrap_or(5);
        let class_name = Self::format_style(&rating.style);

        let mut stars = String::new();
        for i in 1..=max {
            let filled = if rating.showValue == Some(true) {
                if let Some(bind) = &rating.bind {
                    format!(
                        "{{{{ {} >= {} ? '★' : '☆' }}}}",
                        Self::to_camel_case(bind),
                        i
                    )
                } else {
                    "★".to_string()
                }
            } else {
                "★".to_string()
            };
            stars.push_str(&format!(
                "{}<span className=\"text-yellow-400 text-xl\">{}</span>\n",
                indent_str, filled
            ));
        }

        let value_label = if rating.showValue == Some(true) {
            if let Some(bind) = &rating.bind {
                format!(
                    "<span className=\"ml-2 text-gray-600\">{{{}}}</span>",
                    Self::to_camel_case(bind)
                )
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        Ok(format!(
            "{}<div{}>{}<label className=\"flex items-center cursor-pointer\">{}<input type=\"radio\" name=\"rating\" value=\"\" className=\"sr-only peer\" />{}</label></div>",
            indent_str, wrapper_class, value_label, indent_str, stars
        ))
    }

    fn generate_badge(
        badge: &nwl_shared::BadgeElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let variant = badge
            .variant
            .as_ref()
            .map(|v| match v.as_str() {
                "success" => "bg-green-100 text-green-800",
                "warning" => "bg-yellow-100 text-yellow-800",
                "error" => "bg-red-100 text-red-800",
                "info" => "bg-blue-100 text-blue-800",
                _ => "bg-gray-100 text-gray-800",
            })
            .unwrap_or("bg-gray-100 text-gray-800");

        Ok(format!(
            "{}<span className=\"{} px-2 py-1 rounded-full text-xs font-semibold\">{}</span>",
            indent_str, variant, badge.content
        ))
    }

    fn generate_tag(tag: &nwl_shared::TagElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&tag.style);

        let close_btn = if tag.removable == Some(true) {
            let handler = tag
                .onRemove
                .as_ref()
                .map(|h| format!("onClick={{() => {}}}", h))
                .unwrap_or_default();
            format!(
                "<button {} className=\"ml-1 text-gray-400 hover:text-gray-600\">×</button>",
                handler
            )
        } else {
            String::new()
        };

        Ok(format!(
            "{}<span className=\"inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 {}\">{}{}</span>",
            indent_str, class_name, tag.content, close_btn
        ))
    }

    fn generate_alert(
        alert: &nwl_shared::AlertElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let alert_type = alert
            .alertType
            .as_ref()
            .map(|t| match t.as_str() {
                "success" => "bg-green-50 text-green-800 border-green-200",
                "error" => "bg-red-50 text-red-800 border-red-200",
                "warning" => "bg-yellow-50 text-yellow-800 border-yellow-200",
                _ => "bg-blue-50 text-blue-800 border-blue-200",
            })
            .unwrap_or("bg-blue-50 text-blue-800 border-blue-200");

        let dismiss_btn = if alert.dismissible == Some(true) {
            let handler = alert
                .onDismiss
                .as_ref()
                .map(|h| format!("onClick={{() => {}}}", h))
                .unwrap_or_default();
            format!("<button {} className=\"ml-auto -mx-1.5 -my-1.5 bg-transparent text-current rounded-lg focus:ring-2 inline-flex items-center justify-center h-8 w-8\">×</button>", handler)
        } else {
            String::new()
        };

        Ok(format!(
            "{}<div className=\"{} border p-4 rounded-lg flex items-start {}\">{}<p>{}</p></div>",
            indent_str,
            alert_type,
            if alert.dismissible == Some(true) {
                "relative"
            } else {
                ""
            },
            dismiss_btn,
            alert.content
        ))
    }

    fn generate_spinner(
        spinner: &nwl_shared::SpinnerElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let size = spinner
            .size
            .as_ref()
            .map(|s| match s.as_str() {
                "sm" => "w-4 h-4",
                "lg" => "w-8 h-8",
                _ => "w-6 h-6",
            })
            .unwrap_or("w-6 h-6");

        let label = spinner
            .label
            .as_ref()
            .map(|l| {
                format!(
                    "<span className=\"ml-2 text-sm text-gray-600\">{}</span>",
                    l
                )
            })
            .unwrap_or_default();

        Ok(format!(
            "{}<div className=\"flex items-center justify-center\">{}<div className=\"{} animate-spin rounded-full border-2 border-gray-300 border-t-blue-600\"></div>{}</div>",
            indent_str,
            label,
            size,
            if label.is_empty() { "<div className=\"sr-only\">Loading...</div>".to_string() } else { String::new() }
        ))
    }

    fn generate_counter(
        counter: &nwl_shared::CounterElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&counter.style);

        let decrement = if let Some(bind) = &counter.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            let min = counter.min.unwrap_or(0);
            format!(
                "onClick={{() => {} > {} && {}({} - 1)}}",
                camel_name, min, setter_name, camel_name
            )
        } else {
            "onClick={() => {}}".to_string()
        };

        let increment = if let Some(bind) = &counter.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            let max = counter.max.unwrap_or(999);
            format!(
                "onClick={{() => {} < {} && {}({} + 1)}}",
                camel_name, max, setter_name, camel_name
            )
        } else {
            "onClick={() => {}}".to_string()
        };

        let value = if let Some(bind) = &counter.bind {
            format!(
                "<span className=\"text-lg font-semibold mx-4\">{{{}}}</span>",
                Self::to_camel_case(bind)
            )
        } else {
            "<span className=\"text-lg font-semibold mx-4\">0</span>".to_string()
        };

        Ok(format!(
            "{}<div className=\"flex items-center border rounded-lg {}\">{}<button {} className=\"px-3 py-1 border-r hover:bg-gray-100\">-</button>{}<button {} className=\"px-3 py-1 border-l hover:bg-gray-100\">+</button></div>",
            indent_str,
            if counter.step.is_some() { "w-32" } else { "" },
            indent_str,
            decrement,
            value,
            increment
        ))
    }

    fn generate_search_input(
        search: &nwl_shared::SearchInputElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&search.style);

        let mut props = Vec::new();
        props.push("type=\"search\"".to_string());
        if !class_name.is_empty() {
            props.push(format!("className=\"{}\"", class_name));
        }
        if let Some(placeholder) = &search.placeholder {
            props.push(format!("placeholder=\"{}\"", placeholder));
        }

        if let Some(bind) = &search.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            props.push(format!("value={{{}}}", camel_name));
            props.push(format!(
                "onChange={{(e) => {}(e.target.value)}}",
                setter_name
            ));
        }

        if search.clearable == Some(true) {
            props.push("onClear".to_string());
        }

        if let Some(onSearch) = &search.onSearch {
            props.push(format!("onSearch={{() => {}}}", onSearch));
        }

        let props_str = props.join(" ");

        Ok(format!(
            "{}<div className=\"relative\">{}<input {} /><button className=\"absolute right-3 top-1/2 -translate-y-1/2 text-gray-400\">🔍</button></div>",
            indent_str,
            if !class_name.is_empty() { format!("<span className=\"{}\"></span>", class_name) } else { String::new() },
            props_str
        ))
    }

    fn generate_copy_button(
        copy: &nwl_shared::CopyButtonElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&copy.style);

        let content = copy
            .content
            .as_ref()
            .or(copy.text.as_ref())
            .map(|s| format!("\"{}\"", s))
            .unwrap_or("''".to_string());
        let handler = copy
            .onCopy
            .as_ref()
            .map(|h| {
                format!(
                    "onClick={{() => navigator.clipboard.writeText({}) && {}}}",
                    content, h
                )
            })
            .unwrap_or_else(|| {
                format!(
                    "onClick={{() => navigator.clipboard.writeText({})}}",
                    content
                )
            });

        Ok(format!(
            "{}<button {} className=\"inline-flex items-center px-3 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 {}\">Copy</button>",
            indent_str,
            handler,
            class_name
        ))
    }

    fn generate_pagination(
        pagination: &nwl_shared::PaginationElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&pagination.style);

        let total = pagination.total.unwrap_or(100);
        let per_page = pagination.perPage.unwrap_or(10);
        let total_pages = (total as f64 / per_page as f64).ceil() as u32;

        let prev_handler = if let Some(bind) = &pagination.bind {
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!(
                "onClick={{() => {} > 1 && {}({} - 1)}}",
                Self::to_camel_case(bind),
                setter_name,
                Self::to_camel_case(bind)
            )
        } else {
            "onClick={{() => {}}}".to_string()
        };

        let next_handler = if let Some(bind) = &pagination.bind {
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!(
                "onClick={{() => {} < {} && {}({} + 1)}}",
                Self::to_camel_case(bind),
                total_pages,
                setter_name,
                Self::to_camel_case(bind)
            )
        } else {
            "onClick={{() => {}}}".to_string()
        };

        let current = if let Some(bind) = &pagination.bind {
            format!("<span className=\"px-4 py-2 border bg-blue-50 text-blue-600 font-medium\">Page {{ {bind} }} of {total_pages}</span>", bind = Self::to_camel_case(bind))
        } else {
            format!("<span className=\"px-4 py-2 border bg-blue-50 text-blue-600 font-medium\">Page 1 of {total_pages}</span>")
        };

        Ok(format!(
            "{}<div className=\"flex items-center justify-center space-x-2 {}\">{}<button {} className=\"px-3 py-1 border rounded hover:bg-gray-100\">Previous</button>{}<button {} className=\"px-3 py-1 border rounded hover:bg-gray-100\">Next</button></div>",
            indent_str,
            class_name,
            indent_str,
            prev_handler,
            current,
            next_handler
        ))
    }

    fn generate_breadcrumb(
        breadcrumb: &nwl_shared::BreadcrumbElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&breadcrumb.style);

        let mut items = String::new();
        for (index, item) in breadcrumb.items.iter().enumerate() {
            let separator = if index < breadcrumb.items.len() - 1 {
                "<span className=\"mx-2 text-gray-400\">/</span>"
            } else {
                ""
            };
            let content = if let Some(href) = &item.href {
                format!(
                    "<a href=\"{}\" className=\"text-blue-600 hover:underline\">{}</a>{}",
                    href, item.label, separator
                )
            } else {
                format!(
                    "<span className=\"text-gray-600\">{}</span>{}",
                    item.label, separator
                )
            };
            items.push_str(&format!("{}<span>{}</span>\n", indent_str, content));
        }

        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        Ok(format!("{}<nav{} aria-label=\"Breadcrumb\"><ol className=\"flex items-center space-x-2\">{}</ol></nav>", indent_str, wrapper_class, items))
    }

    fn generate_avatar(
        avatar: &nwl_shared::AvatarElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let size = avatar
            .size
            .as_ref()
            .map(|s| match s.as_str() {
                "sm" => "w-8 h-8 text-xs",
                "lg" => "w-16 h-16 text-lg",
                _ => "w-10 h-10 text-sm",
            })
            .unwrap_or("w-10 h-10 text-sm");

        let image = if let Some(src) = &avatar.src {
            format!(
                "<img src=\"{}\" className=\"w-full h-full object-cover rounded-full\" />",
                src
            )
        } else {
            let fallback = avatar
                .fallback
                .as_ref()
                .map(|f| format!("{}", f.chars().take(2).collect::<String>()))
                .unwrap_or_else(|| "?".to_string());
            format!("<span className=\"flex items-center justify-center w-full h-full rounded-full bg-gray-200 text-gray-600 font-medium\">{}</span>", fallback)
        };

        let name = if let Some(name) = &avatar.name {
            format!(
                "<span className=\"ml-2 font-medium text-gray-700\">{}</span>",
                name
            )
        } else {
            String::new()
        };

        Ok(format!(
            "{}<div className=\"flex items-center {}\">{}<div className=\"{} rounded-full overflow-hidden bg-gray-100\">{}</div>{}</div>",
            indent_str,
            Self::format_style(&avatar.style),
            indent_str,
            size,
            image,
            name
        ))
    }

    fn generate_chip_input(
        chip: &nwl_shared::ChipInputElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&chip.style);

        let mut suggestions_html = String::new();
        for suggestion in &chip.suggestions {
            suggestions_html.push_str(&format!("<span className=\"inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200\">{}</span>\n", suggestion));
        }

        let input_handler = if let Some(bind) = &chip.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!("onKeyDown={{(e) => e.key === 'Enter' && {}([...{}, e.target.value]) && (e.target.value = '')}}", setter_name, camel_name)
        } else {
            "onKeyDown={{() => {{}}}}".to_string()
        };

        let chips = if let Some(bind) = &chip.bind {
            let camel_name = Self::to_camel_case(bind);
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!("<div className=\"flex flex-wrap gap-2 mb-2\">{{ {}.map((chip, i) => <span key={{i}} className=\"px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-sm\">{{chip}}<button className=\"ml-1 text-blue-600\" onClick={{() => {}({}.filter((_, j) => j !== i))}}>×</button></span>) }}</div>", camel_name, setter_name, camel_name)
        } else {
            String::new()
        };

        Ok(format!(
            "{}<div className=\"{}\">{}{}<input type=\"text\" placeholder=\"{}\" {} className=\"w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500\" /></div>",
            indent_str,
            class_name,
            chips,
            if !chip.suggestions.is_empty() { format!("<div className=\"mb-2\">{}</div>", suggestions_html) } else { String::new() },
            chip.placeholder.as_ref().unwrap_or(&"Add tags...".to_string()),
            input_handler
        ))
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
