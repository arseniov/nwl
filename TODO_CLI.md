# NWL CLI Development TODO

This document tracks the tasks for completing the NWL CLI with full functionality.

## ✅ Completed Features

### Project Scaffolding

- [x] Implement `nwl new` command with location validation
- [x] Check if project directory exists/is empty before creation
- [x] Display warning and exit if location is not suitable
- [x] Create project structure from templates
- [x] Support different project templates (blank, demo)

### Compilation Pipeline

- [x] Implement full `nwl build` command
- [x] Read and parse `nwl.yaml` project configuration
- [x] Compile all NWL files in `pages/` directory
- [x] Generate React components with proper imports
- [x] Create/update router configuration
- [x] Implement watch mode for auto-recompilation

### Integrated Dev Server

- [x] Add `nwl dev` command
- [x] Start Vite dev server automatically
- [x] Watch NWL files for changes (via build --watch)
- [x] Support `--port` flag for custom port

## 🚀 Next Steps

### GitHub Template Distribution

- [ ] Publish templates to GitHub repository
- [ ] Configure GitHub Actions to build template zip files
- [ ] Implement GitHub API fetch in CLI (currently uses local fallback)
- [ ] Add template versioning support
- [ ] Cache templates locally for offline use

### CLI Improvements

- [ ] Auto-detect package manager (npm/pnpm/yarn)
- [ ] Add `--yes` flag to skip prompts
- [ ] Support custom template URLs
- [ ] Add template list command
- [ ] Add `nwl init` to initialize in existing project

### Documentation

- [ ] Document template structure
- [ ] Add template contribution guide
- [ ] Create template showcase in docs

## Example Usage

```bash
# Create new project
nwl new my-app

# Create project in specific location
nwl new my-app --location ~/projects

# Use demo template (includes all components)
nwl new my-app --template demo

# Build project
cd my-app
nwl build

# Build with watch mode
nwl build --watch

# Run with dev server (auto-watch + hot reload)
nwl dev

# Run on custom port
nwl dev --port 3000
```
