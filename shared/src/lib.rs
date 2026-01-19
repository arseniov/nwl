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
    #[serde(rename = "modal")]
    Modal(ModalElement),
    #[serde(rename = "rating")]
    Rating(RatingElement),
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
pub struct FormElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onSubmit: Option<String>,
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
pub struct ModalElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onClose: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RatingElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub showValue: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onChange: Option<String>,
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
