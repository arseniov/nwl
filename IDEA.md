# NWL - Natural Web Language

A declarative UI + app framework that compiles to React (web) and React Native (mobile). Developers describe intent, not implementation.

---

## 1. Core Concept

**Not building:**
- A new programming language
- A React replacement

**Building:**
- A declarative UI + app framework that compiles to React and React Native, not plain HTML
- Think of it as: Figma → code, but for developers / Markdown → HTML, but for full apps / SQL for UI

**Mental model for users (3 concepts only):**
1. Pages
2. Components
3. Data

Everything else is implicit.

---

## 2. Design Principles

### 1. Intent over mechanics

If using straight Tailwind classes:

```yaml
button:
  content: "Save"
  style: [px-4, py-2, rounded, bg-blue-600]
```

OR (if using custom declared classes in a theme file):

```yaml
button:
  content: "Save"
  style: [primary]
```

NOT: `<button className="px-4 py-2 rounded bg-blue-600">`

### 2. Zero HTML/CSS/JS knowledge required

- HTML/CSS/JS are targets, not inputs
- Users never touch them unless they opt in

### 3. Compile-time intelligence

- Server vs client decided automatically
- Styling generated once
- Accessibility baked in

### 4. Progressive escape hatches

- Advanced users can inject React, JS, or CSS
- Beginners never need to

### 5. One syntax, many targets

- React DOM
- React Native
- (later) Web Components, email, PDFs

---

## 3. Technology Stack

### Single Language: Rust

All components built with Rust for consistency and performance.

```
nwl/
├── compiler/       # YAML → AST → React/Native
├── cli/            # Commands, TUI
└── shared/         # Shared types and utilities
```

**Why Rust:**
1. Zero-cost abstractions for the compiler
2. One language, one toolchain
3. WASM support for web-based tools
4. Excellent for concurrent/IO-heavy tasks
5. Great error messages

---

## 4. Language Specification

### 4.1 File Format

All UI definitions use YAML. Elements are keys, properties are values, and children nest via indentation.

### 4.2 File Structure

```
project/
├── nwl.yaml              # Route configuration
├── pages/                # Page definitions
│   ├── home.yaml
│   └── settings.yaml
└── src/                  # Generated React code
    ├── main.tsx
    ├── home.tsx
    └── settings.tsx
```

### 4.3 Route Configuration (nwl.yaml)

```yaml
name: My App

routes:
  - path: /
    page: pages/home.yaml
  - path: /landing
    page: pages/landing.yaml
```

### 4.4 Page Grammar

```yaml
page:
  name: PageName
  layout:
    type: column | row | stack | grid
    properties: [center, gap-24, ...]
    columns: 3  # only for grid
  style: [bg-black, ...]
  children:
    - element: type
        [properties...]
        children:
          - [nested elements...]
```

### 4.5 Page Example

```yaml
page:
  name: Home
  layout:
    type: column
    properties: [center, gap-24]
  style: [bg-gray-50, min-h-screen]
  children:
    - element: heading
      content: "Welcome back"
      style: [text-3xl, font-bold, text-gray-900]
    - element: text
      content: "Your projects"
      style: [text-xl, text-gray-600]
    - element: list
      data: projects
      children:
        - element: container
          children:
            - element: heading
              content: project.name
            - element: button
              content: "Open"
              onClick: "/project/{project.id}"
```

### 4.6 Available Elements

| Element | Description |
|---------|-------------|
| `page` | Root page container |
| `container` | Generic container div |
| `layout` | Layout wrapper with type (column/row/grid) |
| `heading` | H1 heading element |
| `text` | Paragraph text |
| `button` | Clickable button |
| `card` | Card container (styled div) |
| `list` | List with data binding |
| `image` | Image element |
| `input` | Text input |
| `spacer` | Spacing element |

### 4.7 Layout Types

- `column` - Flex column
- `row` - Flex row
- `stack` - Relative positioning
- `grid` - CSS grid (with `columns: N`)

### 4.8 Styling

Styles use Tailwind CSS class names:

```yaml
button:
  content: "Submit"
  style: [bg-blue-600, text-white, px-4, py-2, rounded-lg, font-medium]
```

### 4.9 Event Handling

```yaml
button:
  content: "Open Project"
  onClick: "/project/123"  # Route navigation
  # onClick: handlers/save  # Future: action file reference
```

### 4.10 Data Binding (Future)

```yaml
list:
  data: projects
  children:
    - element: heading
      content: project.name  # Dot-notation access
```

---

## 5. Compiler Architecture

```
YAML file
  ↓
serde_yaml (battle-tested parser)
  ↓
Validator (check elements, references)
  ↓
AST Builder
  ↓
React Code Generator
  ↓
React + CSS (web)
```

### 5.1 Parsing

- Uses `serde_yaml` for robust YAML parsing
- Validates element keys against component registry
- Resolves dot-notation references

### 5.2 Semantic Analysis

Validates:
- Layout hierarchy
- Event handlers
- Data boundaries
- Referenced files exist

### 5.3 Target Generation

| Target | Output |
|--------|--------|
| Web | React + CSS (Tailwind) |
| Native | React Native (future) |

---

## 6. Project Structure

```
nwl/
├── Cargo.toml              # Rust workspace
├── dev.sh                  # Build script
├── IDEA.md                 # Concept & idea
├── ROADMAP.md              # Development roadmap
├── shared/                 # Shared Rust types
│   └── src/lib.rs
├── compiler/               # YAML → React compiler
│   └── src/
│       ├── lib.rs          # Compiler entry point
│       └── codegen/mod.rs  # React code generation
├── cli/                    # NWL CLI tool
│   └── src/main.rs
└── examples/demo/          # Demo project
    ├── nwl.yaml
    ├── pages/
    └── src/
```

---

## 7. Quick Start

```bash
# Install dependencies
cd examples/demo
npm install

# Build project (generates routes from nwl.yaml)
cd ../..
./dev.sh build-project

# Start dev server
./dev.sh dev

# Add a new page
# 1. Create pages/new-page.yaml
# 2. Add route to examples/demo/nwl.yaml
# 3. Done! Routes auto-generate
```

---

## 8. Why NWL Works

- **Simplicity** - 3 concepts only (Pages, Components, Data)
- **Performance** - Rust compiler, zero runtime overhead
- **Familiarity** - YAML syntax developers already know
- **Type Safety** - Compile-time validation
- **Progressive** - Escape hatches for advanced users
- **Future-Proof** - Multi-target support (Web → Native → Mobile)
