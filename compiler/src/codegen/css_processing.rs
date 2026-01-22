use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use nwl_shared::{Document, Page, PageData};

#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: String,
    pub properties: HashMap<String, String>,
    pub is_pseudo: bool,
}

#[derive(Debug)]
pub struct ProcessedCss {
    pub rules: Vec<CssRule>,
    pub import_statements: Vec<String>,
}

pub fn process_css(
    project_dir: &PathBuf,
    css_theme: &Option<String>,
    css_override: &Option<String>,
    inline_styles: &HashMap<String, Vec<String>>,
) -> Result<ProcessedCss, String> {
    let mut rules: Vec<CssRule> = Vec::new();
    let mut import_statements: Vec<String> = Vec::new();

    // Step 1: Process base theme
    if let Some(ref theme) = css_theme {
        if theme == "default" {
            // Use built-in default theme
            let default_theme = include_str!("../../../themes/default.css");
            let parsed = parse_css(default_theme)?;
            rules.extend(parsed);
        } else {
            // Custom theme file
            let theme_path = project_dir.join("themes").join(format!("{}.css", theme));
            if theme_path.exists() {
                let theme_content = fs::read_to_string(&theme_path)
                    .map_err(|e| format!("Failed to read theme file: {}", e))?;
                let parsed = parse_css(&theme_content)?;
                rules.extend(parsed);
            }
        }
    } else {
        // Default to built-in theme
        let default_theme = include_str!("../../../themes/default.css");
        let parsed = parse_css(default_theme)?;
        rules.extend(parsed);
    }

    // Step 2: Process override theme (merge with base, replacing competing properties)
    if let Some(ref override_file) = css_override {
        let override_file_str = if override_file == "default" {
            "theme.override.css"
        } else {
            override_file.as_str()
        };
        let override_path = project_dir.join("themes").join(override_file_str);
        if override_path.exists() {
            let override_content = fs::read_to_string(&override_path)
                .map_err(|e| format!("Failed to read override file: {}", e))?;
            rules = merge_css_rules(rules, &override_content);
        }
    }

    // Step 3: Apply inline styles from YAML (highest priority)
    apply_inline_styles(&mut rules, inline_styles);

    Ok(ProcessedCss {
        rules,
        import_statements,
    })
}

fn parse_css(content: &str) -> Result<Vec<CssRule>, String> {
    let mut rules = Vec::new();
    let mut current_selector = String::new();
    let mut current_properties = HashMap::new();
    let mut in_rule = false;
    let mut is_pseudo = false;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            if in_rule && !current_selector.is_empty() {
                rules.push(CssRule {
                    selector: current_selector.clone(),
                    properties: current_properties.clone(),
                    is_pseudo,
                });
                current_selector.clear();
                current_properties.clear();
                in_rule = false;
                is_pseudo = false;
            }
            continue;
        }

        // Handle comments
        if line.starts_with("/*") {
            continue;
        }

        // Check for selector start
        if line.ends_with("{") {
            in_rule = true;
            is_pseudo = line.contains(":");
            current_selector = line[..line.len() - 1].trim().to_string();
            current_properties.clear();
        }
        // Check for selector end
        else if line.ends_with("}") {
            if in_rule && !current_selector.is_empty() {
                rules.push(CssRule {
                    selector: current_selector.clone(),
                    properties: current_properties.clone(),
                    is_pseudo,
                });
                current_selector.clear();
                current_properties.clear();
                in_rule = false;
                is_pseudo = false;
            }
        }
        // Property line
        else if in_rule {
            if let Some(colon_pos) = line.find(":") {
                let property = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..]
                    .trim_end_matches(";")
                    .trim()
                    .to_string();
                current_properties.insert(property, value);
            }
        }
    }

    Ok(rules)
}

