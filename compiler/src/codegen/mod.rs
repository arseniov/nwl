use nwl_shared::{
    ButtonElement, CaptchaConfig, CardElement, ContainerElement, Document, FieldElement,
    FieldsetElement, FormElement, HeadingElement, ImageElement, InputElement, LayoutElement,
    LayoutType, ListElement, Page, PageData, PopoverElement, SpacerElement, TextElement,
    TooltipElement,
};

pub mod css_processing;

pub use css_processing::{collect_inline_styles, generate_css_output, process_css, ProcessedCss};

#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    #[error("Unsupported element type")]
    UnsupportedElement,
}

pub struct ReactGenerator;

impl ReactGenerator {
    pub fn generate(
        page: &Page,
        css_theme: &Option<String>,
        css_override: &Option<String>,
    ) -> Result<String, CodegenError> {
        let mut output = String::new();

        let has_state = !page.page_data.state.is_empty();

        if has_state {
            output.push_str("import React, { useState } from 'react';\n");
        } else {
            output.push_str("import React from 'react';\n");
        }

        output.push_str("import {\n");
        output.push_str("  Button,\n");
        output.push_str("  Checkbox,\n");
        output.push_str("  Select,\n");
        output.push_str("  Radio,\n");
        output.push_str("  RadioGroup,\n");
        output.push_str("  Switch,\n");
        output.push_str("  Separator,\n");
        output.push_str("  NumberField,\n");
        output.push_str("  Dialog,\n");
        output.push_str("  Menu,\n");
        output.push_str("  Accordion,\n");
        output.push_str("  Form,\n");
        output.push_str("  Field,\n");
        output.push_str("  Fieldset,\n");
        output.push_str("  Tooltip,\n");
        output.push_str("  Popover,\n");
        output.push_str("} from '@base-ui/react';\n\n");

        output.push_str(&Self::generate_page(&page.page_data)?);
        output.push('\n');

        Ok(output)
    }

    fn generate_page(page: &PageData) -> Result<String, CodegenError> {
        let mut output = String::new();

        let component_name = Self::to_pascal_case(&page.name);

        output.push_str(&format!(
            "export default function {}() {{\n",
            component_name
        ));

        // Check if any menu has hamburger enabled
        let needs_menu_state = page.children.iter().any(|child| {
            if let nwl_shared::Element::NavigationMenu(menu) = child {
                menu.hamburger == Some(true)
            } else {
                false
            }
        });

        let total_states = if needs_menu_state {
            page.state.len() + 1
        } else {
            page.state.len()
        };

        if !page.state.is_empty() || needs_menu_state {
            output.push_str("  const ");
            let mut first = true;
            let mut state_index = 0;

            // First, add menuOpen state if needed
            if needs_menu_state {
                output.push_str("[menuOpen, setMenuOpen] = useState(false)");
                first = false;
            }

            // Then add the rest of the states
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
                state_index += 1;
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
            nwl_shared::Element::Field(field) => Self::generate_field(field, indent),
            nwl_shared::Element::Fieldset(fieldset) => Self::generate_fieldset(fieldset, indent),
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
            nwl_shared::Element::Dialog(dialog) => Self::generate_dialog(dialog, indent),
            nwl_shared::Element::Tooltip(tooltip) => Self::generate_tooltip(tooltip, indent),
            nwl_shared::Element::Popover(popover) => Self::generate_popover(popover, indent),
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
            nwl_shared::Element::Nav(nav) => Self::generate_nav(nav, indent),
            nwl_shared::Element::NavigationMenu(menu) => {
                Self::generate_navigation_menu(menu, indent)
            }
            nwl_shared::Element::Url(url) => Self::generate_url(url, indent),
            nwl_shared::Element::Email(email) => Self::generate_email(email, indent),
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

        // Combine Base UI class with user classes
        if class_name.is_empty() {
            props.push("className=\"Button\"".to_string());
        } else {
            props.push(format!("className=\"Button {}\"", class_name));
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
            "{}<Button{}>{}</Button>",
            indent_str, props_str, button.content
        ))
    }

    fn generate_card(card: &CardElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&card.style);

