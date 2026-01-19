import React from 'react';

export default function Home() {
  return (
    <>
      <div className="bg-gray-50 min-h-screen">
            <h1 className="text-3xl font-bold text-gray-900">Welcome back</h1>
            <p className="text-xl text-gray-600">Your projects</p>
            <div className="p-6 bg-white rounded-lg shadow-md">
            <h1 className="text-xl font-semibold mb-2">Sample Project</h1>
            <p className="text-gray-600">This is a sample project card compiled from YAML</p>
            <button className="bg-blue-600 text-white px-4 py-2 rounded mt-4" onClick={() => window.location.href = "/project/123"}>Open Project</button>
    </div>

      </div>
    </>
  );
}

