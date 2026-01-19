import React, { useState } from 'react';

export default function Counter() {
  const [count, setCount] = useState(0), [name, setName] = useState("Guest"), [email, setEmail] = useState("");
  return (
    <>
      <div className="bg-gray-100">
            <div className="p-8 bg-white rounded-xl shadow-lg max-w-md w-full">
            <h1 className="text-2xl font-bold text-gray-900 mb-6">Counter Example</h1>
            <p className="text-lg text-gray-600 mb-4">Hello, {name}!</p>
            <div className="flex flex-row items-center justify-center gap-4 mb-6">
                <button className="bg-red-500 text-white w-12 h-12 rounded-full text-xl font-bold hover:bg-red-600" onClick={() => setCount(count - 1)}>-</button>
                <h1 className="text-4xl font-bold text-gray-900 w-16 text-center">{count}</h1>
                <button className="bg-green-500 text-white w-12 h-12 rounded-full text-xl font-bold hover:bg-green-600" onClick={() => setCount(count + 1)}>+</button>
      </div>

            <div className="h-6" />
            <h1 className="text-xl font-semibold text-gray-900 mb-4">Update Your Name</h1>
            <input className="w-full border border-gray-300 rounded-lg px-4 py-2 mb-4" placeholder="Enter your name" value={name} onChange={(e) => setName(e.target.value)} />
            <p className="text-sm text-gray-600 mb-2">Your email (auto-saves):</p>
            <input className="w-full border border-gray-300 rounded-lg px-4 py-2" placeholder="Enter your email" value={email} onChange={(e) => setEmail(e.target.value)} />
    </div>

      </div>
    </>
  );
}

