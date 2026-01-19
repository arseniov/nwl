# DSL for UI Development

A declarative UI DSL that compiles to React with Tailwind CSS.

## Quick Start

### 1. Build the DSL Compiler

```bash
cargo build --release
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Compile DSL to React

```bash
./target/release/dsl compile examples/home.app -o src/App.tsx
```

### 4. Start Development Server

```bash
npm run dev
```

Then open the URL shown (usually http://localhost:5173) to see your compiled DSL app.

## Development Workflow

```bash
# Build the compiler
./dev.sh build

# Compile DSL to React
./dev.sh compile

# Start Vite dev server
./dev.sh dev

# Watch DSL files and auto-compile on changes
./dev.sh watch
```

## DSL Syntax

```dsl
page Home:
  layout [column gap-4 p-8 items-center]
  heading "My App"

  card:
    heading "Counter"
    button "Increment":
      text "Clicked!"

  card:
    heading "List"
    list items=[a, b, c]:
      text "{item}"
```

### Elements
- `page <Name>` - Page component
- `layout [column|row <classes...>]` - Layout container
- `heading "Text"` - Heading element
- `text "Text"` - Paragraph text
- `button "Label"` - Button element
- `button "Label":` - Button with children
- `card:` - Card container
- `card: <children>` - Card with children
- `list <variable>=[items]:` - List iteration
- `style [<tailwind-classes>]` - Apply Tailwind classes

### Layout Types
- `column` - flex-col
- `row` - flex-row

### Tailwind Classes
All Tailwind CSS classes are supported in layout and style directives.

## Example

Input:
```dsl
page Home:
  layout [column center gap-4]
  heading "Hello"
```

Output:
```tsx
export default function Home() {
  return (
    <>
      <div className="flex flex-col center gap-4">
        <p className="text-xl">Hello</p>
      </div>
    </>
  );
}
```
