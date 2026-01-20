/// <reference types="vitest" />
import { describe, it, expect } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import Playground from './playground'

describe('Playground Components', () => {
  const renderPlayground = () => {
    return render(
      <BrowserRouter>
        <Playground />
      </BrowserRouter>
    )
  }

  describe('Heading Component', () => {
    it('renders heading text correctly', () => {
      renderPlayground()
      expect(screen.getByText('NWL Component Playground')).toBeInTheDocument()
    })
  })

  describe('Form Components', () => {
    it('renders form with email and password inputs', () => {
      renderPlayground()
      expect(screen.getByPlaceholderText('Email')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('Password')).toBeInTheDocument()
    })

    it('renders submit button', () => {
      renderPlayground()
      expect(screen.getByRole('button', { name: /submit/i })).toBeInTheDocument()
    })
  })

  describe('State Management', () => {
    it('renders checkbox with initial state', () => {
      renderPlayground()
      const checkbox = screen.getByLabelText(/I agree to the terms/i)
      expect(checkbox).toBeInTheDocument()
      expect(checkbox).not.toBeChecked()
    })

    it('checkbox toggles state on click', () => {
      renderPlayground()
      const checkbox = screen.getByLabelText(/I agree to the terms/i)
      expect(checkbox).not.toBeChecked()
      fireEvent.click(checkbox)
      expect(checkbox).toBeChecked()
    })
  })

  describe('Navigation', () => {
    it('renders navigation with links', () => {
      renderPlayground()
      expect(screen.getByRole('link', { name: /home/i })).toBeInTheDocument()
      expect(screen.getByRole('link', { name: /playground/i })).toBeInTheDocument()
      expect(screen.getByRole('link', { name: /docs/i })).toBeInTheDocument()
    })
  })

  describe('Selection Components', () => {
    it('renders select dropdown', () => {
      renderPlayground()
      expect(screen.getByRole('combobox')).toBeInTheDocument()
    })

    it('renders slider input', () => {
      renderPlayground()
      const slider = screen.getByRole('slider')
      expect(slider).toBeInTheDocument()
    })
  })
})
