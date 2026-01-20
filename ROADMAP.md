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
- [ ] Forms & validation (server-side form submission)
- [ ] Data binding (dot-notation like `project.name`)

### 1.4 MVP Skip List (Future)

- Animations
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

- [x] State management (local state)
- [x] Input binding (two-way)
- [x] Interactive components
- [ ] Forms & validation (server-side form submission)
- [ ] Forms captcha (Cloudflare Turnstile, reCAPTCHA, hCaptcha)
- [ ] Routing with parameters
- [ ] Data mutations
- [ ] Server-side data fetching
- [ ] Error boundaries
- [ ] Custom CSS support (external stylesheet)
- [ ] Reusable YAML components (imports)
- [x] Navigation components (Nav, Menu with mobile hamburger)

### Milestone 2b: CLI & DevOps ✓ COMPLETED

- [x] Interactive project scaffolding (`nwl new`)
- [x] Configurable project location
- [x] Selective build/compile pipeline
- [x] Integrated dev server with live reload
- [ ] Multi-target deployment support:
  - [ ] Node.js (npm) deployment
  - [ ] Docker Compose deployment
  - [ ] Vercel deployment
  - [ ] Netlify deployment
- [ ] Environment configuration management
- [ ] Database schema initialization
- [ ] CI/CD pipeline generation

### Milestone 2c: Full CLI Feature Set ✓ COMPLETED

Complete the `nwl` CLI with a single bundled executable:

- [x] **Project Scaffolding**
  - [x] User-specified project location with validation
  - [x] Warning if location exists or is not empty
  - [x] Templates for different project types (blank, demo)
  - [ ] Automatic package manager detection (npm/pnpm/yarn)

- [x] **Compilation Pipeline**
  - [x] Compile all NWL pages to React components
  - [x] Watch mode for automatic recompilation
  - [ ] Incremental builds for faster compilation
  - [ ] Configurable output directory

- [x] **Integrated Dev Server**
  - [x] Built-in dev server with watch mode
  - [x] Automatic compilation on file changes
  - [x] Hot module replacement (HMR) support
  - [x] Configurable port and host binding
  - [x] Browser auto-open on startup

- [ ] **Single Executable Distribution**
  - [ ] Bundle compiler with CLI for standalone use
  - [ ] Cross-platform builds (Linux, macOS, Windows)
  - [ ] No external dependencies required

### Milestone 3: Mobile (Future)

- React Native compiler
- Platform-aware components
- Shared NWL files

### Milestone 4: Power Users (Future)

- Custom components
- Plugin system
- JS/React escape hatches
- Performance optimizations
- UI Component Libraries integration

### Milestone 5: AI-Native (Future)

- AI generates NWL
- Visual ↔ NWL sync
- Refactoring via intent
- Design tokens from Figma
- Custom trained AI assistant for NWL

### Milestone 6: Enterprise (Future)

- Multi-tenant architecture
- SSO/SAML integration
- Advanced permission systems
- Audit logging
- Enterprise support SLAs

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
- [x] Modal
- [x] Table
- [x] Tabs
- [x] Breadcrumb
- [x] Pagination
- [x] Accordion
- [x] Alert
- [x] Badge
- [x] Tag
- [x] Spinner
- [x] Progress
- [x] Toggle
- [x] DateInput
- [x] TimeInput
- [x] DateTimeInput
- [x] ColorPicker
- [x] FileUpload
- [x] SearchInput
- [x] Counter
- [x] CopyButton
- [x] Avatar
- [x] ChipInput
- [x] Nav (navigation bar)
- [x] Menu (with hamburger mobile menu)

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

### Phase 7: Advanced Features - PLANNED

#### 7.1 Forms & Validation
- [ ] Form validation YAML schema
  ```yaml
  form:
    onSubmit: handleSubmit
    validation:
      email:
        - required: true
        - pattern: "^[^\s@]+@[^\s@]+\.[^\s@]+$"
        - message: "Please enter a valid email"
      password:
        - required: true
        - minLength: 8
        - message: "Password must be at least 8 characters"
    ```
- [ ] Input validation messages
- [ ] Error state rendering
- [ ] Server-side form submission integration

#### 7.2 External CSS & Stylesheets
- [ ] External user-defined CSS file support
- [ ] External YAML stylesheet for custom classes
- [ ] Custom class declaration syntax in YAML
  ```yaml
  styles:
    - file: "styles/custom.yaml"
  component:
    class: my-custom-class
  ```

#### 7.3 Reusable Components
- [ ] Component import syntax
  ```yaml
  components:
    - file: "components/my-form.yaml"
      as: ContactForm
  ```
- [ ] Component usage
  ```yaml
  - ContactForm:
      prop: value
  ```
