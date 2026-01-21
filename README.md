# NWL - Natural Web Language

<div align="center">

**A DSL that compiles declarative YAML to production-ready React and React Native code**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-18-blue?logo=react)](https://reactjs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue?logo=typescript)](https://www.typescriptlang.org)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind-3.4-38bdf8?logo=tailwindcss)](https://tailwindcss.com)

</div>

## What is NWL?

NWL (Natural Web Language) is a domain-specific language that lets you describe user interfaces in simple, declarative YAML. The Rust compiler generates production-ready React/React Native TypeScript code with Tailwind CSS styling.

**No JavaScript knowledge required.** Write what you want, not how to build it.

## Features

- 🚀 **Instant Compilation** - Built with Rust for blazing fast builds
- 📝 **Simple YAML Syntax** - Declarative, easy to read and maintain
- 🔒 **Type-Safe Output** - Generates full TypeScript with proper types
- 🎨 **Tailwind CSS Integration** - All Tailwind classes supported
- 🧩 **30+ Built-in Components** - Forms, navigation, display, and more
- 🔄 **Auto State Management** - `useState` hooks generated automatically
- 📦 **Zero Runtime** - Pure compile-time code generation
- 💻 **Full CLI** - Project scaffolding, build, watch mode, and dev server
- ♿ **BaseUI Integration** - Accessible, unstyled components for production apps

## Quick Start

```bash
# 1. Build the CLI
cargo build --release

# 2. Create a new project
./target/release/nwl new my-app --template blank    # Minimal template
./target/release/nwl new my-app --template demo     # Full demo with all components

# 3. Navigate and install dependencies
cd my-app
npm install

# 4. Start development server with watch mode
./target/release/nwl dev --watch

# 5. Open http://localhost:5173
```

## Available Commands

```bash
nwl new <name>          # Create a new project
  --location, -l        # Project location (default: current directory)
  --template, -t        # Template: blank or demo (default: blank)

nwl build [path]        # Compile NWL files to React
  --watch, -w           # Watch for changes and auto-recompile
  --no-test             # Skip JSX validation tests

nwl dev [path]          # Build + start dev server with hot reload
  --port, -p            # Port (default: 5173)
  --host                # Host binding (default: localhost)
  --no-open             # Don't open browser automatically
  --watch, -w           # Watch YAML files and recompile on change
  --no-test             # Skip JSX validation tests

nwl compile <file>      # Compile a single YAML file
  --output, -o          # Output file path
```

## JSX Validation

NWL automatically validates generated JSX/TSX code to ensure:
- Valid React syntax
- Proper Base UI component structure
- No undefined components
- Balanced braces in JSX expressions

The validation runs automatically during `nwl build` and `nwl dev` commands via vitest. You can also run it manually:

```bash
npm run test:jsx
```

## Project Templates

### Blank Template
Minimal starter project with a single home page.

```bash
nwl new my-project --template blank
```

### Demo Template
Full playground with all 30+ components ready to use.

```bash
nwl new my-project --template demo
```

Both templates include:
- Full Vite + React + TypeScript setup
- Tailwind CSS configured
- React Router for navigation
- Hot Module Replacement (HMR)
```

## YAML Syntax

```yaml
page:
  name: MyPage
  state:
    - name: count
      initial: 0
  children:
    - heading:
        content: "Counter: {count}"
        style: [text-2xl, font-bold]
    - button:
        content: "Increment"
        onClick: "setCount(count + 1)"
```

Compiles to:

```tsx
export default function MyPage() {
  const [count, setCount] = useState(0);
  return (
    <>
      <h1 className="text-2xl font-bold">Counter: {count}</h1>
      <button onClick={() => setCount(count + 1)}>Increment</button>
    </>
  );
}
```

## Components

NWL uses [Base UI](https://base-ui.com/react) as its default component library for maximum accessibility and flexibility. All form and interactive components generate BaseUI code.

### BaseUI Components

| Component | BaseUI Import | Generated Code |
|-----------|---------------|----------------|
| `button` | `@base-ui/react` | `<Button className="...">...</Button>` |
| `checkbox` | `@base-ui/react` | `<Checkbox.Root>...</Checkbox.Root>` |
| `select` | `@base-ui/react` | `<Select.Root>...</Select.Root>` |
| `radio-group` | `@base-ui/react` | `<RadioGroup>...</RadioGroup>` |
| `switch` | `@base-ui/react` | `<Switch.Root>...</Switch.Root>` |
| `counter` | `@base-ui/react` | `<NumberField.Root>...</NumberField.Root>` |
| `dialog` | `@base-ui/react` | `<Dialog.Root>...</Dialog.Root>` |
| `navigation-menu` | `@base-ui/react` | `<Menu.Root>...</Menu.Root>` |

### Form Components

| Component | Description |
|-----------|-------------|
| `input` | Text input field |
| `checkbox` | Checkbox with label |
| `slider` | Range slider with min/max |
| `select` | Dropdown select |
| `radio-group` | Radio button group |
| `textarea` | Multi-line text input |
| `form` | Form with submit handler |
| `counter` | Number field with increment/decrement |

### Date & Time

| Component | Description |
|-----------|-------------|
| `date-input` | Date picker |
| `time-input` | Time picker |
| `datetime-input` | Combined date/time picker |

### Selection & Display

| Component | Description |
|-----------|-------------|
| `color-picker` | Color selection input |
| `file-upload` | File upload input |
| `progress` | Progress bar |

### Toggle & Switch

| Component | Description |
|-----------|-------------|
| `toggle` | Toggle switch component |

### Navigation

| Component | Description |
|-----------|-------------|
| `tabs` | Tab navigation |
| `breadcrumb` | Breadcrumb navigation |
| `pagination` | Page navigation |
| `accordion` | Collapsible sections |

### Modal & Dialogs

| Component | Description |
|-----------|-------------|
| `dialog` | Modal dialog using Base UI Dialog |
| `navigation-menu` | Responsive navigation menu with hamburger |

### Navigation

| Component | Description |
|-----------|-------------|
| `nav` | Navigation bar with links |
| `navigation-menu` | Hamburger menu for mobile |

### Input Components

| Component | Description |
|-----------|-------------|
| `search-input` | Search input with search button |
| `counter` | Number counter (+/- buttons) |
| `copy-button` | Copy to clipboard button |
| `avatar` | User avatar display |
| `chip-input` | Tag/chip input with suggestions |

## State Management

Declare state in your page and NWL generates `useState` hooks automatically:

```yaml
page:
  name: MyPage
  state:
    - name: username
      initial: ""
    - name: isLoading
      initial: false
    - name: items
      initial: ["apple", "banana"]
  children:
    - input:
        bind: username
    - checkbox:
        bind: isLoading
```

Generates:

```tsx
export default function MyPage() {
  const [username, setUsername] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [items, setItems] = useState(["apple", "banana"]);
  // ...
}
```

## Two-Way Binding

Use `bind:` to connect inputs to state - NWL handles both reading and writing:

```yaml
- input:
    bind: email
- checkbox:
    bind: agreedToTerms
- slider:
    bind: volume
    min: 0
    max: 100
```

Generates proper event handlers automatically:

```tsx
<input value={email} onChange={(e) => setEmail(e.target.value)} />
<input type="checkbox" checked={agreedToTerms} onChange={() => setAgreedToTerms(!agreedToTerms)} />
<input type="range" value={volume} onChange={(e) => setVolume(e.target.value)} />
```

## Event Handlers

Bind events with JavaScript expressions:

```yaml
- button:
    content: "Click Me"
    onClick: "setCount(count + 1)"
- form:
    onSubmit: "handleSubmit(formData)"
```

## Layout System

Use the `layout` element for container layouts:

```yaml
- layout:
    type: column
    properties: [items-center, gap-4]
  children:
    - heading: "Section 1"
    - heading: "Section 2"
```

Layout types:
- `column` → `flex flex-col`
- `row` → `flex flex-row`
- `grid` → `grid` (use `columns` property for `grid-cols-N`)

## Styling

Apply Tailwind CSS classes with the `style` property:

```yaml
- heading:
    content: "Hello"
    style: [text-3xl, font-bold, text-blue-600, mb-4]
- container:
    style: [bg-white, p-6, rounded-lg, shadow-md]
```

## Project Structure

```
nwl/
├── Cargo.toml           # Rust workspace config
├── dev.sh              # Development workflow script
├── README.md           # This file
├── ROADMAP.md          # Project roadmap
├── shared/             # Shared YAML schema definitions
│   └── src/lib.rs      # Element structs and enums
├── compiler/           # YAML → React compiler
│   └── src/codegen/    # Code generation logic
├── cli/                # CLI entry point
│   └── src/main.rs     # nwl CLI commands
└── examples/demo/      # Demo project
    ├── nwl.yaml        # Route configuration
    └── pages/          # Page YAML files
        ├── home.yaml       # Homepage
        └── playground.yaml # Component playground
```

## Routes Configuration

Define routes in `examples/demo/nwl.yaml`:

```yaml
name: My App
routes:
  - path: /
    page: pages/home.yaml
  - path: /playground
    page: pages/playground.yaml
```

## Development

### Running the Demo

```bash
# Start dev server (builds + watches)
./dev.sh dev

# Or watch for YAML changes only
./dev.sh watch
```

### Building the Compiler

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing Changes

```bash
# Rebuild the project with latest compiler
./dev.sh build-project

# Build and start dev server
./dev.sh dev
```

## Testing

### Rust Tests (Compiler & Shared)

```bash
# Run all Rust tests
cargo test

# Run tests for specific crate
cargo test -p nwl-shared
cargo test -p nwl-compiler

# Run with output
cargo test -- --nocapture
```

### TypeScript Type Checking

```bash
# Type check generated code
cd examples/demo
npm run typecheck
```

### React Component Tests

```bash
# Run vitest tests
cd examples/demo
npm run test        # Watch mode
npm run test:run    # Single run
```

## Example: Interactive Form

```yaml
page:
  name: ContactForm
  state:
    - name: name
      initial: ""
    - name: email
      initial: ""
    - name: message
      initial: ""
  children:
    - heading:
        content: "Contact Us"
        style: [text-2xl, font-bold, mb-6]
    - input:
        placeholder: "Your name"
        bind: name
        style: [w-full, mb-4]
    - input:
        placeholder: "Email"
        bind: email
        style: [w-full, mb-4]
    - textarea:
        placeholder: "Message"
        bind: message
        rows: 4
        style: [w-full, mb-4]
    - button:
        content: "Send Message"
        style: [bg-blue-600, text-white, px-6, py-3, rounded-lg]
        onClick: "console.log({name, email, message})"
```

## Tech Stack

- **Language**: Rust 1.70+
- **Output**: TypeScript 5.x + React 18
- **Component Library**: Base UI (headless, accessible components)
- **Styling**: Tailwind CSS 3.4
- **Build Tool**: Vite 5.x
- **Parser**: serde_yaml

## License

MIT

---

<div align="center">

**Built with Rust + React**

</div>
