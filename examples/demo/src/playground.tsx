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
      <div className="bg-gray-50 py-8">
            <nav className="w-full flex items-center justify-between px-6 py-4 sticky top-0 z-50 bg-black"><a href="/" className="text-xl font-bold text-white">NWL</a><div className="flex items-center gap-6">    <a href="/" className="text-sm font-medium transition-colors text-white hover:text-blue-400">Home</a>
    <a href="/playground" className="text-sm font-medium transition-colors text-blue-400">Playground</a>
    <a href="/docs" className="text-sm font-medium transition-colors text-white hover:text-blue-400">Docs</a>
</div></nav>
            <div className="max-w-4xl w-full p-8 bg-white rounded-xl shadow-lg">
            <h1 className="text-2xl font-bold text-gray-900 mb-2">NWL Component Playground</h1>
            <p className="text-gray-600 mb-8">Explore all interactive components available in NWL with BaseUI</p>
            <Separator className="w-full h-6 my-4" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Button Component</h1>
            <div className="flex flex-row gap-4 flex-wrap">
                <Button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">Primary Button</Button>
                <Button className="bg-gray-200 text-gray-800 px-4 py-2 rounded hover:bg-gray-300">Secondary</Button>
                <Button className="border border-blue-600 text-blue-600 px-4 py-2 rounded hover:bg-blue-50">Outline</Button>
      </div>

            <Separator className="w-full h-6 my-4" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Form Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">1. Checkbox</h1>
                    <label className="flex items-center gap-2 cursor-pointer">          <Checkbox.Root checked={agreedToTerms}  onCheckedChange={(checked) => setAgreedToTerms(checked)} className="flex size-5 items-center justify-center rounded border data-[checked]:bg-blue-600 data-[checked]:border-blue-600 data-[unchecked]:border-gray-300">            <Checkbox.Indicator className="text-white text-xs" />          </Checkbox.Root><span className="text-gray-700">I agree to the terms and conditions</span></label>
                    <p className="text-sm text-gray-600 mt-2">{agreedToTerms ? 'Agreed!' : 'Not agreed'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">2. NumberField (Counter)</h1>
                    <NumberField.Root className="flex items-center border rounded-lg overflow-hidden" value={counterValue} min={0} max={99} onValueChange={(value) => setCounterValue(value)}>
          <NumberField.Decrement className="px-3 py-2 border-r hover:bg-gray-100 flex items-center justify-center bg-gray-50">
              <span className="text-gray-600 font-medium">-</span>
            </NumberField.Decrement>
            <NumberField.Input className="w-16 text-center border-none focus:outline-none py-2" />
            <NumberField.Increment className="px-3 py-2 border-l hover:bg-gray-100 flex items-center justify-center bg-gray-50">
              <span className="text-gray-600 font-medium">+</span>
            </NumberField.Increment>
          </NumberField.Root>
                    <p className="text-sm text-gray-600 mt-2">Value: {counterValue}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">3. Toggle Switch</h1>
                    <label className="inline-flex items-center cursor-pointer"><label htmlFor="toggle" className="mr-3 text-sm font-medium text-gray-900">Enable feature</label><Switch.Root id="toggle" checked={enableFeature} onCheckedChange={(checked) => setEnableFeature(checked)} className="flex w-11 h-6 bg-gray-200 data-[checked]:bg-green-500 rounded-full transition-colors">            <Switch.Thumb className="block w-5 h-5 bg-white rounded-full shadow-sm transition-transform data-[checked]:translate-x-5 data-[unchecked]:translate-x-0" />          </Switch.Root></label>
                    <p className="text-sm text-gray-600 mt-2">{enableFeature ? 'Enabled' : 'Disabled'}</p>
        </div>

      </div>

            <Separator className="w-full h-6 my-4" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Select & RadioGroup</h1>
            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">4. Select Dropdown</h1>
                <Select.Root value={country} onValueChange={(value) => setCountry(value)}>
          <Select.Trigger>
            <Select.Value />
            <Select.Icon />
          </Select.Trigger>
          <Select.Portal>
            <Select.Positioner sideOffset={8}>
              <Select.Popup className="p-1 bg-white border rounded-lg shadow-lg">
                <Select.Item value="" className="px-3 py-2 cursor-pointer hover:bg-gray-100 rounded">
                  <Select.ItemText>Choose country</Select.ItemText>
                </Select.Item>
                <Select.Item value="us" className="px-3 py-2 cursor-pointer hover:bg-gray-100 rounded">
                  <Select.ItemText>United States</Select.ItemText>
                </Select.Item>
                <Select.Item value="uk" className="px-3 py-2 cursor-pointer hover:bg-gray-100 rounded">
                  <Select.ItemText>United Kingdom</Select.ItemText>
                </Select.Item>
                <Select.Item value="ca" className="px-3 py-2 cursor-pointer hover:bg-gray-100 rounded">
                  <Select.ItemText>Canada</Select.ItemText>
                </Select.Item>
              </Select.Popup>
            </Select.Positioner>
          </Select.Portal>
        </Select.Root>
                <p className="text-sm text-gray-600 mt-2">Selected: {country || 'None'}</p>
      </div>

            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">5. Radio Group</h1>
                <RadioGroup defaultValue="low">
          <label className="flex items-center gap-2 cursor-pointer py-1">
            <Radio.Root value="low" className="flex size-5 items-center justify-center rounded-full data-[checked]:bg-gray-900 data-[unchecked]:border data-[unchecked]:border-gray-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-800">
              <Radio.Indicator className="flex before:size-2 before:rounded-full before:bg-gray-50 data-[unchecked]:hidden" />
            </Radio.Root>
            <span className="text-gray-700">Low Priority</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer py-1">
            <Radio.Root value="medium" className="flex size-5 items-center justify-center rounded-full data-[checked]:bg-gray-900 data-[unchecked]:border data-[unchecked]:border-gray-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-800">
              <Radio.Indicator className="flex before:size-2 before:rounded-full before:bg-gray-50 data-[unchecked]:hidden" />
            </Radio.Root>
            <span className="text-gray-700">Medium Priority</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer py-1">
            <Radio.Root value="high" className="flex size-5 items-center justify-center rounded-full data-[checked]:bg-gray-900 data-[unchecked]:border data-[unchecked]:border-gray-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-800">
              <Radio.Indicator className="flex before:size-2 before:rounded-full before:bg-gray-50 data-[unchecked]:hidden" />
            </Radio.Root>
            <span className="text-gray-700">High Priority</span>
          </label>
        </RadioGroup>

      </div>

            <Separator className="w-full h-6 my-4" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">Form Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Text Input</h1>
                    <input className="w-full border border-gray-300 rounded px-3 py-2" placeholder="Enter your name..." />
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Textarea</h1>
                    <textarea className="w-full border border-gray-300 rounded px-3 py-2" placeholder="Your bio..." rows={3} value={bio} onChange={(e) => setBio(e.target.value)} />
                    <p className="text-xs text-gray-500 mt-2">{bio.length} characters</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Form with Validation</h1>
                    <Form className="flex flex-col gap-4">
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
                        <Button className="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">Submit</Button>
          </Form>

                    <p className="text-sm mt-2">{formStatus}</p>
        </div>

      </div>

            <Separator className="w-full h-6 my-4" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Dialog & Menu</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">6. Dialog</h1>
                    <Button className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700" onClick={() => setDialogOpen(true)}>Open Dialog</Button>
                    <Dialog.Root open={dialogOpen} onOpenChange={open => setDialogOpen(open)}>
            <Dialog.Portal>
              <Dialog.Backdrop className="fixed inset-0 bg-black/50 animate-fade-in" />
              <Dialog.Popup className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-lg shadow-xl p-6 min-w-[320px] max-w-[90vw]">
                <Dialog.Title className="text-xl font-semibold text-gray-900 mb-4">Dialog Title</Dialog.Title>
                                    <p>This is the dialog content. It uses Base UI Dialog components with Portal, Backdrop, and Popup.</p>
                                    <div className="flex flex-row gap-2 justify-end mt-4">
                                        <Button className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50" onClick={() => setDialogOpen(false)}>Cancel</Button>
                                        <Button className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700" onClick={() => setDialogOpen(false)}>Confirm</Button>
                  </div>

                <div className="flex justify-end gap-2 mt-4">
                  <Dialog.Close onClick={() => setDialogOpen(false)}>
                    <span className="sr-only">Close</span>
                    <svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M18 6L6 18M6 6l12 12"></path></svg>
                  </Dialog.Close>
                </div>
              </Dialog.Popup>
            </Dialog.Portal>
          </Dialog.Root>

        </div>

      </div>

            <Separator className="w-full h-6 my-4" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Layout Components</h1>
            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">7. Accordion</h1>
                <Accordion.Root multiple>
          <Accordion.Item value="item-0" className="border-b border-gray-200">
            <Accordion.Header className="flex">
              <Accordion.Trigger className="flex flex-1 items-center justify-between py-4 px-1 font-medium text-gray-900 hover:bg-gray-50 rounded-t-lg transition-colors group">
                What is NWL?
                <svg className="w-5 h-5 text-gray-500 transition-transform group-data-[open]:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M6 9l6 6 6-6"></path></svg>
              </Accordion.Trigger>
            </Accordion.Header>
              <Accordion.Panel className="pb-4 px-1 text-gray-600">
                NWL (Natural Web Language) is a domain-specific language that compiles YAML to production-ready React code.
              </Accordion.Panel>
          </Accordion.Item>
          <Accordion.Item value="item-1" className="border-b border-gray-200">
            <Accordion.Header className="flex">
              <Accordion.Trigger className="flex flex-1 items-center justify-between py-4 px-1 font-medium text-gray-900 hover:bg-gray-50 rounded-t-lg transition-colors group">
                How does it work?
                <svg className="w-5 h-5 text-gray-500 transition-transform group-data-[open]:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M6 9l6 6 6-6"></path></svg>
              </Accordion.Trigger>
            </Accordion.Header>
              <Accordion.Panel className="pb-4 px-1 text-gray-600">
                The Rust compiler reads YAML files and generates TypeScript React components with Tailwind CSS styling.
              </Accordion.Panel>
          </Accordion.Item>
          <Accordion.Item value="item-2" className="border-b border-gray-200">
            <Accordion.Header className="flex">
              <Accordion.Trigger className="flex flex-1 items-center justify-between py-4 px-1 font-medium text-gray-900 hover:bg-gray-50 rounded-t-lg transition-colors group">
                Why Base UI?
                <svg className="w-5 h-5 text-gray-500 transition-transform group-data-[open]:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M6 9l6 6 6-6"></path></svg>
              </Accordion.Trigger>
            </Accordion.Header>
              <Accordion.Panel className="pb-4 px-1 text-gray-600">
                Base UI provides accessible, unstyled components that work perfectly with NWL's Tailwind-first approach.
              </Accordion.Panel>
          </Accordion.Item>
        </Accordion.Root>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Form Components</h1>
            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">8. Field & Fieldset</h1>
                <Fieldset.Root>
          <Fieldset.Legend className="text-lg font-semibold text-gray-900 mb-2">Personal Information</Fieldset.Legend>
                    <Field.Root name="fullName">
            <Field.Label className="text-sm font-medium text-gray-900">Full Name</Field.Label>
            <Field.Control className="h-10 w-full rounded-md border border-gray-200 pl-3.5 text-base text-gray-900 focus:outline focus:outline-2 focus:-outline-offset-1 focus:outline-blue-800" placeholder="Enter your name" />
            <Field.Error className="text-sm text-red-800" />
          </Field.Root>

                    <Field.Root name="email">
            <Field.Label className="text-sm font-medium text-gray-900">Email Address</Field.Label>
            <Field.Control className="h-10 w-full rounded-md border border-gray-200 pl-3.5 text-base text-gray-900 focus:outline focus:outline-2 focus:-outline-offset-1 focus:outline-blue-800" placeholder="you@example.com" />
            <Field.Error className="text-sm text-red-800" />
          </Field.Root>

        </Fieldset.Root>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Tooltip & Popover</h1>
            <div className="flex flex-row gap-4">
                <div className="p-4 border border-gray-200 rounded-lg flex-1">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">9. Tooltip</h1>
                    <Tooltip.Provider>
            <Tooltip.Root>
              <Tooltip.Trigger>
                Hover me
              </Tooltip.Trigger>
              <Tooltip.Portal>
                <Tooltip.Positioner sideOffset={8}>
                  <Tooltip.Popup className="bg-gray-900 text-white text-sm rounded px-3 py-2 shadow-lg">
                    This is a helpful tooltip message!
                  </Tooltip.Popup>
                </Tooltip.Positioner>
              </Tooltip.Portal>
            </Tooltip.Root>
          </Tooltip.Provider>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg flex-1">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">10. Popover</h1>
                    <Popover.Root>
            <Popover.Trigger>
              Click me
            </Popover.Trigger>
            <Popover.Portal>
              <Popover.Positioner sideOffset={8}>
                <Popover.Popup className="bg-white rounded-lg shadow-xl border border-gray-200 p-4 min-w-[200px]">
                  <Popover.Title className="font-semibold text-gray-900 mb-2">Popover Title</Popover.Title>
                  This is the popover content. It can contain any elements and is positioned relative to the trigger.
                </Popover.Popup>
              </Popover.Positioner>
            </Popover.Portal>
          </Popover.Root>

        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Navigation</h1>
            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">11. NavigationMenu</h1>
                <div className="relative">
          <div className="flex items-center justify-between px-6 py-4 bg-white rounded-lg border border-gray-200">
            <div className="hidden md:flex gap-4">
              <a href="/" className="text-gray-300 hover:text-white transition-colors">Home</a>
              <a href="/features" className="text-gray-300 hover:text-white transition-colors">Features</a>
              <a href="/pricing" className="text-gray-300 hover:text-white transition-colors">Pricing</a>
              <a href="/about" className="text-gray-300 hover:text-white transition-colors">About</a>
              <a href="/contact" className="text-gray-300 hover:text-white transition-colors">Contact</a>
            </div>
            <Button variant="soft" className="md:hidden">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M4 6h16M4 12h16M4 18h16"></path></svg>
            </Button>
          </div>
          <Menu.Root>
            <Menu.Portal>
              <Menu.Positioner sideOffset={8} className="z-50">
                <Menu.Popup className="min-w-[200px] p-1 bg-white border rounded-lg shadow-lg">
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

            <Separator className="w-full h-6 my-4" />
    </div>

      </div>
    </>
  );
}