fn merge_css_rules(base_rules: Vec<CssRule>, override_css: &str) -> Vec<CssRule> {
    let override_rules = parse_css(override_css).unwrap_or_default();

    // Create a map of base rules by selector
    let mut rule_map: HashMap<String, CssRule> = base_rules
        .into_iter()
        .map(|r| (format_selector_key(&r.selector), r))
        .collect();

    // Merge override rules
    for override_rule in override_rules {
        let key = format_selector_key(&override_rule.selector);
        if let Some(existing_rule) = rule_map.get_mut(&key) {
            // Merge properties: override values replace base values
            for (prop, value) in override_rule.properties {
                existing_rule.properties.insert(prop, value);
            }
        } else {
            // Add new rule from override
            rule_map.insert(key, override_rule);
        }
    }

    rule_map.into_values().collect()
}

fn format_selector_key(selector: &str) -> String {
    // Normalize selector for matching
    selector
        .split(":")
        .next()
        .unwrap_or(selector)
        .trim()
        .to_string()
}

fn apply_inline_styles(rules: &mut Vec<CssRule>, inline_styles: &HashMap<String, Vec<String>>) {
    for rule in rules.iter_mut() {
        let key = format_selector_key(&rule.selector);
        if let Some(utility_classes) = inline_styles.get(&key) {
            // Convert utility classes to CSS properties
            for class in utility_classes {
                let (property, value) = resolve_utility_class(class);
                if !property.is_empty() {
                    rule.properties.insert(property, value);
                }
            }
        }
    }
}

