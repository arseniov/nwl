import React, { useState } from 'react';
import { Button } from '@base-ui/react/button';
import { Select } from '@base-ui/react/select';
import { Radio } from '@base-ui/react/radio';
import { RadioGroup } from '@base-ui/react/radio-group';
import { Dialog } from '@base-ui/react/dialog';
import { Menu } from '@base-ui/react/menu';

export default function Playground() {
  const [country, setCountry] = useState(""), [bio, setBio] = useState(""), [formStatus, setFormStatus] = useState(""), [email, setEmail] = useState(""), [password, setPassword] = useState(""), [searchQuery, setSearchQuery] = useState("");
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
            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Button Component</h1>
            <div className="flex flex-row gap-4 flex-wrap">
                <Button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">Primary Button</Button>
                <Button className="bg-gray-200 text-gray-800 px-4 py-2 rounded hover:bg-gray-300">Secondary</Button>
                <Button className="border border-blue-600 text-blue-600 px-4 py-2 rounded hover:bg-blue-50">Outline</Button>
      </div>

            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI Select Component</h1>
            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">3. Select Dropdown</h1>
                <Select.Root>
          <Select.Trigger>
            <Select.Value value={country} />
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

            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">BaseUI RadioGroup Component</h1>
            <div className="p-4 border border-gray-200 rounded-lg mb-4">
                <h1 className="text-base font-semibold text-gray-900 mb-3">4. Radio Button Group</h1>
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

                <p className="text-sm text-gray-600 mt-2">Select an option above</p>
      </div>

            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">Form Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">5. Textarea</h1>
                    <textarea className="w-full border border-gray-300 rounded px-3 py-2" placeholder="Your bio..." rows={3} value={bio} onChange={(e) => setBio(e.target.value)} />
                    <p className="text-xs text-gray-500 mt-2">{bio.length} characters</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">6. Form with Validation</h1>
                    <form onSubmit={(e) => { e.preventDefault(); let _hasError = false; if (!password.trim()) { console.error('Password is required'); _hasError = true; } if (password.length < 8) { console.error('Password must be at least 8 characters'); _hasError = true; } if (!email.trim()) { console.error('Email is required'); _hasError = true; } if (!new RegExp('^[^@]+@[^@]+.[^@]+$').test(email)) { console.error('Please enter a valid email address'); _hasError = true; } if (_hasError) { console.error('Validation failed'); setFormStatus('Validation failed!'); return; } setFormStatus('Submitted!') }}>
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
                        <Button className="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">Submit</Button>
          </form>

                    <p className="text-sm mt-2">{formStatus}</p>
        </div>

      </div>

            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">Input Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Text Input</h1>
                    <input className="w-full border border-gray-300 rounded px-3 py-2" placeholder="Enter your name..." />
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Search Input</h1>
                    <div className="relative"><input type="search" placeholder="Search..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} /><button className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400">🔍</button></div>
                    <p className="text-sm text-gray-600 mt-2">Query: {searchQuery || '-'}</p>
        </div>

      </div>

            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">Display Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">21. Badge</h1>
                    <div className="flex flex-row gap-2 flex-wrap">
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Default</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Success</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Warning</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Error</span>
          </div>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Alert Messages</h1>
                    <div className="my-2 bg-blue-50 text-blue-800 border-blue-200 border p-4 rounded-lg flex items-start "><p>This is an informational alert message!</p></div>
                    <div className="my-2 bg-green-50 text-green-800 border-green-200 border p-4 rounded-lg flex items-start "><p>Success! Your changes have been saved.</p></div>
                    <div className="my-2 bg-yellow-50 text-yellow-800 border-yellow-200 border p-4 rounded-lg flex items-start "><p>Warning: This action cannot be undone.</p></div>
                    <div className="my-2 bg-red-50 text-red-800 border-red-200 border p-4 rounded-lg flex items-start "><p>Error: Something went wrong.</p></div>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">Tags</h1>
                    <div className="flex flex-row gap-2 flex-wrap">
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 ">React</span>
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 ">Rust</span>
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 ">TypeScript</span>
          </div>

        </div>

      </div>

            <div className="h-6" />
    </div>

      </div>
    </>
  );
}

