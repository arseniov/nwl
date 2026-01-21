/// <reference types="vitest" />
import { describe, it, expect } from 'vitest'
import * as fs from 'fs'
import * as path from 'path'
import { execSync } from 'child_process'

describe('Compiled JSX Validation', () => {
  const nwlPath = '/home/giacomo/coding/nwl/target/release/nwl'
  const playgroundYamlPath = '/home/giacomo/coding/nwl/examples/demo/pages/playground.yaml'
  const outputPath = '/home/giacomo/coding/nwl/examples/demo/src/playground.tsx'

  it('should compile playground.yaml without errors', () => {
    expect(fs.existsSync(nwlPath)).toBe(true)
    expect(fs.existsSync(playgroundYamlPath)).toBe(true)
    
    const cmd = `${nwlPath} compile ${playgroundYamlPath} -o ${outputPath}`
    execSync(cmd, { encoding: 'utf-8' })
    
    expect(fs.existsSync(outputPath)).toBe(true)
  })

  it('should generate valid JSX syntax', () => {
    const content = fs.readFileSync(outputPath, 'utf-8')
    
    expect(content).toContain('import React')
    expect(content).toContain('@base-ui/react')
    
    // Check that component names are valid (not undefined)
    expect(content).not.toMatch(/<[^>]+undefined[^>]*>/)
    
    // Check for balanced braces in JSX expressions
    const openBraces = (content.match(/\{/g) || []).length
    const closeBraces = (content.match(/\}/g) || []).length
    expect(openBraces).toBe(closeBraces)
  })

  it('should have all Base UI imports from main package', () => {
    const content = fs.readFileSync(outputPath, 'utf-8')

    expect(content).toContain("import {")
    expect(content).toContain("} from '@base-ui/react'")

    const expectedComponents = [
      'Button',
      'Checkbox',
      'Select',
      'Radio',
      'RadioGroup',
      'Switch',
      'Separator',
      'NumberField',
      'Dialog',
      'Menu',
    ]

    for (const component of expectedComponents) {
      expect(content).toContain(component)
    }

    expect(content).not.toMatch(/from '@base-ui\/react\/[^']+'/g)
  })

  it('should generate proper component structure for Base UI components', () => {
    const content = fs.readFileSync(outputPath, 'utf-8')
    
    // Check Checkbox structure
    expect(content).toMatch(/<Checkbox\.Root[^>]*>/)
    expect(content).toMatch(/<Checkbox\.Indicator[^>]*\/?>/)
    
    // Check Switch structure
    expect(content).toMatch(/<Switch\.Root[^>]*>/)
    expect(content).toMatch(/<Switch\.Thumb[^>]*\/?>/)
    
    // Check Separator structure - Separator is not a compound component in BaseUI v1.1.0
    expect(content).toMatch(/<Separator[^>]*\/?>/)
    
    // Check NumberField structure
    expect(content).toMatch(/<NumberField\.Root[^>]*>/)
    expect(content).toMatch(/<NumberField\.Decrement[^>]*>/)
    expect(content).toMatch(/<NumberField\.Input[^>]*\/?>/)
    expect(content).toMatch(/<NumberField\.Increment[^>]*>/)
    
    // Check Select structure
    expect(content).toMatch(/<Select\.Root[^>]*>/)
    expect(content).toMatch(/<Select\.Trigger[^>]*>/)
    expect(content).toMatch(/<Select\.Value[^>]*\/?>/)
    
    // Check RadioGroup structure
    expect(content).toMatch(/<RadioGroup[^>]*>/)
    expect(content).toMatch(/<Radio\.Root[^>]*>/)
    expect(content).toMatch(/<Radio\.Indicator[^>]*\/?>/)
  })
})
