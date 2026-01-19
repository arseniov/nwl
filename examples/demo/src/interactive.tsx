import React, { useState } from 'react';

export default function Interactive() {
  const [agreedToTerms, setAgreedToTerms] = useState(false), [volume, setVolume] = useState(50), [country, setCountry] = useState(""), [priority, setPriority] = useState("medium"), [bio, setBio] = useState(""), [formStatus, setFormStatus] = useState("");
  return (
    <>
      <div className="bg-gray-50 py-8">
            <div className="max-w-2xl w-full p-8 bg-white rounded-xl shadow-lg">
            <h1 className="text-2xl font-bold text-gray-900 mb-8">Interactive Components Demo</h1>
            <div className="h-6" />
            <h1 className="text-lg font-semibold text-gray-900 mb-4">1. Checkbox (Toggle)</h1>
            <label className="flex items-center gap-2"><input type="checkbox" className="mb-6" checked={agreedToTerms} onChange={() => setAgreedToTerms(!agreedToTerms)} />I agree to the terms and conditions</label>
            <p className="text-gray-600 mb-8">Status: {agreedToTerms ? 'Agreed!' : 'Not agreed'}</p>
            <h1 className="text-lg font-semibold text-gray-900 mb-4">2. Slider (Volume Control)</h1>
            <label className="flex flex-col gap-1"><span>Volume: {volume}%</span><input type="range" className="w-full mb-2" min="0" max="100" step="10" value={volume} onChange={(e) => setVolume(Number(e.target.value))} /></label>
            <div className="flex flex-row justify-between">
                <p className="text-sm text-gray-500">0%</p>
                <p className="text-sm text-gray-500">100%</p>
      </div>

            <div className="h-6" />
            <h1 className="text-lg font-semibold text-gray-900 mb-4">3. Select Dropdown</h1>
            <select className="w-full mb-6" value={country} onChange={(e) => setCountry(e.target.value)}>
      <option value="" disabled>Choose a country</option>
      <option value="us">United States</option>
      <option value="uk">United Kingdom</option>
      <option value="ca">Canada</option>
      <option value="de">Germany</option>
      </select>
            <p className="text-gray-600 mb-8">You selected: {country || 'None'}</p>
            <h1 className="text-lg font-semibold text-gray-900 mb-4">4. Radio Group (Priority)</h1>
            <div className="mb-6">
        <p className="font-semibold mb-2">Select priority level</p>
        <label className="flex items-center gap-2"><input type="radio" value="low" checked={priority === "low"} onChange={() => setPriority("low")} />Low Priority</label>
        <label className="flex items-center gap-2"><input type="radio" value="medium" checked={priority === "medium"} onChange={() => setPriority("medium")} />Medium Priority</label>
        <label className="flex items-center gap-2"><input type="radio" value="high" checked={priority === "high"} onChange={() => setPriority("high")} />High Priority</label>
      </div>

            <p className="text-gray-600 mb-8">Current priority: {priority}</p>
            <h1 className="text-lg font-semibold text-gray-900 mb-4">5. Textarea (Bio)</h1>
            <textarea className="w-full border border-gray-300 rounded-lg px-4 py-2 mb-4" placeholder="Tell us about yourself..." rows="4" value={bio} onChange={(e) => setBio(e.target.value)} />
            <p className="text-sm text-gray-500 mb-8">Character count: {bio.length}</p>
            <h1 className="text-lg font-semibold text-gray-900 mb-4">6. Form Submission</h1>
            <form className="space-y-4" onSubmit={(e) => { e.preventDefault(); setFormStatus('Form submitted successfully!') }}>
                <input className="w-full border border-gray-300 rounded-lg px-4 py-2" placeholder="Email address" />
                <input className="w-full border border-gray-300 rounded-lg px-4 py-2" placeholder="Password" />
                <button className="bg-blue-600 text-white px-6 py-3 rounded-lg font-semibold hover:bg-blue-700 w-full">Submit Form</button>
      </form>

            <p className="text-green-600 font-medium mt-4">{formStatus}</p>
    </div>

      </div>
    </>
  );
}

