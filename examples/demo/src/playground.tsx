import React, { useState } from 'react';

export default function Playground() {
  const [menuOpen, setMenuOpen] = useState(false), [agreedToTerms, setAgreedToTerms] = useState(false), [volume, setVolume] = useState(50), [country, setCountry] = useState(""), [priority, setPriority] = useState("medium"), [bio, setBio] = useState(""), [formStatus, setFormStatus] = useState(""), [email, setEmail] = useState(""), [password, setPassword] = useState(""), [selectedDate, setSelectedDate] = useState(""), [selectedTime, setSelectedTime] = useState(""), [selectedColor, setSelectedColor] = useState("#3b82f6"), [uploadedFiles, setUploadedFiles] = useState([]), [progressValue, setProgressValue] = useState(45), [toggleValue, setToggleValue] = useState(true), [activeTab, setActiveTab] = useState("tab1"), [isModalOpen, setIsModalOpen] = useState(false), [ratingValue, setRatingValue] = useState(3), [searchQuery, setSearchQuery] = useState(""), [counterValue, setCounterValue] = useState(5), [currentPage, setCurrentPage] = useState(1), [tags, setTags] = useState(["react", "rust"]), [notification, setNotification] = useState(""), [listSelected, setListSelected] = useState("item2"), [imageSrc, setImageSrc] = useState("https://picsum.photos/seed/picsum/200/300");
  return (
    <>
      <div className="bg-gray-50 py-8">
            <nav className="w-full flex items-center justify-between px-6 py-4 sticky top-0 z-50 bg-black"><a href="/" className="text-xl font-bold text-white">NWL</a><div className="flex items-center gap-6">    <a href="/" className="text-sm font-medium transition-colors text-white hover:text-blue-400">Home</a>
    <a href="/playground" className="text-sm font-medium transition-colors text-blue-400">Playground</a>
    <a href="/docs" className="text-sm font-medium transition-colors text-white hover:text-blue-400">Docs</a>
</div></nav>
            <div className="relative">
      <div className="flex items-center justify-between px-6 py-4 bg-black">
            <button className="md:hidden text-white p-2" onClick={() => setMenuOpen(!menuOpen)}>
      <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
      </svg>
    </button>
    <a href="/" className="text-white hover:text-blue-400 transition-colors text-sm font-medium">Home</a>
    <a href="/playground" className="text-white hover:text-blue-400 transition-colors text-sm font-medium">Playground</a>
    <a href="/docs" className="text-white hover:text-blue-400 transition-colors text-sm font-medium">Docs</a>
    <div className="hidden md:flex items-center gap-6">
        </div>
      </div>
      <div className="fixed inset-0 bg-gray-900 z-50 transform {{menuOpen ? 'translate-x-0' : 'translate-x-full'}} transition-transform duration-300 md:hidden"
           style={{display: menuOpen ? 'block' : 'none'}}>
        <div className="p-6">
          <button className="absolute top-4 right-4 text-white" onClick={() => setMenuOpen(false)}>
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
          </button>
          <div className="mt-8">
        <a href="/" className="block py-3 text-white hover:text-blue-400 border-b border-gray-700">Home</a>
    <a href="/playground" className="block py-3 text-white hover:text-blue-400 border-b border-gray-700">Playground</a>
    <a href="/docs" className="block py-3 text-white hover:text-blue-400 border-b border-gray-700">Docs</a>
      </div>
        </div>
      </div>
    </div>

            <div className="max-w-4xl w-full p-8 bg-white rounded-xl shadow-lg">
            <h1 className="text-2xl font-bold text-gray-900 mb-2">NWL Component Playground</h1>
            <p className="text-gray-600 mb-8">Explore all interactive components available in NWL</p>
            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">Form Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">1. Checkbox</h1>
                    <label className="flex items-center gap-2"><input type="checkbox" checked={agreedToTerms} onChange={() => setAgreedToTerms(!agreedToTerms)} />I agree to the terms</label>
                    <p className="text-sm text-gray-600 mt-2">{agreedToTerms ? 'Agreed!' : 'Not agreed'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">2. Slider</h1>
                    <label className="flex flex-col gap-1"><span>Volume: {volume}%</span><input type="range" min="0" max="100" step="10" value={volume} onChange={(e) => setVolume(Number(e.target.value))} /></label>
                    <div className="flex flex-row justify-between">
                        <p className="text-xs text-gray-500">0%</p>
                        <p className="text-xs text-gray-500">100%</p>
          </div>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">3. Select</h1>
                    <select value={country} onChange={(e) => setCountry(e.target.value)}>
          <option value="">Choose country</option>
          <option value="us">United States</option>
          <option value="uk">United Kingdom</option>
          <option value="ca">Canada</option>
          </select>
                    <p className="text-sm text-gray-600 mt-2">{country || 'None'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">4. Radio Group</h1>
                    <div>
            <label className="flex items-center gap-2"><input type="radio" value="low" checked={priority === "low"} onChange={() => setPriority("low")} />Low</label>
            <label className="flex items-center gap-2"><input type="radio" value="medium" checked={priority === "medium"} onChange={() => setPriority("medium")} />Medium</label>
            <label className="flex items-center gap-2"><input type="radio" value="high" checked={priority === "high"} onChange={() => setPriority("high")} />High</label>
          </div>

                    <p className="text-sm text-gray-600 mt-2">{priority}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">5. Textarea</h1>
                    <textarea className="w-full border border-gray-300 rounded px-3 py-2" placeholder="Your bio..." rows={3} value={bio} onChange={(e) => setBio(e.target.value)} />
                    <p className="text-xs text-gray-500 mt-2">{bio.length} chars</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">6. Form</h1>
                    <form onSubmit={(e) => { e.preventDefault(); let _hasError = false; if (!password.trim()) { console.error('Password is required'); _hasError = true; } if (password.length < 8) { console.error('Password must be at least 8 characters'); _hasError = true; } if (!email.trim()) { console.error('Email is required'); _hasError = true; } if (!new RegExp('^[^@]+@[^@]+.[^@]+$').test(email)) { console.error('Please enter a valid email address'); _hasError = true; } if (_hasError) { console.error('Validation failed'); setFormStatus('Validation failed!'); return; } setFormStatus('Submitted!') }}>
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
                        <button className="w-full bg-blue-600 text-white px-4 py-2 rounded">Submit</button>
          </form>

                    <p className="text-sm mt-2">{formStatus}</p>
        </div>

      </div>

            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">7. Date Input</h1>
                    <input type="date" value={selectedDate} onChange={(e) => setSelectedDate(e.target.value)} />
                    <p className="text-sm text-gray-600 mt-2">Selected: {selectedDate || 'None'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">8. Time Input</h1>
                    <input type="time" value={selectedTime} onChange={(e) => setSelectedTime(e.target.value)} />
                    <p className="text-sm text-gray-600 mt-2">Selected: {selectedTime || 'None'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">9. DateTime Input</h1>
                    <input type="datetime-local" />
                    <p className="text-sm text-gray-600 mt-2">DateTime picker</p>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Toggle Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">13. Toggle Switch</h1>
                    <label className="inline-flex relative items-center cursor-pointer"><span className="mr-3 text-sm font-medium text-gray-900">Enable notifications</span><input type="checkbox" className="sr-only peer" checked={toggleValue} onChange={() => setToggleValue(!toggleValue)} /><div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-green-500"></div></label>
                    <p className="text-sm text-gray-600 mt-2">{toggleValue ? 'Enabled' : 'Disabled'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">14. Rating</h1>
                    <div><label className="flex items-center cursor-pointer">          <input type="radio" name="rating" value="" className="sr-only peer" />          <span className="text-yellow-400 text-xl">★</span>
          <span className="text-yellow-400 text-xl">★</span>
          <span className="text-yellow-400 text-xl">★</span>
          <span className="text-yellow-400 text-xl">★</span>
          <span className="text-yellow-400 text-xl">★</span>
</label></div>
                    <p className="text-sm text-gray-600 mt-2">Rating: {ratingValue}/5</p>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Modal & Dialogs</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">19. Modal</h1>
                    <button className="bg-blue-600 text-white px-4 py-2 rounded" onClick={() => setIsModalOpen(true)}>Open Modal</button>
          {isModalOpen && (
            <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onClick={() => setIsModalOpen(false)}>
              <div className="bg-white rounded-lg p-6 max-w-md w-full shadow-xl" onClick={(e) => { e.stopPropagation(); setIsModalOpen(false) }}>
                <div className="flex items-center justify-between mb-4">
                  <h2 className="text-xl font-bold">Example Modal</h2>
                  <button className="text-gray-500 hover:text-gray-700" onClick={() => setIsModalOpen(false)}>
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                  </button>
                </div>
                                    <p className="text-gray-600">This is a modal dialog. Click outside or the X to close.</p>
              </div>
            </div>
)}
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">20. Alert</h1>
                    <div className="my-2 bg-blue-50 text-blue-800 border-blue-200 border p-4 rounded-lg flex items-start "><p>This is an informational alert message!</p></div>
                    <div className="my-2 bg-green-50 text-green-800 border-green-200 border p-4 rounded-lg flex items-start "><p>Success! Your changes have been saved.</p></div>
                    <div className="my-2 bg-yellow-50 text-yellow-800 border-yellow-200 border p-4 rounded-lg flex items-start "><p>Warning: This action cannot be undone.</p></div>
                    <div className="my-2 bg-red-50 text-red-800 border-red-200 border p-4 rounded-lg flex items-start "><p>Error: Something went wrong.</p></div>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Indicators & Badges</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">21. Badge</h1>
                    <div className="flex flex-row gap-2 flex-wrap">
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Default</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Primary</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Success</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Warning</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Danger</span>
          </div>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">22. Tag</h1>
                    <div className="flex flex-row gap-2 flex-wrap">
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 ">React</span>
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 ">Rust</span>
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 ">TypeScript</span>
          </div>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">23. Spinner</h1>
                    <div className="flex flex-row gap-4">
                        <div className="flex items-center justify-center"><div className="w-6 h-6 animate-spin rounded-full border-2 border-gray-300 border-t-blue-600"></div><div className="sr-only">Loading...</div></div>
                        <div className="flex items-center justify-center"><div className="w-6 h-6 animate-spin rounded-full border-2 border-gray-300 border-t-blue-600"></div><div className="sr-only">Loading...</div></div>
                        <div className="flex items-center justify-center"><div className="w-6 h-6 animate-spin rounded-full border-2 border-gray-300 border-t-blue-600"></div><div className="sr-only">Loading...</div></div>
          </div>

        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Input Components</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">24. Search Input</h1>
                    <div className="relative"><input type="search" placeholder="Search..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} /><button className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400">🔍</button></div>
                    <p className="text-sm text-gray-600 mt-2">Query: {searchQuery || '-'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">25. Counter</h1>
                    <div className="flex items-center border rounded-lg ">          <button onClick={() => counterValue > 0 && setCounterValue(counterValue - 1)} className="px-3 py-1 border-r hover:bg-gray-100">-</button><span className="text-lg font-semibold mx-4">{counterValue}</span><button onClick={() => counterValue < 99 && setCounterValue(counterValue + 1)} className="px-3 py-1 border-l hover:bg-gray-100">+</button></div>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">26. Copy Button</h1>
                    <button onClick={() => navigator.clipboard.writeText("https://nwl.dev")} className="inline-flex items-center px-3 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 ">Copy</button>
                    <button onClick={() => navigator.clipboard.writeText("Copy Text")} className="inline-flex items-center px-3 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 ml-2">Copy</button>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">27. Avatar</h1>
                    <div className="flex flex-row gap-2 items-center">
                        <div className="flex items-center ">            <div className="w-10 h-10 text-sm rounded-full overflow-hidden bg-gray-100"><span className="flex items-center justify-center w-full h-full rounded-full bg-gray-200 text-gray-600 font-medium">?</span></div><span className="ml-2 font-medium text-gray-700">JD</span></div>
                        <div className="flex items-center ">            <div className="w-10 h-10 text-sm rounded-full overflow-hidden bg-gray-100"><span className="flex items-center justify-center w-full h-full rounded-full bg-gray-200 text-gray-600 font-medium">?</span></div><span className="ml-2 font-medium text-gray-700">AB</span></div>
                        <div className="flex items-center ">            <div className="w-10 h-10 text-sm rounded-full overflow-hidden bg-gray-100"><span className="flex items-center justify-center w-full h-full rounded-full bg-gray-200 text-gray-600 font-medium">?</span></div><span className="ml-2 font-medium text-gray-700">CD</span></div>
          </div>

        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Chip Input</h1>
            <div className="p-4 border border-gray-200 rounded-lg">
                <h1 className="text-base font-semibold text-gray-900 mb-3">28. Chip Input (Tags)</h1>
                <div className=""><div className="flex flex-wrap gap-2 mb-2">{tags.map((chip, i) => <span key={i} className="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-sm flex items-center">{chip}<button className="ml-1 text-blue-600 hover:text-blue-800" onClick={() => setTags(tags.filter((_, j) => j !== i))}>×</button></span>)}</div><div className="mb-2 flex flex-wrap"><span key="react" className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200" style={tags.includes("react") ? { opacity: 0.5, pointerEvents: 'none' } : {}} onClick={() => { if (!tags.includes("react")) { setTags([...tags, "react"]); } }}>react</span><span key="rust" className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200" style={tags.includes("rust") ? { opacity: 0.5, pointerEvents: 'none' } : {}} onClick={() => { if (!tags.includes("rust")) { setTags([...tags, "rust"]); } }}>rust</span><span key="typescript" className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200" style={tags.includes("typescript") ? { opacity: 0.5, pointerEvents: 'none' } : {}} onClick={() => { if (!tags.includes("typescript")) { setTags([...tags, "typescript"]); } }}>typescript</span><span key="javascript" className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200" style={tags.includes("javascript") ? { opacity: 0.5, pointerEvents: 'none' } : {}} onClick={() => { if (!tags.includes("javascript")) { setTags([...tags, "javascript"]); } }}>javascript</span><span key="python" className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200" style={tags.includes("python") ? { opacity: 0.5, pointerEvents: 'none' } : {}} onClick={() => { if (!tags.includes("python")) { setTags([...tags, "python"]); } }}>python</span></div><input type="text" placeholder="Type and press Enter..." onKeyDown={(e) => { if (e.key === 'Enter') { const newValue = e.target.value.trim(); if (newValue && !tags.includes(newValue)) { setTags([...tags, newValue]); } e.target.value = ''; } }} className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" /></div>
                <p className="text-sm text-gray-600 mt-3">Tags: {tags.join(', ')}</p>
      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">List & Media</h1>
            <div className="flex flex-col gap-4">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">29. List</h1>
                    <div className="border rounded border rounded">
            <div key="list-item-0" className="p-2 border-b last:border-b-0 cursor-pointer hover:bg-gray-50" onClick={() => setListSelected('item1')}>
              First item
            </div>
            <div key="list-item-1" className="p-2 border-b last:border-b-0 cursor-pointer hover:bg-gray-50" onClick={() => setListSelected('item2')}>
              Second item
            </div>
            <div key="list-item-2" className="p-2 border-b last:border-b-0 cursor-pointer hover:bg-gray-50" onClick={() => setListSelected('item3')}>
              Third item
            </div>
          </div>

                    <p className="text-sm text-gray-600 mt-2">Selected: {listSelected || 'None'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">30. Image</h1>
                    <img className="w-full rounded" src="https://picsum.photos/seed/picsum/1280/720" alt="Placeholder image" />
                    <p className="text-sm text-gray-600 mt-2">Sample image with placeholder</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">31. Container</h1>
                    <div className="bg-gray-100 p-4 rounded">
                        <p className="text-gray-700">This is a container with background color and padding</p>
          </div>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">32. Email Input</h1>
                    <input type="email" placeholder="Enter your email" value={email} className="w-full border border-gray-300 rounded px-3 py-2"/>
                    <p className="text-sm text-gray-600 mt-2">Email: {email || 'Not entered'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">33. URL Input</h1>
                    <input type="url" placeholder="Enter a URL" className="w-full border border-gray-300 rounded px-3 py-2"/>
                    <p className="text-sm text-gray-600 mt-2">URL input field for web addresses</p>
        </div>

      </div>

            <div className="h-6" />
    </div>

      </div>
    </>
  );
}

