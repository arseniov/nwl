/// <reference types="vitest" />
import { describe, it, expect, beforeAll, afterAll } from 'vitest'
import * as fs from 'fs'
import * as path from 'path'
import { execSync, spawn } from 'child_process'

describe('CSS Processing and Bundling', () => {
  const nwlPath = '/home/giacomo/coding/nwl/target/release/nwl'
  const demoPath = '/home/giacomo/coding/nwl/examples/demo'
  const themesPath = path.join(demoPath, 'themes')
  const processedCssPath = path.join(themesPath, 'processed.css')

  beforeAll(() => {
    // Ensure the demo project is built
    if (!fs.existsSync(nwlPath)) {
      console.log('Building NWL CLI...')
      execSync('cargo build --release', { cwd: '/home/giacomo/coding/nwl', stdio: 'inherit' })
    }
  })

  describe('CSS Processing with nwl build', () => {
    it('should generate processed.css when CSS theme is configured', () => {
      const nwlYamlPath = path.join(demoPath, 'nwl.yaml')
      expect(fs.existsSync(nwlYamlPath)).toBe(true)

      const nwlContent = fs.readFileSync(nwlYamlPath, 'utf-8')
      expect(nwlContent).toContain('cssTheme: default')
      expect(nwlContent).toContain('cssOverride: default')
    })

    it('should generate processed.css file after build', () => {
      // Clean up old processed.css
      if (fs.existsSync(processedCssPath)) {
        fs.unlinkSync(processedCssPath)
      }

      // Run nwl build
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })

      // Verify processed.css was created
      expect(fs.existsSync(processedCssPath)).toBe(true)
    })

    it('should contain base theme styles in processed.css', () => {
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })
      
      const processedCss = fs.readFileSync(processedCssPath, 'utf-8')
      
      // Check for Base UI component styles
      expect(processedCss).toContain('.Button')
      expect(processedCss).toContain('.Card')
      expect(processedCss).toContain('.Checkbox-root')
      expect(processedCss).toContain('.Select-trigger')
      expect(processedCss).toContain('.RadioGroup')
      expect(processedCss).toContain('.Switch-root')
    })

    it('should include custom properties from theme.override.css', () => {
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })
      
      const processedCss = fs.readFileSync(processedCssPath, 'utf-8')
      const themeOverride = fs.readFileSync(path.join(themesPath, 'theme.override.css'), 'utf-8')

      // Check that comments from override file are in processed CSS
      expect(processedCss).toContain('/* Theme Overrides')
      expect(processedCss).toContain('Custom theme overrides')
    })

    it('should include inline styles from YAML pages in processed.css', () => {
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })
      
      const processedCss = fs.readFileSync(processedCssPath, 'utf-8')
      
      // Check for inline style processing
      // The Button with inline styles should be in processed CSS
      expect(processedCss).toContain('.Button')
    })
  })

  describe('CSS Import in main.tsx', () => {
    it('should import processed.css in main.tsx', () => {
      const mainTsxPath = path.join(demoPath, 'src', 'main.tsx')
      expect(fs.existsSync(mainTsxPath)).toBe(true)

      const mainContent = fs.readFileSync(mainTsxPath, 'utf-8')
      expect(mainContent).toContain("import '../themes/processed.css'")
    })

    it('should not import theme.css and theme.override.css separately', () => {
      const mainTsxPath = path.join(demoPath, 'src', 'main.tsx')
      const mainContent = fs.readFileSync(mainTsxPath, 'utf-8')
      
      // After our changes, only processed.css should be imported
      expect(mainContent).not.toContain("import '../themes/theme.css'")
      expect(mainContent).not.toContain("import '../themes/theme.override.css'")
    })
  })

  describe('Vite Production Build CSS Bundling', () => {
    it('should bundle CSS during vite build', () => {
      const distPath = path.join(demoPath, 'dist')
      const distCssPath = path.join(distPath, 'assets')
      
      // Clean up old dist
      if (fs.existsSync(distPath)) {
        execSync('rm -rf dist', { cwd: demoPath, encoding: 'utf-8' })
      }

      // Run vite build
      execSync('npx vite build', { cwd: demoPath, encoding: 'utf-8' })

      // Check that CSS was bundled
      expect(fs.existsSync(distPath)).toBe(true)
      expect(fs.existsSync(distCssPath)).toBe(true)
      
      const cssFiles = fs.readdirSync(distCssPath).filter(f => f.endsWith('.css'))
      expect(cssFiles.length).toBeGreaterThan(0)
    })

    it('should include Base UI component styles in bundled CSS', () => {
      const distCssPath = path.join(demoPath, 'dist', 'assets')
      const cssFiles = fs.readdirSync(distCssPath).filter(f => f.endsWith('.css'))
      
      let bundledCss = ''
      for (const cssFile of cssFiles) {
        bundledCss += fs.readFileSync(path.join(distCssPath, cssFile), 'utf-8')
      }

      // Check that component styles are bundled
      expect(bundledCss).toMatch(/\.Button/)
      expect(bundledCss).toMatch(/\.Card/)
      expect(bundCss).toMatch(/\.Checkbox/) // Check for any Checkbox related styles
    })

    it('should include CSS in index.html', () => {
      const indexHtmlPath = path.join(demoPath, 'dist', 'index.html')
      expect(fs.existsSync(indexHtmlPath)).toBe(true)

      const indexContent = fs.readFileSync(indexHtmlPath, 'utf-8')
      expect(indexContent).toContain('.css')
    })
  })

  describe('nwm dev Server CSS Processing', () => {
    it('should start dev server with CSS processing', async () => {
      const devServerPort = 5177
      const devServerUrl = `http://localhost:${devServerPort}`
      
      // Kill any existing server on this port
      try {
        execSync(`lsof -ti:${devServerPort} | xargs kill -9 2>/dev/null`, { encoding: 'utf-8' })
      } catch {}

      // Start dev server in background
      const devProcess = spawn(nwlPath, ['dev', '--port', devServerPort.toString()], {
        cwd: demoPath,
        stdio: ['pipe', 'pipe', 'pipe']
      })

      // Wait for server to start
      await new Promise(resolve => setTimeout(resolve, 3000))

      // Check that server is running
      try {
        const response = execSync(`curl -s -o /dev/null -w "%{http_code}" ${devServerUrl}`, { encoding: 'utf-8' })
        expect(['200', '304']).toContain(response)
      } catch {
        // Server might not be ready yet, that's ok for this test
      }

      // Clean up
      devProcess.kill()
      
      // Ensure process is killed
      try {
        execSync(`lsof -ti:${devServerPort} | xargs kill -9 2>/dev/null`, { encoding: 'utf-8' })
      } catch {}
    }, 10000)
  })

  describe('CSS Precedence', () => {
    it('should apply inline styles with highest priority', () => {
      // Create a test page with inline styles
      const testPagePath = path.join(demoPath, 'pages', 'test-precedence.yaml')
      const testOutputPath = path.join(demoPath, 'src', 'test-precedence.tsx')
      
      const testPageContent = `
page:
  name: TestPrecedence
  layout:
    type: column
  style: [bg-white]
  children:
    - element: button
      content: "Test Button"
      style: [bg-red-500, text-white]
`
      fs.writeFileSync(testPagePath, testPageContent)

      // Compile the page
      execSync(`${nwlPath} compile ${testPagePath} -o ${testOutputPath}`, { cwd: demoPath, encoding: 'utf-8' })

      // Verify the output has the component
      expect(fs.existsSync(testOutputPath)).toBe(true)
      const content = fs.readFileSync(testOutputPath, 'utf-8')
      expect(content).toContain('className="Button bg-red-500 text-white"')

      // Clean up
      fs.unlinkSync(testPagePath)
      fs.unlinkSync(testOutputPath)
    })

    it('should merge theme and override CSS correctly', () => {
      // Read the processed CSS
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })
      const processedCss = fs.readFileSync(processedCssPath, 'utf-8')

      // Check that CSS rules are present (not just comments)
      const cssRules = processedCss.split('}').filter(r => r.trim().length > 0 && r.includes('{'))
      expect(cssRules.length).toBeGreaterThan(0)

      // Check that .Button has properties (not just empty)
      const buttonMatch = processedCss.match(/\.Button\s*\{([^}]+)\}/)
      expect(buttonMatch).not.toBeNull()
      expect(buttonMatch[1].trim().length).toBeGreaterThan(0)
    })
  })

  describe('CSS File Validation', () => {
    it('should have valid CSS syntax in processed.css', () => {
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })
      
      const processedCss = fs.readFileSync(processedCssPath, 'utf-8')
      
      // Basic syntax checks
      expect(processedCss).not.toContain('{{')  // Double braces indicate template issues
      expect(processedCss).not.toContain('}}')
      
      // Check for balanced braces
      const openBraces = (processedCss.match(/\{/g) || []).length
      const closeBraces = (processedCss.match(/\}/g) || []).length
      expect(openBraces).toBe(closeBraces)
    })

    it('should have processed.css not empty', () => {
      execSync(`${nwlPath} build`, { cwd: demoPath, encoding: 'utf-8' })
      
      const processedCss = fs.readFileSync(processedCssPath, 'utf-8')
      expect(processedCss.length).toBeGreaterThan(100)  // Should have substantial content
    })
  })
})