- [ ] Component prop passing
- [ ] Component nesting

#### 7.4 Navigation Components
- [x] Nav component
  ```yaml
  nav:
    links:
      - label: Home
        href: /
      - label: About
        href: /about
    style: [fixed, top-0]
  ```
- [x] Menu component with mobile hamburger
  ```yaml
  menu:
    items:
      - label: Services
        href: /services
      - label: Projects
        href: /projects
      mobileBreakpoint: 768
      hamburger: true
    ```

#### 7.5 Forms Captcha
- [ ] Cloudflare Turnstile integration
  ```yaml
  form:
    onSubmit: handleSubmit
    captcha:
      provider: cloudflare
      siteKey: "${CLOUDFLARE_SITE_KEY}"
      theme: auto
  ```
- [ ] Google reCAPTCHA v2/v3 integration
  ```yaml
  form:
    captcha:
      provider: recaptcha
      siteKey: "${RECAPTCHA_SITE_KEY}"
      version: v3
      action: submit_form
  ```
- [ ] hCaptcha integration
  ```yaml
  form:
    captcha:
      provider: hcaptcha
      siteKey: "${HCAPTCHA_SITE_KEY}"
      theme: light
  ```
- [ ] Captcha verification on server-side
- [ ] Captcha error handling and fallback
- [ ] Configurable captcha expiry and refresh

#### 7.6 Additional React Components
- [ ] URL/Link with external navigation
- [ ] Email mailto component
- [ ] Image with lazy loading
- [ ] Video player
- [ ] Audio player
- [ ] Iframe
- [ ] Canvas
- [ ] SVG components
- [ ] Portal
- [ ] Suspense
- [ ] Memo

### Phase 8: CLI & DevOps - PLANNED

#### 8.1 Project Initialization
- [ ] Interactive `nwl init` command
- [ ] Project template selection (blank, demo, agency, e-commerce)
- [ ] Custom project name and description
- [ ] Auto-install dependencies
- [ ] Git repository initialization

#### 8.2 Configurable Build Pipeline
- [ ] User-defined project location
- [ ] Selective build steps:
  - [ ] `nwl build compiler` - Build only the NWL compiler
  - [ ] `nwl compile` - Compile YAML to React
  - [ ] `nwl build project` - Build full project
- [ ] Parallel/sequential build options
- [ ] Build caching configuration

#### 8.3 Development Server
- [ ] Integrated dev server (`nwl dev`)
- [ ] Live reload on YAML changes
- [ ] Hot module replacement
- [ ] Configurable port and host
- [ ] Proxy configuration for API calls

#### 8.4 Deployment Support
- [ ] Node.js (npm) deployment
  ```bash
  nwl deploy --target nodejs
  ```
  Generates:
  - package.json
  - Dockerfile
  - docker-compose.yml (optional)
- [ ] Docker Compose deployment
  ```bash
  nwl deploy --target docker
  ```
  Generates:
  - Dockerfile
  - docker-compose.yml
  - nginx.conf (optional)
- [ ] Vercel deployment
  ```bash
  nwl deploy --target vercel
  ```
- [ ] Netlify deployment
  ```bash
  nwl deploy --target netlify
  ```
- [ ] Environment configuration management
  - .env files per environment (dev, staging, prod)
  - Environment variable validation
- [ ] Database schema initialization
  - SQLite support
  - PostgreSQL support
  - MySQL support
- [ ] CI/CD pipeline generation
  - GitHub Actions
  - GitLab CI
  - CircleCI

#### 8.5 CLI Commands Reference

```bash
# Project management
nwl init                  # Initialize new project
nwl create <name>         # Create new page/component
nwl import <file>         # Import external component

# Build commands
nwl build compiler        # Build NWL compiler
nwl compile [input]       # Compile YAML to React
nwl build project         # Build full project

# Development
nwl dev                   # Start dev server
nwl watch                 # Watch and rebuild on changes

# Deployment
nwl deploy                # Deploy to configured target
nwl deploy --target nodejs   # Deploy to Node.js
nwl deploy --target docker   # Deploy with Docker
nwl deploy --target vercel   # Deploy to Vercel
nwl deploy --target netlify  # Deploy to Netlify

# Utilities
nwl validate              # Validate YAML syntax
nwl lint                  # Lint NWL files
nwl info                  # Show project info
nwl doctor                # Check system requirements
```

---

### Phase 9: UI Component Libraries - FUTURE

#### 9.1 Radix UI Integration
- [ ] Radix primitives wrapper generation
  ```yaml
  component:
    use: dialog
    open: isOpen
    onOpenChange: setOpen
  ```
- [ ] Accessible component primitives
- [ ] Keyboard navigation support
- [ ] Screen reader compatibility
- [ ] Focus management