        let mut output = String::new();
        let props = if class_name.is_empty() {
            " className=\"Card\"".to_string()
        } else {
            format!(" className=\"Card {}\"", class_name)
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
        let class_name = Self::format_style(&list.style);

        let mut output = String::new();
        let props = if class_name.is_empty() {
            " className=\"List\"".to_string()
        } else {
            format!(" className=\"List {}\"", class_name)
        };

        output.push_str(&format!("{}<div{}>\n", indent_str, props));

        for (index, item) in list.items.iter().enumerate() {
            let key = format!("list-item-{}", index);
            let click_handler = if let Some(onClick) = &item.onClick {
                format!(" onClick={{() => {}}}", onClick)
            } else {
                String::new()
            };

            output.push_str(&format!(
                "{}  <div key=\"{}\" className=\"List-item\"{}>\n",
                indent_str, key, click_handler
            ));
            output.push_str(&format!("{}    {}\n", indent_str, item.content));
            output.push_str(&format!("{}  </div>\n", indent_str));
        }

        output.push_str(&format!("{}</div>\n", indent_str));

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

    fn generate_spacer(spacer: &SpacerElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);

        Ok(format!("{}<Separator />", indent_str))
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

        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" {}", class_name)
        };

        let checked_attr = if let Some(bind) = &checkbox.bind {
            format!(" checked={{{}}}", Self::to_camel_case(bind))
        } else {
            String::new()
        };

        let onchange_attr = if let Some(bind) = &checkbox.bind {
            format!(
                " onCheckedChange={{({}) => set{}({})}}",
                "checked",
                Self::to_pascal_case(bind),
                "checked"
            )
        } else {
            String::new()
        };

        let label_html = if let Some(label) = &checkbox.label {
            format!("<span>{}</span>", label)
        } else {
            String::new()
        };

        let wrapper_class = if class_name.is_empty() {
            "".to_string()
        } else {
            format!(" {}", class_name)
        };

        Ok(format!(
            "{}<label className=\"flex items-center gap-2 cursor-pointer{}\">{}<Checkbox.Root className=\"Checkbox-root\"{} {}>{}  <Checkbox.Indicator className=\"Checkbox-indicator\" />{}</Checkbox.Root>{}</label>",
            indent_str, wrapper_class, indent_str, checked_attr, onchange_attr, indent_str, indent_str, label_html
        ))
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

        let value_attr = if let Some(bind) = &select.bind {
            let camel_name = Self::to_camel_case(bind);
            format!(" value={{{}}}", camel_name)
        } else {
            String::new()
        };

        let onvaluechange_attr = if let Some(bind) = &select.bind {
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!(" onValueChange={{(value) => {}(value)}}", setter_name)
        } else {
            String::new()
        };

        let props_str = if class_name.is_empty() {
            " className=\"Select-trigger\"".to_string()
        } else {
            format!(" className=\"Select-trigger {}\"", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!(
            "{}<Select.Root{}{}>\n",
            indent_str, value_attr, onvaluechange_attr
        ));
        output.push_str(&format!("{}  <Select.Trigger{}>\n", indent_str, props_str));
        output.push_str(&format!("{}    <Select.Value />\n", indent_str));
        output.push_str(&format!("{}    <Select.Icon />\n", indent_str));
        output.push_str(&format!("{}  </Select.Trigger>\n", indent_str));
        output.push_str(&format!("{}  <Select.Portal>\n", indent_str));
        output.push_str(&format!(
            "{}    <Select.Positioner sideOffset={{8}}>\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}      <Select.Popup className=\"Select-popup\">\n",
            indent_str
        ));

        for opt in &select.options {
            let label = opt.label.as_ref().unwrap_or(&opt.value);
            output.push_str(&format!(
                "{}        <Select.Item className=\"Select-item\" value=\"{}\">\n{}          <Select.ItemText>{}</Select.ItemText>\n{}        </Select.Item>\n",
                indent_str, opt.value, indent_str, label, indent_str
            ));
        }

        output.push_str(&format!("{}      </Select.Popup>\n", indent_str));
        output.push_str(&format!("{}    </Select.Positioner>\n", indent_str));
        output.push_str(&format!("{}  </Select.Portal>\n", indent_str));
        output.push_str(&format!("{}</Select.Root>", indent_str));

        Ok(output)
    }

    fn generate_radio_group(
        radio: &nwl_shared::RadioGroupElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&radio.style);

        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" className=\"{}\"", class_name)
        };

        let default_value = if radio.bind.is_some() {
            if let Some(initial) = radio.options.first() {
                format!(" defaultValue=\"{}\"", initial.value)
            } else {
                String::new()
            }
        } else {
            if let Some(initial) = radio.options.first() {
                format!(" defaultValue=\"{}\"", initial.value)
            } else {
                String::new()
            }
        };

        let mut output = String::new();
        let wrapper_class = if class_name.is_empty() {
            " className=\"RadioGroup\"".to_string()
        } else {
            format!(" className=\"RadioGroup {}\"", class_name)
        };

        output.push_str(&format!(
            "{}<RadioGroup{}{}>\n",
            indent_str, wrapper_class, default_value
        ));

        if let Some(label) = &radio.label {
            output.push_str(&format!("{}  <p>{}</p>\n", indent_str, label));
        }

        for opt in &radio.options {
            let label = opt.label.as_ref().unwrap_or(&opt.value);
            output.push_str(&format!(
                "{}  <label className=\"flex items-center gap-2 cursor-pointer py-1\">\n{}    <Radio.Root className=\"Radio-root\" value=\"{}\">\n{}      <Radio.Indicator className=\"Radio-indicator\" />\n{}    </Radio.Root>\n{}    <span>{}</span>\n{}  </label>\n",
                indent_str, indent_str, opt.value, indent_str, indent_str, indent_str, label, indent_str
            ));
        }

        output.push_str(&format!("{}</RadioGroup>\n", indent_str));

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

    fn generate_form(form: &FormElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&form.style);

        let wrapper_class = if class_name.is_empty() {
            " className=\"Form\"".to_string()
        } else {
            format!(" className=\"Form {}\"", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!("{}<Form{}>\n", indent_str, wrapper_class));

        for child in &form.children {
            output.push_str(&format!(
                "{}  {}\n",
                indent_str,
                Self::generate_element(child, indent + 1)?
            ));
        }

        output.push_str(&format!("{}</Form>\n", indent_str));

        Ok(output)
    }

    fn generate_field(field: &FieldElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&field.style);

        let wrapper_class = if class_name.is_empty() {
            " className=\"Field-root\"".to_string()
        } else {
            format!(" className=\"Field-root {}\"", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!(
            "{}<Field.Root name=\"{}\"{}>\n",
            indent_str, field.name, wrapper_class
        ));

        if let Some(label) = &field.label {
            output.push_str(&format!(
                "{}  <Field.Label className=\"Field-label\">{}</Field.Label>\n",
                indent_str, label
            ));
        }

        output.push_str(&format!("{}  <Field.Control", indent_str));

        if let Some(placeholder) = &field.placeholder {
            output.push_str(&format!(" placeholder=\"{}\"", placeholder));
        }

        output.push_str(&format!(" className=\"Field-control\" />\n"));

        output.push_str(&format!(
            "{}  <Field.Error className=\"Field-error\" />\n",
            indent_str
        ));

        output.push_str(&format!("{}</Field.Root>\n", indent_str));

        Ok(output)
    }

    fn generate_fieldset(
        fieldset: &FieldsetElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&fieldset.style);

        let wrapper_class = if class_name.is_empty() {
            " className=\"Fieldset-root\"".to_string()
        } else {
            format!(" className=\"Fieldset-root {}\"", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!("{}<Fieldset.Root{}>\n", indent_str, wrapper_class));

        if let Some(legend) = &fieldset.legend {
            output.push_str(&format!(
                "{}  <Fieldset.Legend className=\"Fieldset-legend\">{}</Fieldset.Legend>\n",
                indent_str, legend
            ));
        }

        for child in &fieldset.children {
            output.push_str(&format!(
                "{}  {}\n",
                indent_str,
                Self::generate_element(child, indent + 1)?
            ));
        }

        output.push_str(&format!("{}</Fieldset.Root>\n", indent_str));

        Ok(output)
    }

    fn generate_captcha(captcha: &CaptchaConfig, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        match captcha.provider {
            nwl_shared::CaptchaProvider::Cloudflare => {
                let site_key = captcha.siteKey.as_deref().unwrap_or("");
                let theme = captcha.theme.as_deref().unwrap_or("auto");
                format!(
                    "{}<div className=\"cf-turnstile\" data-sitekey=\"{}\" data-theme=\"{}\"></div>",
                    indent_str,
                    site_key,
                    theme
                )
            }
            nwl_shared::CaptchaProvider::Recaptcha => {
                let site_key = captcha.siteKey.as_deref().unwrap_or("");
                let version = captcha.version.as_deref().unwrap_or("v2");
                if version == "v3" {
                    let action = captcha.action.as_deref().unwrap_or("submit");
                    format!(
                        "{}<div id=\"recaptcha-container\" data-sitekey=\"{}\" data-action=\"{}\"></div>",
                        indent_str,
                        site_key,
                        action
                    )
                } else {
                    format!(
                        "{}<div className=\"g-recaptcha\" data-sitekey=\"{}\"></div>",
                        indent_str, site_key
                    )
                }
            }
            nwl_shared::CaptchaProvider::HCaptcha => {
                let site_key = captcha.siteKey.as_deref().unwrap_or("");
                let theme = captcha.theme.as_deref().unwrap_or("light");
                format!(
                    "{}<div className=\"h-captcha\" data-sitekey=\"{}\" data-theme=\"{}\"></div>",
                    indent_str, site_key, theme
                )
            }
        }
    }

    fn generate_form_submit_handler(form: &FormElement) -> String {
        let mut validation_checks = Vec::new();

        if let Some(validation) = &form.validation {
            for (field, rules) in validation {
                let camel_field = Self::to_camel_case(field);
                for rule in rules {
                    if let Some(true) = rule.required {
                        let msg = rule
                            .message
                            .clone()
                            .unwrap_or_else(|| format!("{} is required", field));
                        validation_checks.push(format!(
                            "if (!{}.trim()) {{ console.error('{}'); _hasError = true; }}",
                            camel_field, msg
                        ));
                    }

                    if let Some(pattern) = &rule.pattern {
                        let msg = rule
                            .message
                            .clone()
                            .unwrap_or_else(|| format!("Invalid format for {}", field));
                        validation_checks.push(format!(
                            "if (!new RegExp('{}').test({})) {{ console.error('{}'); _hasError = true; }}",
                            pattern.replace("\\", "\\\\"),
                            camel_field,
                            msg
                        ));
                    }

                    if let Some(min_len) = rule.minLength {
                        let msg = rule.message.clone().unwrap_or_else(|| {
                            format!("{} must be at least {} characters", field, min_len)
                        });
                        validation_checks.push(format!(
                            "if ({}.length < {}) {{ console.error('{}'); _hasError = true; }}",
                            camel_field, min_len, msg
                        ));
                    }

                    if let Some(max_len) = rule.maxLength {
                        let msg = rule.message.clone().unwrap_or_else(|| {
                            format!("{} must be no more than {} characters", field, max_len)
                        });
                        validation_checks.push(format!(
                            "if ({}.length > {}) {{ console.error('{}'); _hasError = true; }}",
                            camel_field, max_len, msg
                        ));
                    }
                }
            }
        }

        let mut all_checks = validation_checks;

        if form.captcha.is_some() {
            all_checks.push(
                "if (!window.captchaToken) { console.error('Please complete the captcha'); _hasError = true; }"
                    .to_string(),
            );
        }

        let validation_code = if all_checks.is_empty() {
            String::new()
        } else {
            let error_handling = form
                .onValidationError
                .as_ref()
                .map(|h| format!("{};", h))
                .unwrap_or_default();
            format!(
                "let _hasError = false; {} if (_hasError) {{ console.error('Validation failed'); {} return; }}",
                all_checks.join(" "),
                error_handling
            )
        };

        if let Some(handler) = &form.onSubmit {
            format!(
                "onSubmit={{(e) => {{ e.preventDefault(); {} {} }}}}",
                validation_code, handler
            )
        } else {
            format!(
                "onSubmit={{(e) => {{ e.preventDefault(); {} }}}}",
                validation_code
            )
        }
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

        let wrapper_class = if class_name.is_empty() {
            " className=\"Switch-root\"".to_string()
        } else {
            format!(" className=\"Switch-root {}\"", class_name)
        };

        let checked_attr = if let Some(bind) = &toggle.bind {
            format!(" checked={{{}}}", Self::to_camel_case(bind))
        } else {
            String::new()
        };

        let onchange_attr = if let Some(bind) = &toggle.bind {
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!("onCheckedChange={{(checked) => {}(checked)}}", setter_name)
        } else {
            String::new()
        };

        let label_html = if let Some(label) = &toggle.label {
            format!("<span className=\"font-medium\">{}</span>", label)
        } else {
            String::new()
        };

        Ok(format!(
            "{}<label className=\"inline-flex items-center cursor-pointer gap-3\">{}<Switch.Root className=\"Switch-root\" id=\"toggle\"{} {}>{}  <Switch.Thumb className=\"Switch-thumb\" />{}</Switch.Root></label>",
            indent_str, label_html, checked_attr, onchange_attr, indent_str, indent_str
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
            output.push_str(&format!("{}  <p>{}</p>\n", indent_str, label));
        }

        output.push_str(&format!("{}  <div>\n", indent_str));
        output.push_str(&format!("{}    <nav>\n", indent_str));

        for (index, tab) in tabs.options.iter().enumerate() {
            let tab_label = tab.label.as_ref().unwrap_or(&tab.value);

            let mut input_props = Vec::new();
            input_props.push("type=\"radio\"".to_string());
            input_props.push("name=\"tabs\"".to_string());
            input_props.push(format!("value=\"{}\"", tab.value));
            input_props.push(format!("className=\"sr-only\""));

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
                "{}      <label className=\"cursor-pointer whitespace-nowrap py-4 px-1 border-b-2\">{}<input {} /></label>\n",
                indent_str,
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

        let multiple_attr = if accordion.multiple == Some(true) {
            " multiple"
        } else {
            ""
        };

        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" {}", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!(
            "{}<Accordion.Root{}>\n",
            indent_str, multiple_attr
        ));

        for (index, item) in accordion.items.iter().enumerate() {
            let item_value = format!("item-{}", index);
            output.push_str(&format!(
                "{}  <Accordion.Item value=\"{}\">\n",
                indent_str, item_value
            ));
            output.push_str(&format!("{}    <Accordion.Header>\n", indent_str));
            output.push_str(&format!(
                "{}      <Accordion.Trigger>\n{}        {}\n{}      </Accordion.Trigger>\n",
                indent_str, indent_str, item.title, indent_str
            ));
            output.push_str(&format!("{}    </Accordion.Header>\n", indent_str));
            output.push_str(&format!(
                "{}      <Accordion.Panel>\n{}        {}\n{}      </Accordion.Panel>\n",
                indent_str, indent_str, item.content, indent_str
            ));
            output.push_str(&format!("{}  </Accordion.Item>\n", indent_str));
        }

        output.push_str(&format!("{}</Accordion.Root>\n", indent_str));

        Ok(output)
    }

    fn generate_dialog(
        dialog: &nwl_shared::DialogElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&dialog.style);

        let close_handler = if let Some(onClose) = &dialog.onClose {
            format!(" onClick={{() => {}}}", onClose)
        } else {
            String::new()
        };

        let wrapper_class = if class_name.is_empty() {
            "".to_string()
        } else {
            format!(" {}", class_name)
        };

        let popup_class = format!("Dialog-popup{}", wrapper_class);

        let mut output = String::new();

        let open_attr = if let Some(open) = &dialog.open {
            format!(" open={{{}}}", Self::to_camel_case(open))
        } else {
            String::new()
        };

        let onopenchange_attr = if let Some(open) = &dialog.open {
            let setter_name = format!("set{}", Self::to_pascal_case(open));
            format!(" onOpenChange={{open => {}(open)}}", setter_name)
        } else {
            String::new()
        };

        output.push_str(&format!(
            "{}<Dialog.Root{}{}>\n",
            indent_str, open_attr, onopenchange_attr
        ));

        if dialog.open.is_none() {
            output.push_str(&format!("{}  <Dialog.Trigger asChild>\n{}    <Button>Open dialog</Button>\n{}  </Dialog.Trigger>\n", indent_str, indent_str, indent_str));
        }

        output.push_str(&format!("{}  <Dialog.Portal>\n", indent_str));
        output.push_str(&format!(
            "{}    <Dialog.Backdrop className=\"Dialog-backdrop\" />\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}    <Dialog.Popup className=\"{}\">\n",
            indent_str, popup_class
        ));

        if let Some(title) = &dialog.title {
            output.push_str(&format!(
                "{}      <Dialog.Title className=\"Dialog-title\">{}</Dialog.Title>\n",
                indent_str, title
            ));
        }

        for child in &dialog.children {
            output.push_str(&format!(
                "{}        {}\n",
                indent_str,
                Self::generate_element(child, indent + 4)?
            ));
        }

        if dialog.onClose.is_some() {
            output.push_str(&format!(
                "{}      <Dialog.Close className=\"Dialog-close\">\n{}        <span className=\"sr-only\">Close</span>\n{}      </Dialog.Close>\n",
                indent_str, indent_str, indent_str
            ));
        }

        output.push_str(&format!("{}    </Dialog.Popup>\n", indent_str));
        output.push_str(&format!("{}  </Dialog.Portal>\n", indent_str));
        output.push_str(&format!("{}</Dialog.Root>\n", indent_str));

        Ok(output)
    }

    fn generate_tooltip(
        tooltip: &nwl_shared::TooltipElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&tooltip.style);

        let side_offset = " sideOffset={8}";

        let wrapper_class = if class_name.is_empty() {
            String::new()
        } else {
            format!(" {}", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!("{}<Tooltip.Provider>\n", indent_str));
        output.push_str(&format!("{}  <Tooltip.Root>\n", indent_str));

        if let Some(trigger) = &tooltip.trigger {
            output.push_str(&format!(
                "{}    <Tooltip.Trigger>\n{}      {}\n{}    </Tooltip.Trigger>\n",
                indent_str, indent_str, trigger, indent_str
            ));
        }

        output.push_str(&format!("{}    <Tooltip.Portal>\n", indent_str));
        output.push_str(&format!(
            "{}      <Tooltip.Positioner sideOffset={{8}}>\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}        <Tooltip.Popup className=\"Tooltip-popup\">\n{}          {}\n{}        </Tooltip.Popup>\n",
            indent_str, indent_str, tooltip.content, indent_str
        ));
        output.push_str(&format!("{}      </Tooltip.Positioner>\n", indent_str));
        output.push_str(&format!("{}    </Tooltip.Portal>\n", indent_str));
        output.push_str(&format!("{}  </Tooltip.Root>\n", indent_str));
        output.push_str(&format!("{}</Tooltip.Provider>\n", indent_str));

        Ok(output)
    }

    fn generate_popover(
        popover: &nwl_shared::PopoverElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);

        let mut output = String::new();
        output.push_str(&format!("{}<Popover.Root>\n", indent_str));

        if let Some(trigger) = &popover.trigger {
            output.push_str(&format!(
                "{}  <Popover.Trigger>\n{}    {}\n{}  </Popover.Trigger>\n",
                indent_str, indent_str, trigger, indent_str
            ));
        }

        output.push_str(&format!("{}  <Popover.Portal>\n", indent_str));
        output.push_str(&format!(
            "{}    <Popover.Positioner sideOffset={{8}}>\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}      <Popover.Popup className=\"Popover-popup\">\n",
            indent_str
        ));

        if let Some(title) = &popover.title {
            output.push_str(&format!(
                "{}        <Popover.Title className=\"Popover-title\">{}</Popover.Title>\n",
                indent_str, title
            ));
        }

        output.push_str(&format!("{}        {}\n", indent_str, popover.content));
        output.push_str(&format!("{}      </Popover.Popup>\n", indent_str));
        output.push_str(&format!("{}    </Popover.Positioner>\n", indent_str));
        output.push_str(&format!("{}  </Popover.Portal>\n", indent_str));
        output.push_str(&format!("{}</Popover.Root>\n", indent_str));

        Ok(output)
    }

    fn generate_badge(
        badge: &nwl_shared::BadgeElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&badge.style);

        let props_str = if class_name.is_empty() {
            " className=\"Badge\"".to_string()
        } else {
            format!(" className=\"Badge {}\"", class_name)
        };

        let variant_class = match badge.variant.as_deref().unwrap_or("default") {
            "success" => " Badge-success",
            "warning" => " Badge-warning",
            "error" => " Badge-error",
            "info" => " Badge-info",
            _ => " Badge-default",
        };

        let final_class = format!("{}{}", props_str, variant_class);

        Ok(format!(
            "{}<span{}>{}</span>",
            indent_str, final_class, badge.content
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
            format!("<button className=\"Tag-close\" {}>×</button>", handler)
        } else {
            String::new()
        };

        let props_str = if class_name.is_empty() {
            "className=\"Tag\"".to_string()
        } else {
            format!("className=\"Tag {}\"", class_name)
        };

        Ok(format!(
            "{}<span {}>{} {}</span>",
            indent_str, props_str, tag.content, close_btn
        ))
    }

    fn generate_alert(
        alert: &nwl_shared::AlertElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&alert.style);

        let dismiss_btn = if alert.dismissible == Some(true) {
            let handler = alert
                .onDismiss
                .as_ref()
                .map(|h| format!("onClick={{() => {}}}", h))
                .unwrap_or_default();
            format!("<button className=\"Alert-dismiss\" {} />", handler)
        } else {
            String::new()
        };

        let variant_class = match alert.alertType.as_deref().unwrap_or("info") {
            "success" => " Alert-success",
            "warning" => " Alert-warning",
            "error" => " Alert-error",
            _ => " Alert-info",
        };

        let base_class = if class_name.is_empty() {
            format!("Alert{}", variant_class)
        } else {
            format!("Alert{} {}", variant_class, class_name)
        };

        Ok(format!(
            "{}<div className=\"{}\">{}<p>{}</p></div>",
            indent_str, base_class, dismiss_btn, alert.content
        ))
    }

    fn generate_spinner(
        spinner: &nwl_shared::SpinnerElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&spinner.style);

        let size_class = match spinner.size.as_deref().unwrap_or("md") {
            "sm" => " Spinner-sm",
            "lg" => " Spinner-lg",
            _ => "",
        };

        let label = spinner
            .label
            .as_ref()
            .map(|l| format!("<span className=\"Spinner-label\">{}</span>", l))
            .unwrap_or_default();

        let container_class = if class_name.is_empty() {
            "Spinner".to_string()
        } else {
            format!("Spinner {}", class_name)
        };

        Ok(format!(
            "{}<div className=\"{}\">{}<div className=\"Spinner-spinner{}\" />{}</div>",
            indent_str,
            container_class,
            label,
            size_class,
            if label.is_empty() {
                "<span className=\"sr-only\">Loading...</span>".to_string()
            } else {
                String::new()
            }
        ))
    }

    fn generate_counter(
        counter: &nwl_shared::CounterElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&counter.style);

        let min_attr = counter
            .min
            .map(|m| format!(" min={{{}}}", m))
            .unwrap_or_default();
        let max_attr = counter
            .max
            .map(|m| format!(" max={{{}}}", m))
            .unwrap_or_default();
        let step_attr = counter
            .step
            .map(|s| format!(" step={{{}}}", s))
            .unwrap_or_default();

        let value_attr = if let Some(bind) = &counter.bind {
            format!(" value={{{}}}", Self::to_camel_case(bind))
        } else {
            String::new()
        };

        let onvaluechange_attr = if let Some(bind) = &counter.bind {
            let setter_name = format!("set{}", Self::to_pascal_case(bind));
            format!(" onValueChange={{(value) => {}(value)}}", setter_name)
        } else {
            String::new()
        };

        let wrapper_class = if class_name.is_empty() {
            "NumberField-root".to_string()
        } else {
            format!("NumberField-root {}", class_name)
        };

        let children = format!(
            "{}<NumberField.Decrement className=\"NumberField-decrement\">\n{}    <span>-</span>\n{}  </NumberField.Decrement>\n{}  <NumberField.Input className=\"NumberField-input\" />\n{}  <NumberField.Increment className=\"NumberField-increment\">\n{}    <span>+</span>\n{}  </NumberField.Increment>",
            indent_str, indent_str, indent_str, indent_str, indent_str, indent_str, indent_str
        );

        Ok(format!(
            "{}<NumberField.Root className=\"{}\"{}{}{}{}{}>\n{}\n{}</NumberField.Root>",
            indent_str,
            wrapper_class,
            value_attr,
            min_attr,
            max_attr,
            step_attr,
            onvaluechange_attr,
            children,
            indent_str
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
            "{}<div>{}<input {} /></div>",
            indent_str,
            if !class_name.is_empty() {
                format!("<span className=\"{}\"></span>", class_name)
            } else {
                String::new()
            },
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

        let button_text = copy
            .text
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| "Copy".to_string());

        Ok(format!(
            "{}<button {} className=\"inline-flex items-center {}\">{}</button>",
            indent_str, handler, class_name, button_text
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

        let (bind_name, setter_name) = if let Some(bind) = &chip.bind {
            let camel = Self::to_camel_case(bind);
            let setter = format!("set{}", Self::to_pascal_case(bind));
            (camel, setter)
        } else {
            (String::new(), String::new())
        };

        let suggestions_with_handlers: Vec<String> = chip.suggestions.iter().map(|suggestion| {
            format!(
                "<span key=\"{0}\" className=\"inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200\" style={{{1}.includes(\"{0}\") ? {{ opacity: 0.5, pointerEvents: 'none' }} : {{}}}} onClick={{() => {{ if (!{1}.includes(\"{0}\")) {{ {2}([...{1}, \"{0}\"]); }} }}}}>{0}</span>",
                suggestion,
                bind_name,
                setter_name
            )
        }).collect();

        let input_handler = if setter_name.is_empty() {
            String::new()
        } else {
            format!(
                "onKeyDown={{(e) => {{ if (e.key === 'Enter') {{ const newValue = e.target.value.trim(); if (newValue && !{0}.includes(newValue)) {{ {1}([...{0}, newValue]); }} e.target.value = ''; }} }}}}",
                bind_name, setter_name
            )
        };

        let chips_display = if bind_name.is_empty() {
            String::new()
        } else {
            format!(
                "<div className=\"flex flex-wrap gap-2 mb-2\">{{{0}.map((chip, i) => <span key={{i}} className=\"px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-sm flex items-center\">{{chip}}<button className=\"ml-1 text-blue-600 hover:text-blue-800\" onClick={{() => {1}({0}.filter((_, j) => j !== i))}}>×</button></span>)}}</div>",
                bind_name, setter_name
            )
        };

        let suggestions_html = if !chip.suggestions.is_empty() {
            format!(
                "<div className=\"mb-2 flex flex-wrap\">{}</div>",
                suggestions_with_handlers.join("")
            )
        } else {
            String::new()
        };

        let placeholder = chip
            .placeholder
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| "Add tags...".to_string());

        Ok(format!(
            "{}<div className=\"{}\">{}{}<input type=\"text\" placeholder=\"{}\" {} className=\"w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500\" /></div>",
            indent_str,
            class_name,
            chips_display,
            suggestions_html,
            placeholder,
            input_handler
        ))
    }

    fn generate_nav(nav: &nwl_shared::NavElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&nav.style);

        let sticky_suffix = if nav.sticky == Some(true) {
            " sticky top-0 z-50"
        } else {
            ""
        };

        let bg_class = if nav.transparent == Some(true) {
            "bg-transparent"
        } else {
            "bg-black"
        };

        let logo_html = if let Some(logo) = &nav.logo {
            format!(
                "<a href=\"/\" className=\"text-xl font-bold text-white\">{}</a>",
                logo
            )
        } else {
            String::new()
        };

        let mut links_html = String::new();
        for link in &nav.links {
            let active_class = if link.active == Some(true) {
                " text-blue-400"
            } else {
                " text-white hover:text-blue-400"
            };
            let href = link
                .href
                .as_ref()
                .map(|h| format!("href=\"{}\"", h))
                .unwrap_or_default();
            links_html.push_str(&format!(
                "{}<a {} className=\"{} transition-colors{}\">{}</a>\n",
                indent_str, href, "text-sm font-medium", active_class, link.label
            ));
        }

        Ok(format!(
            "{}<nav className=\"{} flex items-center justify-between px-6 py-4{} {}\">{}<div className=\"flex items-center gap-6\">{}</div></nav>",
            indent_str,
            class_name,
            sticky_suffix,
            bg_class,
            logo_html,
            links_html
        ))
    }

    fn generate_navigation_menu(
        menu: &nwl_shared::NavigationMenuElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&menu.style);

        let nav_class = if class_name.is_empty() {
            "NavigationMenu".to_string()
        } else {
            format!("NavigationMenu {}", class_name)
        };

        let mut output = String::new();
        output.push_str(&format!(
            "{}<div className=\"{}\">\n",
            indent_str, nav_class
        ));
        output.push_str(&format!(
            "{}  <nav className=\"NavigationMenu-nav flex items-center justify-between px-6 py-4 bg-gray-900 border-b border-gray-700\">\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}    <a href=\"/\" className=\"NavigationMenu-logo\">NWL</a>\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}    <div className=\"hidden md:flex gap-4 NavigationMenu-nav\">\n",
            indent_str
        ));

        for link in &menu.items {
            let href = link.href.as_deref().unwrap_or("#");
            output.push_str(&format!(
                "{}      <a href=\"{}\" className=\"NavigationMenu-link\">{}</a>\n",
                indent_str, href, link.label
            ));
        }
        output.push_str(&format!("{}    </div>\n", indent_str));
        output.push_str(&format!(
            "{}    <Button className=\"Button md:hidden\">\n",
            indent_str
        ));
        output.push_str(&format!(
            "{}      <svg className=\"w-5 h-5\" fill=\"none\" stroke=\"currentColor\" viewBox=\"0 0 24 24\" strokeWidth=\"2\" strokeLinecap=\"round\" strokeLinejoin=\"round\"><path d=\"M4 6h16M4 12h16M4 18h16\"></path></svg>\n",
            indent_str
        ));
        output.push_str(&format!("{}    </Button>\n", indent_str));
        output.push_str(&format!("{}  </nav>\n", indent_str));

        output.push_str(&format!("{}  <Menu.Root>\n", indent_str));
        output.push_str(&format!("{}    <Menu.Portal>\n", indent_str));
        output.push_str(&format!(
            "{}      <Menu.Positioner sideOffset={{8}} className=\"z-50\">\n",
            indent_str
        ));
        output.push_str(&format!("{}        <Menu.Popup className=\"Popover-popup min-w-[200px] p-1 bg-white border rounded-lg shadow-lg\">\n", indent_str));
        output.push_str(&format!(
            "{}          <Menu.Arrow className=\"fill-white\" />\n",
            indent_str
        ));

        for (index, link) in menu.items.iter().enumerate() {
            let href = link
                .href
                .as_ref()
                .map(|h| format!("href=\"{}\"", h))
                .unwrap_or_default();
            output.push_str(&format!(
                "{}          <Menu.Item key={{{}}} asChild>\n{}            <a {} className=\"flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded cursor-pointer\">\n{}              {}\n{}            </a>\n{}          </Menu.Item>\n",
                indent_str, index, indent_str, href, indent_str, link.label, indent_str, indent_str
            ));
        }

        output.push_str(&format!("{}        </Menu.Popup>\n", indent_str));
        output.push_str(&format!("{}      </Menu.Positioner>\n", indent_str));
        output.push_str(&format!("{}    </Menu.Portal>\n", indent_str));
        output.push_str(&format!("{}  </Menu.Root>\n", indent_str));
        output.push_str(&format!("{}</div>\n", indent_str));

        Ok(output)
    }

    fn generate_url(url: &nwl_shared::UrlElement, indent: usize) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);
        let class_name = Self::format_style(&url.style);

        // If it's used as an input (has placeholder or bind), generate input field
        if url.placeholder.is_some() || url.bind.is_some() {
            let placeholder = url
                .placeholder
                .as_ref()
                .map(|s| format!(" placeholder=\"{}\"", s))
                .unwrap_or_default();
            let bind = url
                .bind
                .as_ref()
                .map(|s| format!(" value={{{}}}", Self::to_camel_case(s)))
                .unwrap_or_default();
            let class_prop = if class_name.is_empty() {
                String::new()
            } else {
                format!(" className=\"{}\"", class_name)
            };

            return Ok(format!(
                "{}<input type=\"url\"{}{}{}/>",
                indent_str, placeholder, bind, class_prop
            ));
        }

        // Otherwise, generate anchor link
        let target = url
            .target
            .as_ref()
            .map(|t| format!("target=\"{}\"", t))
            .unwrap_or_default();

        let rel = if url.target.as_ref().map(|t| t == "_blank").unwrap_or(false) {
            "rel=\"noopener noreferrer\""
        } else {
            ""
        };

        let href = url.href.as_deref().unwrap_or("");
        let content = url
            .content
            .as_ref()
            .or(url.href.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("");

        let props = if class_name.is_empty() {
            format!("href=\"{}\"", href)
        } else {
            format!("href=\"{}\" className=\"{}\"", href, class_name)
        };

        Ok(format!(
            "{}<a {} {} {}>{}</a>",
            indent_str, props, target, rel, content
        ))
    }

    fn generate_email(
        email: &nwl_shared::EmailElement,
        indent: usize,
    ) -> Result<String, CodegenError> {
        let indent_str = "  ".repeat(indent);

        // If it's used as an input (has placeholder or bind), generate input field
        if email.placeholder.is_some() || email.bind.is_some() {
            let placeholder = email
                .placeholder
                .as_ref()
                .map(|s| format!(" placeholder=\"{}\"", s))
                .unwrap_or_default();
            let bind = email
                .bind
                .as_ref()
                .map(|s| format!(" value={{{}}}", Self::to_camel_case(s)))
                .unwrap_or_default();
            let class_name = Self::format_style(&email.style);
            let class_prop = if class_name.is_empty() {
                String::new()
            } else {
                format!(" className=\"{}\"", class_name)
            };

            return Ok(format!(
                "{}<input type=\"email\"{}{}{}/>",
                indent_str, placeholder, bind, class_prop
            ));
        }

        // Otherwise, generate mailto link
        let class_name = Self::format_style(&email.style);

        let address = email.address.as_deref().unwrap_or("");
        let mut mailto = format!("mailto:{}", address);
        if let Some(subject) = &email.subject {
            mailto.push_str(&format!("?subject={}", subject));
        }

        let content = email
            .content
            .as_ref()
            .or(email.address.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("");

        let props = if class_name.is_empty() {
            format!("href=\"{}\"", mailto)
        } else {
            format!("href=\"{}\" className=\"{}\"", mailto, class_name)
        };

        Ok(format!("{}<a {}>{}</a>", indent_str, props, content))
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

        for prop in &layout.style {
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

pub fn generate_react(
    page: &Page,
    css_theme: &Option<String>,
    css_override: &Option<String>,
) -> Result<String, CodegenError> {
    ReactGenerator::generate(page, css_theme, css_override)
}

pub fn generate_router(
    name: &str,
    imports: &str,
    routes: &str,
    css_theme: &Option<String>,
    css_override: &Option<String>,
    document: &Document,
) -> String {
    let has_css = css_theme.is_some() || css_override.is_some();

    // Generate processed CSS with proper precedence
    let _css_content = if has_css {
        let inline_styles = css_processing::collect_inline_styles(document);
        let processed = css_processing::process_css(
            &std::path::PathBuf::from("."),
            css_theme,
            css_override,
            &inline_styles,
        )
        .unwrap_or_else(|_| css_processing::ProcessedCss {
            rules: Vec::new(),
            import_statements: Vec::new(),
        });
        css_processing::generate_css_output(&processed.rules)
    } else {
        String::new()
    };

    let css_import = if has_css {
        "import '../themes/processed.css';\n"
    } else {
        ""
    };

    format!(
        r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import {{ BrowserRouter, Routes, Route }} from 'react-router-dom'
import './index.css'
{}

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
        css_import, imports, routes
    )
}

pub fn generate_css(document: &Document) -> Result<String, CodegenError> {
    let mut css_output = String::new();
    let mut override_css = String::new();
    let mut has_custom_theme = false;

    for page in &document.pages {
        if let Some(theme) = &page.page_data.css_theme {
            has_custom_theme = true;
            if theme == "default" {
                css_output.push_str("/* Default Base UI Theme */\n");
                css_output.push_str(include_str!("../../../themes/default.css"));
                css_output.push('\n');
            } else {
                css_output.push_str(&format!("/* Custom theme: {} */\n", theme));
                css_output.push_str(&format!("@import './themes/{}.css';\n", theme));
            }
        }

        if let Some(override_file) = &page.page_data.css_override {
            override_css.push_str(&format!("/* Override styles: {} */\n", override_file));
            if override_file == "default" {
                override_css.push_str("/* Add your custom override styles here */\n");
            } else {
                override_css.push_str(&format!("@import './themes/{}';\n", override_file));
            }
        }
    }

    if !has_custom_theme {
        css_output.push_str("/* Default Base UI Theme */\n");
        css_output.push_str(include_str!("../../../themes/default.css"));
        css_output.push('\n');
    }

    if override_css.is_empty() {
        override_css.push_str("/* Theme Overrides */\n");
        override_css.push_str("/* Add your custom override styles here */\n");
    }

    Ok(format!(
        "{}\n===OVERRIDE_FILE===\n{}",
        css_output, override_css
    ))
}

pub fn get_css_import_path(
    document: &Document,
    css_theme: &Option<String>,
    css_override: &Option<String>,
) -> (String, String) {
    let has_project_css = css_theme.is_some() || css_override.is_some();
    let has_page_css = document
        .pages
        .iter()
        .any(|p| p.page_data.css_theme.is_some() || p.page_data.css_override.is_some());

    if has_project_css || has_page_css {
        (
            "./themes/theme.css".to_string(),
            "./themes/theme.override.css".to_string(),
        )
    } else {
        (String::new(), String::new())
    }
}

impl From<CodegenError> for nwl_shared::CompileError {
    fn from(e: CodegenError) -> Self {
        nwl_shared::CompileError::Codegen(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nwl_shared::{
        ButtonElement, Document, Element, HeadingElement, Layout, LayoutType, Page, PageData,
        StateDefinition,
    };

    fn make_simple_page() -> Page {
        Page {
            page_data: PageData {
                name: "Test".to_string(),
                layout: None,
                style: vec![],
                children: vec![
                    Element::Heading(HeadingElement {
                        content: "Hello World".to_string(),
                        style: vec!["text-2xl".to_string(), "font-bold".to_string()],
                    }),
                    Element::Button(ButtonElement {
                        content: "Click Me".to_string(),
                        onClick: Some("handleClick()".to_string()),
                        style: vec!["bg-blue-500".to_string(), "text-white".to_string()],
                    }),
                ],
                state: vec![],
                css_theme: None,
                css_override: None,
            },
        }
    }

    #[test]
    fn test_generate_simple_page() {
        let page = make_simple_page();
        let document = Document { pages: vec![page] };
        let result = generate_react(&document, &None, &None);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("export default function Test()"));
        assert!(code.contains("Hello World"));
        assert!(code.contains("Click Me"));
        assert!(code.contains("handleClick()"));
    }

    #[test]
    fn test_generate_heading_component() {
        let page = Page {
            page_data: PageData {
                name: "HeadingTest".to_string(),
                layout: None,
                style: vec![],
                children: vec![Element::Heading(HeadingElement {
                    content: "Test Heading".to_string(),
                    style: vec!["text-xl".to_string(), "font-semibold".to_string()],
                })],
                state: vec![],
                css_theme: None,
                css_override: None,
            },
        };
        let document = Document { pages: vec![page] };
        let result = generate_react(&document, &None, &None).unwrap();
        assert!(result.contains("<h1"));
        assert!(result.contains("Test Heading"));
    }

    #[test]
    fn test_generate_button_with_onclick() {
        let page = Page {
            page_data: PageData {
                name: "ButtonTest".to_string(),
                layout: None,
                style: vec![],
                children: vec![Element::Button(ButtonElement {
                    content: "Submit".to_string(),
                    onClick: Some("handleSubmit()".to_string()),
                    style: vec!["bg-primary".to_string()],
                })],
                state: vec![],
                css_theme: None,
                css_override: None,
            },
        };
        let document = Document { pages: vec![page] };
        let result = generate_react(&document, &None, &None).unwrap();
        assert!(result.contains("onClick={() => handleSubmit()}"));
        assert!(result.contains("Submit"));
    }

    #[test]
    fn test_generate_page_with_layout() {
        let page = Page {
            page_data: PageData {
                name: "LayoutTest".to_string(),
                layout: Some(Layout {
                    layout_type: LayoutType::Column,
                    columns: None,
                    properties: vec!["items-center".to_string(), "gap-4".to_string()],
                }),
                style: vec!["bg-gray-100".to_string()],
                children: vec![Element::Heading(HeadingElement {
                    content: "Title".to_string(),
                    style: vec![],
                })],
                state: vec![],
                css_theme: None,
                css_override: None,
            },
        };
        let document = Document { pages: vec![page] };
        let result = generate_react(&document, &None, &None).unwrap();
        // Page layout creates a div with the page style (bg-gray-100)
        assert!(result.contains("className=\"bg-gray-100\""));
        assert!(result.contains("Title"));
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(ReactGenerator::to_pascal_case("hello"), "Hello");
        assert_eq!(ReactGenerator::to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(ReactGenerator::to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(ReactGenerator::to_pascal_case("hello world"), "HelloWorld");
    }

    #[test]
    fn test_to_camel_case() {
        // to_camel_case preserves first char, only capitalizes after delimiters
        assert_eq!(ReactGenerator::to_camel_case("hello"), "hello");
        assert_eq!(ReactGenerator::to_camel_case("hello_world"), "helloWorld");
        assert_eq!(ReactGenerator::to_camel_case("hello-world"), "helloWorld");
        assert_eq!(ReactGenerator::to_camel_case("hello world"), "helloWorld");
        assert_eq!(ReactGenerator::to_camel_case("Hello"), "Hello");
        assert_eq!(ReactGenerator::to_camel_case("HelloWorld"), "HelloWorld");
        assert_eq!(ReactGenerator::to_camel_case("HELLO"), "HELLO");
    }

    #[test]
    fn test_generate_multiple_pages() {
        let page1 = Page {
            page_data: PageData {
                name: "Home".to_string(),
                layout: None,
                style: vec![],
                children: vec![Element::Heading(HeadingElement {
                    content: "Welcome".to_string(),
                    style: vec![],
                })],
                state: vec![],
                css_theme: None,
                css_override: None,
            },
        };
        let page2 = Page {
            page_data: PageData {
                name: "About".to_string(),
                layout: None,
                style: vec![],
                children: vec![Element::Heading(HeadingElement {
                    content: "About Us".to_string(),
                    style: vec![],
                })],
                state: vec![],
                css_theme: None,
                css_override: None,
            },
        };
        let document = Document {
            pages: vec![page1, page2],
        };
        let result = generate_react(&document, &None, &None).unwrap();
        assert!(result.contains("function Home()"));
        assert!(result.contains("function About()"));
        assert!(result.contains("Welcome"));
        assert!(result.contains("About Us"));
    }

    #[test]
    fn test_generate_router() {
        let imports = "import Home from './home';\nimport About from './about';".to_string();
        let routes = "        <Route path=\"/\" element={<Home />}} />\n        <Route path=\"/about\" element={<About />}} />\n".to_string();
        let router = generate_router("TestApp", &imports, &routes, &None, &None);
        assert!(router.contains("import React from 'react'"));
        assert!(router.contains("BrowserRouter"));
        assert!(router.contains("Home"));
        assert!(router.contains("About"));
        assert!(router.contains("\"/\""));
        assert!(router.contains("\"/about\""));
    }

    #[test]
    fn test_generate_page_with_state() {
        let page = Page {
            page_data: PageData {
                name: "Counter".to_string(),
                layout: None,
                style: vec![],
                children: vec![Element::Button(ButtonElement {
                    content: "Count: 0".to_string(),
                    style: vec![],
                    onClick: None,
                })],
                state: vec![StateDefinition {
                    name: "count".to_string(),
                    value_type: None,
                    initial: Some(serde_yaml::Value::Number(0.into())),
                }],
                css_theme: None,
                css_override: None,
            },
        };
        let document = Document { pages: vec![page] };
        let result = generate_react(&document, &None, &None).unwrap();
        assert!(result.contains("useState"));
        assert!(result.contains("count"));
        assert!(result.contains("setCount"));
    }

    #[test]
    fn test_format_layout_props() {
        let layout = Layout {
            layout_type: LayoutType::Column,
            columns: None,
            properties: vec!["items-center".to_string(), "gap-4".to_string()],
        };
        let result = ReactGenerator::format_layout_props(&layout);
        assert!(result.contains("flex flex-col"));
        assert!(result.contains("items-center"));
        assert!(result.contains("gap-4"));
    }
}