fn resolve_utility_class(class: &str) -> (String, String) {
    // Map Tailwind utility classes to CSS properties
    // Format: (property, value)
    let mappings: Vec<(&str, &str, &str)> = vec![
        // Text utilities - font-size
        ("text-sm", "font-size", "0.875rem"),
        ("text-base", "font-size", "1rem"),
        ("text-lg", "font-size", "1.125rem"),
        ("text-xl", "font-size", "1.25rem"),
        ("text-2xl", "font-size", "1.5rem"),
        ("text-3xl", "font-size", "1.875rem"),
        ("text-4xl", "font-size", "2.25rem"),
        ("text-5xl", "font-size", "3rem"),
        // Text utilities - font-weight
        ("font-normal", "font-weight", "400"),
        ("font-medium", "font-weight", "500"),
        ("font-semibold", "font-weight", "600"),
        ("font-bold", "font-weight", "700"),
        // Text utilities - text-align
        ("text-center", "text-align", "center"),
        ("text-left", "text-align", "left"),
        ("text-right", "text-align", "right"),
        // Text utilities - color
        ("text-white", "color", "#ffffff"),
        ("text-black", "color", "#000000"),
        ("text-gray-300", "color", "#d1d5db"),
        ("text-gray-400", "color", "#9ca3af"),
        ("text-gray-500", "color", "#6b7280"),
        ("text-gray-600", "color", "#4b5563"),
        ("text-gray-700", "color", "#374151"),
        ("text-gray-900", "color", "#111827"),
        ("text-blue-400", "color", "#60a5fa"),
        ("text-blue-500", "color", "#3b82f6"),
        ("text-blue-600", "color", "#2563eb"),
        ("text-cyan-400", "color", "#22d3ee"),
        ("text-orange-500", "color", "#f97316"),
        ("text-green-400", "color", "#4ade80"),
        ("text-yellow-400", "color", "#facc15"),
        // Background utilities
        ("bg-white", "background-color", "#ffffff"),
        ("bg-black", "background-color", "#000000"),
        ("bg-gray-50", "background-color", "#f9fafb"),
        ("bg-gray-100", "background-color", "#f3f4f6"),
        ("bg-gray-200", "background-color", "#e5e7eb"),
        ("bg-gray-800", "background-color", "#1f2937"),
        ("bg-gray-900", "background-color", "#111827"),
        ("bg-blue-500", "background-color", "#3b82f6"),
        ("bg-blue-600", "background-color", "#2563eb"),
        ("bg-blue-700", "background-color", "#1d4ed8"),
        ("bg-green-400", "background-color", "#4ade80"),
        // Spacing utilities - padding
        ("p-1", "padding", "0.25rem"),
        ("p-2", "padding", "0.5rem"),
        ("p-3", "padding", "0.75rem"),
        ("p-4", "padding", "1rem"),
        ("p-6", "padding", "1.5rem"),
        ("px-2", "padding-left", "0.5rem"),
        ("px-3", "padding-left", "0.75rem"),
        ("px-4", "padding-left", "1rem"),
        ("px-6", "padding-left", "1.5rem"),
        ("px-8", "padding-left", "2rem"),
        ("py-1", "padding-top", "0.25rem"),
        ("py-2", "padding-top", "0.5rem"),
        ("py-3", "padding-top", "0.75rem"),
        ("py-4", "padding-top", "1rem"),
        ("py-12", "padding-top", "3rem"),
        ("py-16", "padding-top", "4rem"),
        ("py-20", "padding-top", "5rem"),
        // Spacing utilities - margin
        ("mb-1", "margin-bottom", "0.25rem"),
        ("mb-2", "margin-bottom", "0.5rem"),
        ("mb-3", "margin-bottom", "0.75rem"),
        ("mb-4", "margin-bottom", "1rem"),
        ("mb-6", "margin-bottom", "1.5rem"),
        ("mb-8", "margin-bottom", "2rem"),
        ("mt-2", "margin-top", "0.5rem"),
        ("mt-4", "margin-top", "1rem"),
        ("mt-6", "margin-top", "1.5rem"),
        ("mt-12", "margin-top", "3rem"),
        ("mx-auto", "margin-left", "auto"),
        // Flex utilities
        ("flex", "display", "flex"),
        ("inline-flex", "display", "inline-flex"),
        ("flex-row", "flex-direction", "row"),
        ("flex-col", "flex-direction", "column"),
        ("flex-wrap", "flex-wrap", "wrap"),
        ("items-center", "align-items", "center"),
        ("items-start", "align-items", "flex-start"),
        ("items-end", "align-items", "flex-end"),
        ("justify-center", "justify-content", "center"),
        ("justify-between", "justify-content", "space-between"),
        ("justify-end", "justify-content", "flex-end"),
        ("gap-1", "gap", "0.25rem"),
        ("gap-2", "gap", "0.5rem"),
        ("gap-3", "gap", "0.75rem"),
        ("gap-4", "gap", "1rem"),
        ("gap-6", "gap", "1.5rem"),
        ("gap-8", "gap", "2rem"),
        ("flex-1", "flex", "1 1 0%"),
        // Border utilities
        ("border", "border-width", "1px"),
        ("border-2", "border-width", "2px"),
        ("rounded", "border-radius", "0.25rem"),
        ("rounded-lg", "border-radius", "0.5rem"),
        ("rounded-xl", "border-radius", "0.75rem"),
        ("rounded-full", "border-radius", "9999px"),
        // Width/height
        ("w-5", "width", "1.25rem"),
        ("h-5", "height", "1.25rem"),
        ("min-w-280", "min-width", "70rem"),
        ("max-w-2xl", "max-width", "42rem"),
        ("max-w-4xl", "max-width", "56rem"),
        // Other utilities
        (
            "shadow-lg",
            "box-shadow",
            "0 10px 15px -3px rgb(0 0 0 / 0.1)",
        ),
        ("cursor-pointer", "cursor", "pointer"),
        ("overflow-hidden", "overflow", "hidden"),
        ("text-decoration-none", "text-decoration", "none"),
    ];

    for (class_name, property, value) in mappings.iter() {
        if *class_name == class {
            return ((*property).to_string(), (*value).to_string());
        }
    }

    // For unmapped utilities, return empty - they'll be handled by Tailwind
    (String::new(), String::new())
}

