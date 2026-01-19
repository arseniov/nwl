import React, { useState } from 'react';

export default function Playground() {
  const [agreedToTerms, setAgreedToTerms] = useState(false), [volume, setVolume] = useState(50), [country, setCountry] = useState(""), [priority, setPriority] = useState("medium"), [bio, setBio] = useState(""), [formStatus, setFormStatus] = useState(""), [selectedDate, setSelectedDate] = useState(""), [selectedTime, setSelectedTime] = useState(""), [selectedColor, setSelectedColor] = useState("#3b82f6"), [uploadedFiles, setUploadedFiles] = useState([]), [progressValue, setProgressValue] = useState(45), [toggleValue, setToggleValue] = useState(true), [activeTab, setActiveTab] = useState("tab1"), [isModalOpen, setIsModalOpen] = useState(false), [ratingValue, setRatingValue] = useState(3), [searchQuery, setSearchQuery] = useState(""), [counterValue, setCounterValue] = useState(5), [currentPage, setCurrentPage] = useState(1), [tags, setTags] = useState(["react", "rust"]), [notification, setNotification] = useState("");
  return (
    <>
      <div className="bg-gray-50 py-8">
            <div className="max-w-4xl w-full p-8 bg-white rounded-xl shadow-lg">
            <h1 className="text-2xl font-bold text-gray-900 mb-2">NWL Component Playground</h1>
            <p className="text-gray-600 mb-8">Explore all interactive components available in NWL</p>
            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-4">Form Components</h1>
            <div className="grid grid-cols-2">
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
          <option value="" disabled>Choose country</option>
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
                    <form onSubmit={(e) => { e.preventDefault(); setFormStatus('Submitted!') }}>
                        <input className="w-full border border-gray-300 rounded px-3 py-2 mb-2" placeholder="Email" />
                        <button className="w-full bg-blue-600 text-white px-4 py-2 rounded">Submit</button>
          </form>

                    <p className="text-sm text-green-600 mt-2">{formStatus}</p>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Date & Time Inputs</h1>
            <div className="grid grid-cols-3">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">7. Date Input</h1>
                    <input type="date" value={selectedDate} onChange={(e) => setSelectedDate(e.target.value)} />
                    <p className="text-sm text-gray-600 mt-2">{selectedDate || '-'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">8. Time Input</h1>
                    <input type="time" value={selectedTime} onChange={(e) => setSelectedTime(e.target.value)} />
                    <p className="text-sm text-gray-600 mt-2">{selectedTime || '-'}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">9. DateTime Input</h1>
                    <input type="datetime-local" />
                    <p className="text-sm text-gray-600 mt-2">-</p>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Selection & Display</h1>
            <div className="grid grid-cols-3">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">10. Color Picker</h1>
                    <input type="color" value={selectedColor} onChange={(e) => setSelectedColor(e.target.value)} />
                    <p className="text-sm text-gray-600 mt-2">Selected: {selectedColor}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">11. File Upload</h1>
                    <input type="file" />
                    <p className="text-sm text-gray-600 mt-2">{uploadedFiles ? uploadedFiles.length : 0} files</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">12. Progress Bar</h1>
                    <div role="progressbar" aria-valuenow={progressValue} style={{width: `${progressValue}%`}} />
                    <div className="flex flex-row gap-2 mt-2">
                        <button className="px-2 py-1 border rounded" onClick={() => setProgressValue(Math.max(0, progressValue - 10))}>-</button>
                        <button className="px-2 py-1 border rounded" onClick={() => setProgressValue(Math.min(100, progressValue + 10))}>+</button>
          </div>

        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Toggle Components</h1>
            <div className="grid grid-cols-2">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">13. Toggle Switch</h1>
                    <label className="inline-flex relative items-center cursor-pointer"><span className="ml-3 text-sm font-medium text-gray-900">Enable notifications</span><input type="checkbox" className="sr-only peer" checked={toggleValue} onChange={() => setToggleValue(!toggleValue)} /><div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-green-100"></div><span className="ml-3 text-sm font-medium text-gray-900">Enable notifications</span></label>
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
                    <p className="text-sm text-gray-600 mt-2">{ratingValue}/5</p>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Navigation</h1>
            <div className="grid grid-cols-2">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">15. Tabs</h1>
                    <div>
            <div className="border-b border-gray-200">
              <nav className="-mb-px flex space-x-8">
                <label className="border-blue-500 text-blue-600 cursor-pointer whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm">Tab 1<input type="radio" name="tabs" value="tab1" className="sr-only peer" checked={activeTab === "tab1"} onChange={() => setActiveTab("tab1")} /></label>
                <label className="border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 cursor-pointer whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm">Tab 2<input type="radio" name="tabs" value="tab2" className="sr-only peer" checked={activeTab === "tab2"} onChange={() => setActiveTab("tab2")} /></label>
                <label className="border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 cursor-pointer whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm">Tab 3<input type="radio" name="tabs" value="tab3" className="sr-only peer" checked={activeTab === "tab3"} onChange={() => setActiveTab("tab3")} /></label>
              </nav>
            </div>
          </div>

                    <p className="text-sm text-gray-600 mt-3">Active: {activeTab}</p>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">16. Breadcrumb</h1>
                    <nav aria-label="Breadcrumb"><ol className="flex items-center space-x-2">          <span><a href="/" className="text-blue-600 hover:underline">Home</a><span className="mx-2 text-gray-400">/</span></span>
          <span><a href="/docs" className="text-blue-600 hover:underline">Docs</a><span className="mx-2 text-gray-400">/</span></span>
          <span><span className="text-gray-600">Guide</span></span>
</ol></nav>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">17. Pagination</h1>
                    <div className="flex items-center justify-center space-x-2 ">          <button onClick={() => currentPage > 1 && setCurrentPage(currentPage - 1)} className="px-3 py-1 border rounded hover:bg-gray-100">Previous</button><span className="px-4 py-2 border bg-blue-50 text-blue-600 font-medium">Page { currentPage } of 5</span><button onClick={() => currentPage < 5 && setCurrentPage(currentPage + 1)} className="px-3 py-1 border rounded hover:bg-gray-100">Next</button></div>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">18. Accordion</h1>
                    <div>
            <details className="mb-2 border rounded-lg">
              <summary className="flex items-center justify-between p-4 font-medium cursor-pointer list-none">Section 1      <svg className="w-5 h-5 ml-2 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
    </summary>              <div className="p-4 text-gray-600">Content for section 1</div>
  </details>
            <details className="mb-2 border rounded-lg">
              <summary className="flex items-center justify-between p-4 font-medium cursor-pointer list-none">Section 2      <svg className="w-5 h-5 ml-2 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
    </summary>              <div className="p-4 text-gray-600">Content for section 2</div>
  </details>
            <details className="mb-2 border rounded-lg">
              <summary className="flex items-center justify-between p-4 font-medium cursor-pointer list-none">Section 3      <svg className="w-5 h-5 ml-2 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
    </summary>              <div className="p-4 text-gray-600">Content for section 3</div>
  </details>
          </div>

        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Modal & Dialogs</h1>
            <div className="grid grid-cols-2">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">19. Modal</h1>
                    <button className="bg-blue-600 text-white px-4 py-2 rounded" onClick={() => setIsModalOpen(true)}>Open Modal</button>
                    <>{isModalOpen && (
            <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" >
              <div className="bg-white rounded-lg p-6 max-w-md w-full shadow-xl">
                                    <p className="mb-4">Are you sure you want to proceed?</p>
                                    <div className="flex flex-row justify-end gap-2">
                                        <button className="border px-4 py-2 rounded" onClick={() => setIsModalOpen(false)}>Cancel</button>
                                        <button className="bg-blue-600 text-white px-4 py-2 rounded" onClick={() => setIsModalOpen(false)}>Confirm</button>
                  </div>

              </div>
            </div>
          )}</>
        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">20. Alert</h1>
                    <div className="bg-blue-50 text-blue-800 border-blue-200 border p-4 rounded-lg flex items-start "><p>This is an info alert</p></div>
                    <div className="bg-yellow-50 text-yellow-800 border-yellow-200 border p-4 rounded-lg flex items-start "><p>Warning message here</p></div>
                    <div className="bg-green-50 text-green-800 border-green-200 border p-4 rounded-lg flex items-start "><p>Success message here</p></div>
        </div>

      </div>

            <h1 className="text-xl font-semibold text-gray-900 mb-6 mt-8">Indicators & Badges</h1>
            <div className="grid grid-cols-3">
                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">21. Badges</h1>
                    <div className="flex flex-row gap-2">
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">New</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Hot</span>
                        <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-semibold">Sale</span>
          </div>

        </div>

                <div className="p-4 border border-gray-200 rounded-lg">
                    <h1 className="text-base font-semibold text-gray-900 mb-3">22. Tags</h1>
                    <div className="flex flex-row gap-2">
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
            <div className="grid grid-cols-2">
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
                <div className=""><div className="flex flex-wrap gap-2 mb-2">{ tags.map((chip, i) => <span key={i} className="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">{chip}<button className="ml-1 text-blue-600" onClick={() => setTags(tags.filter((_, j) => j !== i))}>×</button></span>) }</div><div className="mb-2"><span className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200">react</span>
<span className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200">rust</span>
<span className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200">typescript</span>
<span className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200">javascript</span>
<span className="inline-block px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded-full mr-2 mb-1 cursor-pointer hover:bg-gray-200">python</span>
</div><input type="text" placeholder="Type and press Enter..." onKeyDown={(e) => e.key === 'Enter' && setTags([...tags, e.target.value]) && (e.target.value = '')} className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" /></div>
                <p className="text-sm text-gray-600 mt-3">Tags: {tags.join(', ')}</p>
      </div>

            <div className="h-6" />
    </div>

      </div>
    </>
  );
}

