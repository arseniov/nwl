# DSL for UI Development

A declarative UI + app DSL that compiles to React (web) and React Native (mobile). Developers describe intent, not implementation.

---

## 1. Core Concept

**Not building:**
- A new programming language
- A React replacement

**Building:**
- A declarative UI + app DSL that compiles to React and React Native, not plain HTML
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

```
button "Save"
    style (px-4 py-2 rounded bg-blue-600)
```

OR (if using custom declared classes properties in style file):

```
button "Save"
    style [primary]
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

## 2.5 Technology Stack

**Single language: Rust**

All components built with Rust for consistency and performance.

```
project/
├── compiler/       // DSL → AST → React/Native
├── cli/            // Commands, TUI
├── tunnel/         // Cloudflare integration
├── daemon/         // Dev server, file watcher
├── desktop/        // Tauri wrapper
└── shared/         // Shared types and utilities
```

**Why Rust:**
1. Tauri for desktop (10MB vs Electron's 150MB)
2. ratatui for rich TUI
3. Zero-cost abstractions for the compiler
4. One language, one toolchain
5. WASM support for web-based tools
6. Excellent for concurrent/IO-heavy tasks

---

## 3. Language Specification

### 3.1 File Extensions

`.app`, `.ui`, `.page`, `.view`

### 3.2 Language Structure

All UI definitions use YAML. Elements are keys, properties are values, and children nest via indentation.

```
page:
  name: PageName
  layout:
    type: column | row | stack | grid
    properties: [center, gap-24, ...]
  style: [bg-black, ...]
  children:
    - element:
        [properties...]
        children:
          - [nested elements...]
```

### 3.3 File Structure

```
project/
├── pages/
│   ├── home.yaml
│   └── settings.yaml
├── components/
│   ├── card.yaml
│   └── button.yaml
├── state/
│   ├── home.yaml
│   └── settings.yaml
└── nwl.yaml
```

### 3.4 Page Example (Full Grammar)

```yaml
page:
  name: Home
  layout:
    type: column
    properties: [center, gap-24]
  style: [bg-black]
  children:
    - heading:
        content: "Welcome back"
        style: [text-xl]
    - text:
        content: "Your projects"
    - list:
        data: projects
        children:
          - item:
              children:
                - card:
                    children:
                      - heading:
                          content: project.name
                      - text:
                          content: project.description
                      - button:
                          content: "Open"
                          onClick: "/project/{project.id}"
```

### 3.5 State Example

State is defined separately to keep UI and logic concerns distinct.

```yaml
state:
  - name: count
    initial: 0
  - name: projects
    source: query
    query: db.projects
```

### 3.6 Layout Primitives

No flexbox/grid exposed initially. The compiler maps primitives to the optimal CSS/_native layout:

```yaml
page:
  layout:
    type: column | row | stack | grid
    columns: 3  # only for grid
    properties: [center, gap-24, ...]
```

Compiler decides: flexbox vs grid, breakpoints, responsive behavior.

### 3.7 Styling as Tokens

Styles use token names that map to CSS variables (web) and StyleSheet objects (native):

```yaml
button:
  style: [primary | secondary | destructive | ghost]
  size: [sm | md | lg | xl]
  radius: [sm | md | lg | full]
```

### 3.8 Data Fetching (Server-First)

```yaml
state:
  - name: projects
    source: query
    query: db.projects
    cache: true  # default: true
```

Rules: Runs on server, cached automatically, hydrates only if interactive.

### 3.9 Event Handling

Simple events use handlers:

```yaml inline routes or
button:
  content: "Submit"
  onClick: "/submit"  # route navigation

# For complex logic, reference an action file
button:
  content: "Save"
  onClick: actions/save-project
```

Actions are defined in separate files:

```yaml
# actions/save-project.yaml
action:
  name: saveProject
  inputs: [project]
  handler: |
    count += 1
```

---

## 4. Compiler Architecture

```
YAML file
  ↓
YAML Parser → Native Rust types
  ↓
Validator
  ↓
AST Builder
  ↓
Target Compiler
  ↓
React / React Native
```

### 4.1 Parsing

- Use a battle-tested YAML library (e.g., serde_yaml)
- Validate element keys against component registry
- Resolve dot-notation references (e.g., `project.name`)

### 4.2 Semantic Analysis

Validates:
- Layout hierarchy
- State mutations
- Event usage
- Data boundaries
- Referenced files exist

### 4.3 Target Generation

| Target   | Output                  |
|----------|-------------------------|
| Web      | React + CSS             |
| Native   | React Native            |
| Static   | HTML + CSS              |

---

## 5. MVP Scope

### 5.1 Goal
Build a landing page + dashboard app with zero React knowledge.

### 5.2 MVP Features

**UI:**
- Page
- Layout (row/column)
- Text
- Button
- Card
- List

**Styling:**
- Colors
- Spacing
- Typography
- Theme support

**State:**
- Local state
- Click events

**Build:**
- Compile to React
- Vite dev server
- Static export

### 5.3 Skip in MVP
- Animations
- Forms
- Routing parameters
- Auth
- Mobile

### 5.4 Folder Structure

```
project/
├── compiler/       // DSL → AST → React/Native
├── cli/            // Commands, TUI
├── tunnel/         // Cloudflare integration
├── daemon/         // Dev server, file watcher
├── desktop/        // Tauri wrapper
└── shared/         // Shared types and utilities
```

---

## 6. Roadmap

### Milestone 1: Foundation (0-3 months)

- DSL parser and AST
- React DOM compiler
- Static export
- Basic state management
- Theming system

### Milestone 2: Serious Apps (3-6 months)

- Forms & validation
- Routing & params
- Auth primitives
- Data mutations
- Error boundaries

### Milestone 3: Mobile (6-9 months)

- React Native compiler
- Platform-aware components
- Shared DSL files

### Milestone 4: Power Users (9-15 months)

- Custom components
- Plugin system
- JS/React escape hatches
- Performance optimizations

### Milestone 5: AI-Native (15-24 months)

- AI generates DSL
- Visual ↔ DSL sync
- Refactoring via intent
- Design tokens from Figma

---

## 7. Future-Proofing

**Do early:**
- AST versioning
- Stable core primitives
- Token-based styling
- Compiler warnings, not errors

**Avoid:**
- Leaking React concepts
- Exposing CSS too early
- Too much magic without escape hatches

---

## 8. Build Steps

### Phase 1 – Language core (2-3 weeks)
1. Choose YAML library (serde_yaml)
2. Define component schema/types
3. Build YAML → AST converter
4. Validate element registry
5. Output typed AST

**Success:** YAML files parse to structured AST with full type safety.

### Phase 2 – React compiler (3-4 weeks)
1. Map YAML nodes → React components
2. Generate JSX
3. Generate CSS tokens
4. Wire state → hooks
5. Export React project

**Success:** YAML → working React app.

### Phase 3 – Dev experience (2 weeks)
1. CLI: app dev
2. File watcher
3. Hot reload
4. Error messages mapped to YAML line/column

### Phase 4 – Prebuilt components (2 weeks)
- Button
- Card
- Input
- Modal
- Table

Each: stylable, accessible, theme-aware.

---

## 9. Why This Works

Attacking:
- Cognitive load
- Fragmentation
- Boilerplate
- Framework fatigue

Industry is moving in this direction.
