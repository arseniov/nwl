# NWL Roadmap

This document outlines the MVP scope, milestones, and development phases.

---

## 1. MVP Scope

### 1.1 Goal

Build a landing page + dashboard app with zero React knowledge.

### 1.2 MVP Features - COMPLETED ✓

**UI Elements:**
- [x] Page
- [x] Layout (column, row, grid)
- [x] Heading
- [x] Text
- [x] Button
- [x] Card
- [x] List
- [x] Image
- [x] Container
- [x] Spacer
- [x] Input

**Styling:**
- [x] Colors (Tailwind classes)
- [x] Spacing
- [x] Typography
- [x] Theme support

**State & Data:**
- [x] Route configuration (nwl.yaml)
- [x] Click events
- [x] Route navigation

**Build System:**
- [x] YAML to React compilation
- [x] Automatic route generation
- [x] Vite dev server integration
- [x] Static export

**Project Structure:**
- [x] YAML-based page definitions
- [x] Component registry
- [x] React Router integration

### 1.3 In Progress

- [x] Local state management (`state:` YAML block) ✓ DONE
- [x] Input binding (`bind:` property) ✓ DONE
- [x] Checkbox, Slider, Select, Radio, Textarea, Form handlers ✓ DONE
- [ ] Data binding (dot-notation like `project.name`)

### 1.4 MVP Skip List (Future)

- Animations
- Forms & validation
- Routing parameters (`/project/:id`)
- Auth primitives
- Data mutations
- React Native target
- Error boundaries

---

## 2. Milestones

### Milestone 1: Foundation ✓ COMPLETED

- [x] YAML parser and AST
- [x] React DOM compiler
- [x] Static export
- [x] Basic theming system
- [x] Automatic route generation

### Milestone 2: Serious Apps (Next)

- [ ] Forms & validation
- [ ] Routing with parameters
- [ ] State management
- [ ] Data mutations
- [ ] Server-side data fetching
- [ ] Error boundaries

### Milestone 3: Mobile (Future)

- React Native compiler
- Platform-aware components
- Shared DSL files

### Milestone 4: Power Users (Future)

- Custom components
- Plugin system
- JS/React escape hatches
- Performance optimizations

### Milestone 5: AI-Native (Future)

- AI generates DSL
- Visual ↔ DSL sync
- Refactoring via intent
- Design tokens from Figma

---

## 3. Build Phases - STATUS

### Phase 1: Language Core ✓ COMPLETED

1. [x] Choose YAML library (serde_yaml)
2. [x] Define component schema/types
3. [x] Build YAML → AST converter
4. [x] Validate element registry
5. [x] Output typed AST

### Phase 2: React Compiler ✓ COMPLETED

1. [x] Map YAML nodes → React components
2. [x] Generate JSX
3. [x] Generate CSS tokens
4. [x] Wire routes
5. [x] Export React project

### Phase 3: Dev Experience ✓ COMPLETED

1. [x] CLI: `nwl build` command
2. [x] File watcher
3. [x] Hot reload (via Vite)
4. [x] Error messages with YAML line/column

### Phase 4: Prebuilt Components - COMPLETED ✓

- [x] Button
- [x] Card
- [x] Input
- [x] Checkbox
- [x] Slider
- [x] Select
- [x] RadioGroup
- [x] Textarea
- [x] Form
- [ ] Modal
- [ ] Table
- [ ] Navigation

### Phase 5: State Management - COMPLETED ✓

- [x] Local state with `state:` YAML block
- [x] Generate useState hooks
- [x] State declarations in components
- [x] State mutations via onClick handlers
- [x] Input two-way binding
- [ ] Computed properties

### Phase 6: Interactive Components - COMPLETED ✓

- [x] Checkbox toggle binding
- [x] Slider range binding
- [x] Select dropdown binding
- [x] Radio group binding
- [x] Textarea binding
- [x] Form submission handling

---

## 6. Why This Works

Attacking:
- Cognitive load - One YAML file per concept
- Fragmentation - Single toolchain (Rust)
- Boilerplate - Auto-generated React
- Framework fatigue - Declarative, not imperative

Industry is moving in this direction - see Power Apps, Bubble, Flutter, SwiftUI.

---

## 7. TODO - Immediate Next Actions

### High Priority - COMPLETED ✓

1. **Implement State Management** ✓ DONE
   - Added `state:` YAML block parsing
   - Generate useState hooks
   - Support for number, string, boolean initial values

2. **Input Binding** ✓ DONE
   - Added `bind:` property to input elements
   - Auto-generate `value={stateVar}` and `onChange` handlers
   - Two-way binding is automatic

3. **Interactive Handlers** ✓ DONE
   - Checkbox toggle (`bind: agreedToTerms`)
   - Slider range (`bind: volume`, min, max, step)
   - Select dropdown (`bind: country`, options)
   - Radio group (`bind: priority`, options)
   - Textarea (`bind: bio`, rows)
   - Form (`onSubmit: handler`)

### Medium Priority

4. **Data Binding**
   - [x] Dot-notation in text content (`{name}`)
   - [ ] Support data context in lists (future)
   - [ ] Add data prop to components (future)

### Lower Priority

5. **Additional Components**
   - Modal component
   - Table component
   - Navigation component

6. **Routing Parameters**
   - Support `/project/:id` syntax
   - Generate dynamic routes
   - Access params in components

7. **Forms**
   - Form validation YAML schema
   - Input validation messages
   - Error state rendering

---

## 8. Available Routes

The demo project includes these routes:

| Route | Description |
|-------|-------------|
| `/` | Home page |
| `/landing` | Landing page |
| `/counter` | Counter with state + input binding |
| `/interactive` | All interactive components demo |

Try the `/interactive` route to see:
- Checkbox toggle with two-way binding
- Slider with min/max/step
- Select dropdown with options
- Radio group with options
- Textarea with binding
- Form with submit handler

---

## 9. YAML Syntax Reference

### State Declaration
```yaml
state:
  - name: count
    initial: 0
  - name: name
    initial: "Guest"
```

### Interactive Elements
```yaml
checkbox:
  bind: agreedToTerms

slider:
  bind: volume
  min: 0
  max: 100
  step: 10
  label: "Volume: {volume}%"

select:
  bind: country
  options:
    - value: us
      label: United States
    - value: uk
      label: United Kingdom

radio-group:
  bind: priority
  options:
    - value: low
      label: Low Priority
    - value: high
      label: High Priority

textarea:
  bind: bio
  rows: 4

form:
  onSubmit: handleSubmit
```

All JavaScript is automatically generated by the compiler.
