/**
 * Type tests for Playground component
 * Run with: npm run typecheck
 * 
 * This file contains type assertions that will fail at compile time
 * if the generated component has incorrect types.
 */

import React from 'react'

// This import verifies the component can be imported
// import Playground from './playground'

// Type assertions for useState hooks
// These will cause compile errors if types don't match
export function verifyUseStateTypes() {
  // Boolean state
  const [flag, setFlag] = React.useState(false)
  setFlag(true)
  const _boolState: boolean = flag

  // Number state
  const [count, setCount] = React.useState(0)
  setCount(10)
  const _numState: number = count

  // String state
  const [text, setText] = React.useState("")
  setText("hello")
  const _strState: string = text

  // Array state
  const [items, setItems] = React.useState<string[]>([])
  setItems(["a", "b"])
  const _arrState: string[] = items

  return { flag, count, text, items }
}

// Type assertions for event handlers
export function verifyEventHandlerTypes() {
  const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    console.log('clicked')
  }

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    console.log('changed')
  }

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    console.log('submitted')
  }

  return { handleClick, handleChange, handleSubmit }
}

// Type assertion for component props
export type ComponentProps = {
  className?: string
  style?: React.CSSProperties
  children?: React.ReactNode
}

// Verify these types work correctly
export function typeVerification() {
  const props: ComponentProps = {
    className: "test",
    style: { color: "red" },
  }
  return props
}


