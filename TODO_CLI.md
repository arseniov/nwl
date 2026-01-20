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
- [x] Implement watch mode for auto-recompilation (`nwl build --watch`)

### Integrated Dev Server

- [x] Add `nwl dev` command
- [x] Start Vite dev server automatically
- [x] Watch NWL files for changes (`nwl dev --watch`)
- [x] Support `--port` flag for custom port
- [x] Support `--host` flag for custom host binding (e.g., 0.0.0.0)

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
- [ ] Implement incremental builds for faster compilation
- [ ] Add `--output` flag to configure output directory

### Single Executable Distribution

- [ ] Bundle Rust compiler with CLI binary for cross-platform distribution
- [ ] Create build scripts for Linux, macOS, Windows
- [ ] Generate checksums for verification

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
nwl dev --watch

# Run on custom port and host
nwl dev --port 3000 --host 0.0.0.0
```
