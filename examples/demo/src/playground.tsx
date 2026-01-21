import React, { useState } from 'react';
import {
  Button,
  Checkbox,
  Select,
  Radio,
  RadioGroup,
  Switch,
  Separator,
  NumberField,
  Dialog,
  Menu,
  Accordion,
  Form,
  Field,
  Fieldset,
  Tooltip,
  Popover,
} from '@base-ui/react';

export default function Playground() {
  const [agreedToTerms, setAgreedToTerms] = useState(false), [country, setCountry] = useState(""), [bio, setBio] = useState(""), [formStatus, setFormStatus] = useState(""), [email, setEmail] = useState(""), [password, setPassword] = useState(""), [searchQuery, setSearchQuery] = useState(""), [counterValue, setCounterValue] = useState(5), [enableFeature, setEnableFeature] = useState(true), [dialogOpen, setDialogOpen] = useState(false);
  return (
    <>
      <div>
            <nav className=" flex items-center justify-between px-6 py-4 sticky top-0 z-50 bg-black"><a href="/" className="text-xl font-bold text-white">NWL</a><div className="flex items-center gap-6">    <a href="/" className="text-sm font-medium transition-colors text-white hover:text-blue-400">Home</a>
    <a href="/playground" className="text-sm font-medium transition-colors text-blue-400">Playground</a>
    <a href="/docs" className="text-sm font-medium transition-colors text-white hover:text-blue-400">Docs</a>
</div></nav>
            <div className="Card">
            <h1>NWL Component Playground</h1>
            <p>Explore all interactive components available in NWL with BaseUI</p>
            <Separator />
            <h1>BaseUI Button Component</h1>
            <div className="flex flex-row gap-4 flex-wrap">
                <Button className="Button">Primary Button</Button>
                <Button className="Button">Secondary</Button>
                <Button className="Button">Outline</Button>
      </div>

            <Separator />
            <h1>BaseUI Form Components</h1>
            <div className="flex flex-col gap-4">
                <div className="Card">
                    <h1>1. Checkbox</h1>
                    <label className="flex items-center gap-2 cursor-pointer">          <Checkbox.Root className="Checkbox-root" checked={agreedToTerms}  onCheckedChange={(checked) => setAgreedToTerms(checked)}>            <Checkbox.Indicator className="Checkbox-indicator" />          </Checkbox.Root><span>I agree to the terms and conditions</span></label>
                    <p>{agreedToTerms ? 'Agreed!' : 'Not agreed'}</p>
        </div>

                <div className="Card">
                    <h1>2. NumberField (Counter)</h1>
                    <NumberField.Root className="NumberField-root" value={counterValue} min={0} max={99} onValueChange={(value) => setCounterValue(value)}>
          <NumberField.Decrement className="NumberField-decrement">
              <span>-</span>
            </NumberField.Decrement>
            <NumberField.Input className="NumberField-input" />
            <NumberField.Increment className="NumberField-increment">
              <span>+</span>
            </NumberField.Increment>
          </NumberField.Root>
                    <p>Value: {counterValue}</p>
        </div>

                <div className="Card">
                    <h1>3. Toggle Switch</h1>
                    <label className="inline-flex items-center cursor-pointer gap-3"><span className="font-medium">Enable feature</span><Switch.Root className="Switch-root" id="toggle" checked={enableFeature} onCheckedChange={(checked) => setEnableFeature(checked)}>            <Switch.Thumb className="Switch-thumb" />          </Switch.Root></label>
                    <p>{enableFeature ? 'Enabled' : 'Disabled'}</p>
        </div>

      </div>

            <Separator />
            <h1>BaseUI Select & RadioGroup</h1>
            <div className="Card">
                <h1>4. Select Dropdown</h1>
                <Select.Root value={country} onValueChange={(value) => setCountry(value)}>
          <Select.Trigger className="Select-trigger">
            <Select.Value />
            <Select.Icon />
          </Select.Trigger>
          <Select.Portal>
            <Select.Positioner sideOffset={8}>
              <Select.Popup className="Select-popup">
                <Select.Item className="Select-item" value="">
                  <Select.ItemText>Choose country</Select.ItemText>
                </Select.Item>
                <Select.Item className="Select-item" value="us">
                  <Select.ItemText>United States</Select.ItemText>
                </Select.Item>
                <Select.Item className="Select-item" value="uk">
                  <Select.ItemText>United Kingdom</Select.ItemText>
                </Select.Item>
                <Select.Item className="Select-item" value="ca">
                  <Select.ItemText>Canada</Select.ItemText>
                </Select.Item>
              </Select.Popup>
            </Select.Positioner>
          </Select.Portal>
        </Select.Root>
                <p>Selected: {country || 'None'}</p>
      </div>

            <div className="Card">
                <h1>5. Radio Group</h1>
                <RadioGroup className="RadioGroup" defaultValue="low">
          <label className="flex items-center gap-2 cursor-pointer py-1">
            <Radio.Root className="Radio-root" value="low">
              <Radio.Indicator className="Radio-indicator" />
            </Radio.Root>
            <span>Low Priority</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer py-1">
            <Radio.Root className="Radio-root" value="medium">
              <Radio.Indicator className="Radio-indicator" />
            </Radio.Root>
            <span>Medium Priority</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer py-1">
            <Radio.Root className="Radio-root" value="high">
              <Radio.Indicator className="Radio-indicator" />
            </Radio.Root>
            <span>High Priority</span>
          </label>
        </RadioGroup>

      </div>

            <Separator />
            <h1>Form Components</h1>
            <div className="flex flex-col gap-4">
                <div className="Card">
                    <h1>Text Input</h1>
                    <input placeholder="Enter your name..." />
        </div>

                <div className="Card">
                    <h1>Textarea</h1>
                    <textarea placeholder="Your bio..." rows={3} value={bio} onChange={(e) => setBio(e.target.value)} />
                    <p>{bio.length} characters</p>
        </div>

                <div className="Card">
                    <h1>Form with Validation</h1>
                    <Form className="Form">
                        <input placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
                        <input placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
                        <Button className="Button">Submit</Button>
          </Form>

                    <p>{formStatus}</p>
        </div>

      </div>

            <Separator />
            <h1>BaseUI Dialog & Menu</h1>
            <div className="flex flex-col gap-4">
                <div className="Card">
                    <h1>6. Dialog</h1>
                    <Button className="Button" onClick={() => setDialogOpen(true)}>Open Dialog</Button>
                    <Dialog.Root open={dialogOpen} onOpenChange={open => setDialogOpen(open)}>
            <Dialog.Portal>
              <Dialog.Backdrop className="Dialog-backdrop" />
              <Dialog.Popup className="Dialog-popup">
                <Dialog.Title className="Dialog-title">Dialog Title</Dialog.Title>
                                    <p>This is the dialog content. It uses Base UI Dialog components with Portal, Backdrop, and Popup.</p>
                                    <div className="flex flex-row gap-2 justify-end mt-4">
                                        <Button className="Button" onClick={() => setDialogOpen(false)}>Cancel</Button>
                                        <Button className="Button" onClick={() => setDialogOpen(false)}>Confirm</Button>
                  </div>

                <Dialog.Close className="Dialog-close">
                  <span className="sr-only">Close</span>
                </Dialog.Close>
              </Dialog.Popup>
            </Dialog.Portal>
          </Dialog.Root>

        </div>

      </div>

            <Separator />
            <h1>BaseUI Layout Components</h1>
            <div className="Card">
                <h1>7. Accordion</h1>
                <Accordion.Root multiple>
          <Accordion.Item value="item-0">
            <Accordion.Header>
              <Accordion.Trigger>
                What is NWL?
              </Accordion.Trigger>
            </Accordion.Header>
              <Accordion.Panel>
                NWL (Natural Web Language) is a domain-specific language that compiles YAML to production-ready React code.
              </Accordion.Panel>
          </Accordion.Item>
          <Accordion.Item value="item-1">
            <Accordion.Header>
              <Accordion.Trigger>
                How does it work?
              </Accordion.Trigger>
            </Accordion.Header>
              <Accordion.Panel>
                The Rust compiler reads YAML files and generates TypeScript React components with Tailwind CSS styling.
              </Accordion.Panel>
          </Accordion.Item>
          <Accordion.Item value="item-2">
            <Accordion.Header>
              <Accordion.Trigger>
                Why Base UI?
              </Accordion.Trigger>
            </Accordion.Header>
              <Accordion.Panel>
                Base UI provides accessible, unstyled components that work perfectly with NWL's Tailwind-first approach.
              </Accordion.Panel>
          </Accordion.Item>
        </Accordion.Root>

      </div>

            <h1>BaseUI Form Components</h1>
            <div className="Card">
                <h1>8. Field & Fieldset</h1>
                <Fieldset.Root className="Fieldset-root">
          <Fieldset.Legend className="Fieldset-legend">Personal Information</Fieldset.Legend>
                    <Field.Root name="fullName" className="Field-root">
            <Field.Label className="Field-label">Full Name</Field.Label>
            <Field.Control placeholder="Enter your name" className="Field-control" />
            <Field.Error className="Field-error" />
          </Field.Root>

                    <Field.Root name="email" className="Field-root">
            <Field.Label className="Field-label">Email Address</Field.Label>
            <Field.Control placeholder="you@example.com" className="Field-control" />
            <Field.Error className="Field-error" />
          </Field.Root>

        </Fieldset.Root>

      </div>

            <h1>BaseUI Tooltip & Popover</h1>
            <div className="flex flex-row gap-4">
                <div className="Card">
                    <h1>9. Tooltip</h1>
                    <Tooltip.Provider>
            <Tooltip.Root>
              <Tooltip.Trigger>
                Hover me
              </Tooltip.Trigger>
              <Tooltip.Portal>
                <Tooltip.Positioner sideOffset={8}>
                  <Tooltip.Popup className="Tooltip-popup">
                    This is a helpful tooltip message!
                  </Tooltip.Popup>
                </Tooltip.Positioner>
              </Tooltip.Portal>
            </Tooltip.Root>
          </Tooltip.Provider>

        </div>

                <div className="Card">
                    <h1>10. Popover</h1>
                    <Popover.Root>
            <Popover.Trigger>
              Click me
            </Popover.Trigger>
            <Popover.Portal>
              <Popover.Positioner sideOffset={8}>
                <Popover.Popup className="Popover-popup">
                  <Popover.Title className="Popover-title">Popover Title</Popover.Title>
                  This is the popover content. It can contain any elements and is positioned relative to the trigger.
                </Popover.Popup>
              </Popover.Positioner>
            </Popover.Portal>
          </Popover.Root>

        </div>

      </div>

            <h1>BaseUI Navigation</h1>
            <div className="Card">
                <h1>11. NavigationMenu</h1>
                <div className="NavigationMenu">
          <nav className="NavigationMenu-nav flex items-center justify-between px-6 py-4 bg-gray-900 border-b border-gray-700">
            <a href="/" className="NavigationMenu-logo">NWL</a>
            <div className="hidden md:flex gap-4 NavigationMenu-nav">
              <a href="/" className="NavigationMenu-link">Home</a>
              <a href="/features" className="NavigationMenu-link">Features</a>
              <a href="/pricing" className="NavigationMenu-link">Pricing</a>
              <a href="/about" className="NavigationMenu-link">About</a>
              <a href="/contact" className="NavigationMenu-link">Contact</a>
            </div>
            <Button className="Button md:hidden">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M4 6h16M4 12h16M4 18h16"></path></svg>
            </Button>
          </nav>
          <Menu.Root>
            <Menu.Portal>
              <Menu.Positioner sideOffset={8} className="z-50">
                <Menu.Popup className="Popover-popup min-w-[200px] p-1 bg-white border rounded-lg shadow-lg">
                  <Menu.Arrow className="fill-white" />
                  <Menu.Item key={0} asChild>
                    <a href="/" className="flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded cursor-pointer">
                      Home
                    </a>
                  </Menu.Item>
                  <Menu.Item key={1} asChild>
                    <a href="/features" className="flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded cursor-pointer">
                      Features
                    </a>
                  </Menu.Item>
                  <Menu.Item key={2} asChild>
                    <a href="/pricing" className="flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded cursor-pointer">
                      Pricing
                    </a>
                  </Menu.Item>
                  <Menu.Item key={3} asChild>
                    <a href="/about" className="flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded cursor-pointer">
                      About
                    </a>
                  </Menu.Item>
                  <Menu.Item key={4} asChild>
                    <a href="/contact" className="flex items-center px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded cursor-pointer">
                      Contact
                    </a>
                  </Menu.Item>
                </Menu.Popup>
              </Menu.Positioner>
            </Menu.Portal>
          </Menu.Root>
        </div>

      </div>

            <Separator />
    </div>

      </div>
    </>
  );
}