#### 9.2 Styled Components Support
- [ ] CSS-in-JS integration
- [ ] Theme provider configuration
  ```yaml
  theme:
    provider: styled-components
    theme: "theme.yaml"
  ```
- [ ] Dynamic styling based on props
- [ ] Server-side rendering support

#### 9.3 Tailwind + Headless UI
- [ ] Headless UI component wrappers
- [ ] Custom Tailwind plugin generation
- [ ] Design token integration
  ```yaml
  tokens:
    - file: "design-tokens.yaml"
  component:
    className: "btn-primary"
  ```

#### 9.4 Component Library Registry
- [ ] Multiple library support (Radix, Chakra, Mantine)
- [ ] Library selection in project init
- [ ] Auto-generated component mapping
- [ ] Cross-library theme compatibility

#### 9.5 Consistent Design System
- [ ] Design token extraction from Figma
- [ ] Typography system
  ```yaml
  typography:
    headings:
      font: "Inter"
      weights: [400, 500, 600, 700]
    body:
      font: "Inter"
      weights: [400, 500]
  ```
- [ ] Color system
- [ ] Spacing system
- [ ] Breakpoint system

---

### Phase 10: AI-Native Development - FUTURE

#### 10.1 Custom Trained NWL LLM
- [ ] Fine-tuned model for NWL syntax
- [ ] Context-aware code generation
- [ ] Error suggestion and fixing
- [ ] Multi-language support

#### 10.2 AI Assistant Integration
- [ ] Chat interface for NWL help
  ```bash
  nwl ai "Create a contact form with email validation"
  ```
- [ ] Inline suggestions in IDE
- [ ] Voice commands support
- [ ] Natural language to NWL conversion

#### 10.3 Automated Development
- [ ] Page generation from description
  ```
  > "Create a pricing page with 3 tiers"
  ```
- [ ] Component suggestion engine
- [ ] A/B test variant generation
- [ ] Responsive layout suggestions

#### 10.4 Smart Refactoring
- [ ] Code modernization
- [ ] Performance optimization suggestions
- [ ] Accessibility improvements
- [ ] Security audit and fixes

#### 10.5 Visual ↔ NWL Sync
- [ ] Figma import/export
- [ ] Drag-and-drop interface builder
- [ ] Real-time bidirectional sync
- [ ] Version control integration

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

### High Priority - NEXT

4. **Forms & Validation**
    - [ ] Add form validation YAML schema syntax
    - [ ] Implement validation rules (required, pattern, minLength, maxLength)
    - [ ] Generate validation error messages
    - [ ] Implement server-side form submission
    - [ ] Add onValidationError handler support

5. **Forms Captcha**
    - [ ] Implement Cloudflare Turnstile integration
    - [ ] Implement Google reCAPTCHA v2/v3 support
    - [ ] Implement hCaptcha support
    - [ ] Add captcha YAML configuration syntax
    - [ ] Implement server-side captcha verification
    - [ ] Add captcha error handling and retry logic

6. **Type Testing & Unit Tests**
    - [ ] **Type Testing**
      - [ ] Add TypeScript type checking for generated React components
      - [ ] Implement type inference tests for component props
      - [ ] Add generic type tests for data binding
      - [ ] Validate state type consistency
    - [ ] **Unit Tests**
      - [ ] Add Rust unit tests for YAML parsing (shared crate)
      - [ ] Add Rust unit tests for codegen (compiler crate)
      - [ ] Add validation rule tests
      - [ ] Add integration tests for complete page compilation
      - [ ] Add vitest for React component testing
      - [ ] Add snapshot tests for generated code
      - [ ] Add prop validation tests

7. **Navigation Components**
    - [ ] Implement `nav:` component for navigation bars
    - [ ] Implement `menu:` component with hamburger menu for mobile
    - [ ] Add mobile breakpoint configuration
    - [ ] Support dropdown menus

### Medium Priority - CLI & DevOps

7. **Project Initialization**
   - [ ] Implement interactive `nwl init` command
   - [ ] Add project template selection
   - [ ] Auto-install dependencies option
   - [ ] Git repository initialization

8. **Build Pipeline Configuration**
   - [ ] Add configurable project location
   - [ ] Implement selective build commands
   - [ ] Add parallel build support
   - [ ] Implement build caching

9. **Development Server**
   - [ ] Integrate dev server into CLI
   - [ ] Implement live reload on YAML changes
   - [ ] Add hot module replacement
   - [ ] Configurable port and host options

10. **Deployment System**
    - [ ] Implement Node.js deployment target
    - [ ] Implement Docker Compose deployment target
    - [ ] Add Vercel deployment support
    - [ ] Add Netlify deployment support
    - [ ] Environment configuration management
    - [ ] Database schema initialization
    - [ ] CI/CD pipeline generation

