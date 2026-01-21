use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(rename = "checkbox")]
    Checkbox(CheckboxElement),
    #[serde(rename = "slider")]
    Slider(SliderElement),
    #[serde(rename = "select")]
    Select(SelectElement),
    #[serde(rename = "radio-group")]
    RadioGroup(RadioGroupElement),
    #[serde(rename = "textarea")]
    Textarea(TextareaElement),
    #[serde(rename = "form")]
    Form(FormElement),
    #[serde(rename = "field")]
    Field(FieldElement),
    #[serde(rename = "fieldset")]
    Fieldset(FieldsetElement),
    #[serde(rename = "date-input")]
    DateInput(DateInputElement),
    #[serde(rename = "time-input")]
    TimeInput(TimeInputElement),
    #[serde(rename = "datetime-input")]
    DateTimeInput(DateTimeInputElement),
    #[serde(rename = "color-picker")]
    ColorPicker(ColorPickerElement),
    #[serde(rename = "file-upload")]
    FileUpload(FileUploadElement),
    #[serde(rename = "progress")]
    Progress(ProgressElement),
    #[serde(rename = "toggle")]
    Toggle(ToggleElement),
    #[serde(rename = "tabs")]
    Tabs(TabsElement),
    #[serde(rename = "accordion")]
    Accordion(AccordionElement),
    #[serde(rename = "dialog")]
    Dialog(DialogElement),
    #[serde(rename = "tooltip")]
    Tooltip(TooltipElement),
    #[serde(rename = "popover")]
    Popover(PopoverElement),
    #[serde(rename = "badge")]
    Badge(BadgeElement),
    #[serde(rename = "tag")]
    Tag(TagElement),
    #[serde(rename = "alert")]
    Alert(AlertElement),
    #[serde(rename = "spinner")]
    Spinner(SpinnerElement),
    #[serde(rename = "counter")]
    Counter(CounterElement),
    #[serde(rename = "search-input")]
    SearchInput(SearchInputElement),
    #[serde(rename = "copy-button")]
    CopyButton(CopyButtonElement),
    #[serde(rename = "pagination")]
    Pagination(PaginationElement),
    #[serde(rename = "breadcrumb")]
    Breadcrumb(BreadcrumbElement),
    #[serde(rename = "avatar")]
    Avatar(AvatarElement),
    #[serde(rename = "chip-input")]
    ChipInput(ChipInputElement),
    #[serde(rename = "nav")]
    Nav(NavElement),
    #[serde(rename = "navigation-menu")]
    NavigationMenu(NavigationMenuElement),
    #[serde(rename = "url")]
    Url(UrlElement),
    #[serde(rename = "email")]
    Email(EmailElement),
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
pub struct ListItem {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onClick: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
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
    pub bind: Option<String>,
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
pub struct CheckboxElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliderElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<SelectOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadioOption {
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadioGroupElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<RadioOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextareaElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptchaProvider {
    #[serde(rename = "cloudflare")]
    Cloudflare,
    #[serde(rename = "recaptcha")]
    Recaptcha,
    #[serde(rename = "hcaptcha")]
    HCaptcha,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptchaConfig {
    pub provider: CaptchaProvider,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub siteKey: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minLength: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxLength: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onSubmit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<HashMap<String, Vec<ValidationRule>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captcha: Option<CaptchaConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onValidationError: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldElement {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldsetElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateInputElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeInputElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeInputElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorPickerElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub showPalette: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileUploadElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxSize: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub showLabel: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToggleElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onColor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offColor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabItem {
    #[serde(rename = "id")]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabsElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<TabItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccordionItem {
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccordionElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AccordionItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onClose: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onRemove: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlertElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alertType: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onDismiss: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpinnerElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CounterElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchInputElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clearable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onSearch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyButtonElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onCopy: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaginationElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perPage: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbItem {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BreadcrumbItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvatarElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChipInputElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onAdd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onRemove: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavLink {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<NavLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationMenuElement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NavLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobileBreakpoint: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hamburger: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mobileStyle: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TooltipElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopoverElement {
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<StateDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDefinition {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial: Option<serde_yaml::Value>,
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
pub struct ProjectConfig {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<RouteConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    pub page: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_page() {
        let yaml = r#"
page:
  name: Test
  children:
    - element: heading
      content: "Hello"
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse simple page");
        assert_eq!(page.page_data.name, "Test");
        assert_eq!(page.page_data.children.len(), 1);
    }

    #[test]
    fn test_parse_page_with_state() {
        let yaml = r#"
page:
  name: Counter
  state:
    - name: count
      initial: 0
    - name: name
      initial: "Guest"
  children:
    - element: heading
      content: "Counter"
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse page with state");
        assert_eq!(page.page_data.name, "Counter");
        assert_eq!(page.page_data.state.len(), 2);
        assert_eq!(page.page_data.state[0].name, "count");
        assert_eq!(
            page.page_data.state[0].initial,
            Some(serde_yaml::Value::Number(0.into()))
        );
    }

    #[test]
    fn test_parse_page_with_layout() {
        let yaml = r#"
page:
  name: LayoutTest
  layout:
    type: column
    properties: [items-center, gap-4]
  children:
    - element: heading
      content: "Title"
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse page with layout");
        assert!(page.page_data.layout.is_some());
        let layout = page.page_data.layout.unwrap();
        assert_eq!(layout.layout_type, LayoutType::Column);
        assert_eq!(layout.properties, vec!["items-center", "gap-4"]);
    }

    #[test]
    fn test_parse_validation_rule() {
        let yaml = r#"
page:
  name: FormTest
  children:
    - element: form
      validation:
        email:
          - required: true
            message: "Email is required"
          - pattern: "^[^@]+@[^@]+$"
            message: "Invalid email"
        password:
          - required: true
            minLength: 8
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse validation rules");
        let form = match &page.page_data.children[0] {
            Element::Form(f) => f,
            _ => panic!("Expected Form element"),
        };
        assert!(form.validation.is_some());
        let validation = form.validation.as_ref().unwrap();
        assert!(validation.contains_key("email"));
        let email_rules = validation.get("email").unwrap();
        assert_eq!(email_rules.len(), 2);
    }

    #[test]
    fn test_parse_tabs() {
        let yaml = r#"
page:
  name: TabsTest
  state:
    - name: activeTab
      initial: "tab1"
  children:
    - element: tabs
      bind: activeTab
      options:
        - id: tab1
          label: "Tab 1"
        - id: tab2
          label: "Tab 2"
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse tabs");
        let tabs = match &page.page_data.children[0] {
            Element::Tabs(t) => t,
            _ => panic!(
                "Expected Tabs element, got {:?}",
                page.page_data.children[0]
            ),
        };
        assert_eq!(tabs.options.len(), 2);
        assert_eq!(tabs.options[0].value, "tab1");
    }

    #[test]
    fn test_parse_nav_element() {
        let yaml = r#"
page:
  name: NavTest
  children:
    - element: nav
      logo: "MyApp"
      links:
        - label: Home
          href: "/"
        - label: About
          href: "/about"
          active: true
      sticky: true
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse nav");
        let nav = match &page.page_data.children[0] {
            Element::Nav(n) => n,
            _ => panic!("Expected Nav element"),
        };
        assert_eq!(nav.logo, Some("MyApp".to_string()));
        assert_eq!(nav.links.len(), 2);
        assert_eq!(nav.links[1].active, Some(true));
    }

    #[test]
    fn test_parse_list_element() {
        let yaml = r#"
page:
  name: ListTest
  children:
    - element: list
      items:
        - content: "Item 1"
          onClick: "handleClick('item1')"
        - content: "Item 2"
          onClick: "handleClick('item2')"
      style: [border, rounded]
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse list");
        let list = match &page.page_data.children[0] {
            Element::List(l) => l,
            _ => panic!("Expected List element"),
        };
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].content, "Item 1");
    }

    #[test]
    fn test_parse_email_element_as_input() {
        let yaml = r#"
page:
  name: EmailTest
  children:
    - element: email
      placeholder: "Enter email"
      bind: email
      style: [w-full, border]
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse email as input");
        let email = match &page.page_data.children[0] {
            Element::Email(e) => e,
            _ => panic!("Expected Email element"),
        };
        assert_eq!(email.placeholder, Some("Enter email".to_string()));
        assert_eq!(email.bind, Some("email".to_string()));
    }

    #[test]
    fn test_parse_url_element_as_input() {
        let yaml = r#"
page:
  name: UrlTest
  children:
    - element: url
      placeholder: "Enter URL"
      style: [w-full]
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse url as input");
        let url = match &page.page_data.children[0] {
            Element::Url(u) => u,
            _ => panic!("Expected Url element"),
        };
        assert_eq!(url.placeholder, Some("Enter URL".to_string()));
    }

    #[test]
    fn test_parse_select_options() {
        let yaml = r#"
page:
  name: SelectTest
  children:
    - element: select
      bind: country
      options:
        - value: ""
          label: "Choose country"
        - value: us
          label: "United States"
        - value: uk
          label: "United Kingdom"
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse select");
        let select = match &page.page_data.children[0] {
            Element::Select(s) => s,
            _ => panic!("Expected Select element"),
        };
        assert_eq!(select.options.len(), 3);
        assert_eq!(select.options[0].value, "");
    }

    #[test]
    fn test_parse_captcha_config() {
        let yaml = r#"
page:
  name: CaptchaTest
  children:
    - element: form
      captcha:
        provider: cloudflare
        siteKey: "test-key"
        theme: light
"#;
        let page: Page = serde_yaml::from_str(yaml).expect("Should parse captcha config");
        let form = match &page.page_data.children[0] {
            Element::Form(f) => f,
            _ => panic!("Expected Form element"),
        };
        assert!(form.captcha.is_some());
        let captcha = form.captcha.as_ref().unwrap();
        assert_eq!(captcha.provider, CaptchaProvider::Cloudflare);
    }
}