pub fn generate_css_output(rules: &[CssRule]) -> String {
    let mut output = String::new();
    output.push_str("/* Processed CSS - Generated by NWL Compiler */\n");
    output.push_str("/* Precedence: Base Theme < Override Theme < YAML Inline Styles */\n\n");

    for rule in rules {
        output.push_str(&format!("{} {{\n", rule.selector));
        for (prop, value) in &rule.properties {
            output.push_str(&format!("  {}: {};\n", prop, value));
        }
        output.push_str("}\n\n");
    }

    output
}

pub fn collect_inline_styles(document: &nwl_shared::Document) -> HashMap<String, Vec<String>> {
    let mut styles = HashMap::new();
    for page in &document.pages {
        collect_page_styles(&page.page_data, &mut styles);
    }
    styles
}

fn collect_page_styles(page: &nwl_shared::PageData, styles: &mut HashMap<String, Vec<String>>) {
    collect_element_styles(&page.children, styles);
}

fn collect_element_styles(
    elements: &[nwl_shared::Element],
    styles: &mut HashMap<String, Vec<String>>,
) {
    for element in elements {
        match element {
            nwl_shared::Element::Button(btn) => {
                if !btn.style.is_empty() {
                    styles.insert("Button".to_string(), btn.style.clone());
                }
            }
            nwl_shared::Element::Card(card) => {
                if !card.style.is_empty() {
                    styles.insert("Card".to_string(), card.style.clone());
                }
            }
            nwl_shared::Element::List(list) => {
                if !list.style.is_empty() {
                    styles.insert("List".to_string(), list.style.clone());
                }
            }
            nwl_shared::Element::Checkbox(checkbox) => {
                if !checkbox.style.is_empty() {
                    styles.insert("Checkbox-root".to_string(), checkbox.style.clone());
                }
            }
            nwl_shared::Element::Select(select) => {
                if !select.style.is_empty() {
                    styles.insert("Select-trigger".to_string(), select.style.clone());
                }
            }
            nwl_shared::Element::RadioGroup(radio) => {
                if !radio.style.is_empty() {
                    styles.insert("RadioGroup".to_string(), radio.style.clone());
                }
            }
            nwl_shared::Element::Toggle(toggle) => {
                if !toggle.style.is_empty() {
                    styles.insert("Switch-root".to_string(), toggle.style.clone());
                }
            }
            nwl_shared::Element::Form(form) => {
                if !form.style.is_empty() {
                    styles.insert("Form".to_string(), form.style.clone());
                }
            }
            nwl_shared::Element::Field(field) => {
                if !field.style.is_empty() {
                    styles.insert("Field-root".to_string(), field.style.clone());
                }
            }
            nwl_shared::Element::Badge(badge) => {
                if !badge.style.is_empty() {
                    styles.insert("Badge".to_string(), badge.style.clone());
                }
            }
            nwl_shared::Element::Tag(tag) => {
                if !tag.style.is_empty() {
                    styles.insert("Tag".to_string(), tag.style.clone());
                }
            }
            nwl_shared::Element::Alert(alert) => {
                if !alert.style.is_empty() {
                    styles.insert("Alert".to_string(), alert.style.clone());
                }
            }
            nwl_shared::Element::Spinner(spinner) => {
                if !spinner.style.is_empty() {
                    styles.insert("Spinner".to_string(), spinner.style.clone());
                }
            }
            nwl_shared::Element::Counter(counter) => {
                if !counter.style.is_empty() {
                    styles.insert("NumberField-root".to_string(), counter.style.clone());
                }
            }
            nwl_shared::Element::Container(container) => {
                collect_container_styles(container, styles);
            }
            nwl_shared::Element::Layout(layout) => {
                collect_element_styles(&layout.children, styles);
            }
            _ => {} // Other elements
        }
    }
}

fn collect_container_styles(
    container: &nwl_shared::ContainerElement,
    styles: &mut HashMap<String, Vec<String>>,
) {
    if !container.style.is_empty() {
        styles.insert("Container".to_string(), container.style.clone());
    }
    collect_element_styles(&container.children, styles);
}