### Medium Priority

11. **External CSS & Stylesheets**
    - [ ] Add external CSS file import syntax
    - [ ] Add external YAML stylesheet for custom classes
    - [ ] Implement `class:` property for custom classes
    - [ ] Support class composition

12. **Reusable Components**
    - [ ] Implement component import syntax (`import:`)
    - [ ] Add component alias support (`as:`)
    - [ ] Support component props passing
    - [ ] Implement component children nesting

13. **Additional React Components**
    - [ ] URL/Link with external navigation
    - [ ] Email mailto component
    - [ ] Image with lazy loading
    - [ ] Video/Audio players
    - [ ] Iframe, Canvas, SVG

### Lower Priority

14. **Data Binding**
    - [x] Dot-notation in text content (`{name}`)
    - [ ] Support data context in lists (future)
    - [ ] Add data prop to components (future)

15. **Routing Parameters**
    - [ ] Support `/project/:id` syntax
    - [ ] Generate dynamic routes
    - [ ] Access params in components

### Future Features

16. **UI Component Libraries**
    - [ ] Radix UI integration
    - [ ] Styled components support
    - [ ] Headless UI + Tailwind integration
    - [ ] Component library registry
    - [ ] Design system tokens (typography, colors, spacing)

17. **AI-Native Development**
    - [ ] Custom trained NWL LLM
    - [ ] AI assistant integration
    - [ ] Automated page generation from description
    - [ ] Smart refactoring suggestions
    - [ ] Visual ↔ NWL sync (Figma import/export)

---

## 8. Available Routes

The demo project includes these routes:

| Route | Description |
|-------|-------------|
| `/` | Home page |
| `/playground` | All interactive components demo |

Try the `/playground` route to see:
- Form components (input, checkbox, slider, select, radio, textarea, form)
- Date/Time inputs (date, time, datetime)
- Selection components (color picker, file upload, progress, toggle)
- Navigation (tabs, breadcrumb, pagination, accordion)
- Modal & dialogs
- Indicators (badges, tags, spinner)
- Input components (search, counter, copy button, avatar, chip input)

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
  validation:
    email:
      - required: true
      - pattern: "^[^\s@]+@[^\s@]+\.[^\s@]+$"
      - message: "Please enter a valid email"
    password:
      - required: true
      - minLength: 8
      - message: "Password must be at least 8 characters"

### Navigation Components

```yaml
nav:
  links:
    - label: Home
      href: /
    - label: Services
      href: /services
    - label: Projects
      href: /projects
  style: [fixed, top-0, w-full, bg-black]

menu:
  items:
    - label: Services
      href: /services
    - label: Projects
      href: /projects
    - label: Careers
      href: /careers
  mobileBreakpoint: 768
  hamburger: true
  mobileStyle: [fixed, top-0, right-0, w-64, h-full, bg-gray-900]
```

### Reusable Components

```yaml
import:
  - file: "components/contact-form.yaml"
    as: ContactForm

page:
  children:
    - ContactForm:
        onSubmit: handleContact
        title: "Get in Touch"
```

### External Styles

```yaml
styles:
  - file: "styles/theme.yaml"
  - file: "styles/components.yaml"

component:
  class: primary-button
  className: "custom-class {component.class}"
```

### URL & Email

```yaml
url:
  href: "https://example.com"
  content: "Visit our website"
  target: _blank

email:
  address: "info@example.com"
  subject: "Inquiry"
  content: "Contact us"
```

### UI Component Libraries (Future)

```yaml
# Radix UI Dialog
component:
  use: dialog
  open: isModalOpen
  onOpenChange: setModalOpen
  children:
    - button:
        onClick: setModalOpen(true)
        content: "Open Dialog"
    - dialog:
        title: "Dialog Title"
        description: "This is a dialog description"
        children:
          - text:
              content: "Dialog content goes here..."

# Design Tokens
tokens:
  - file: "design-tokens.yaml"

typography:
  headings:
    font: "Inter"
    weights: [400, 500, 600, 700]
  body:
    font: "Inter"
    weights: [400, 500]

colorSystem:
  primary: "#0082FF"
  secondary: "#131212"
  accent: "#FFFFFF"
```

### AI Assistant (Future)

```bash
# Generate a page from description
nwl ai "Create a pricing page with 3 tiers: Free, Pro, Enterprise"

# Get help with syntax
nwl ai help "How do I add form validation?"

# Refactor existing code
nwl ai refactor "Make this page responsive"

# Generate components
nwl ai generate "Create a contact form with name, email, and message fields"
```

All JavaScript is automatically generated by the compiler.
