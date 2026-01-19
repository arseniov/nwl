import React from 'react';

export default function Landing() {
  return (
    <>
      <div className="bg-white">
            <div className="bg-white border-b border-gray-200 sticky top-0 z-50">
            <div className="flex flex-row items-center justify-between px-6 py-4 max-w-7xl mx-auto">
                <div className="flex items-center gap-3">
                    <h1 className="text-xl font-bold text-gray-900">NWL</h1>
        </div>

                <div className="flex flex-row items-center gap-6">
                    <p className="text-gray-600 hover:text-gray-900">Features</p>
                    <p className="text-gray-600 hover:text-gray-900">Pricing</p>
                    <button className="bg-blue-600 text-white px-4 py-2 rounded-lg font-medium hover:bg-blue-700">Get Started</button>
        </div>

      </div>

    </div>

            <div className="bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 py-20 px-6">
            <div className="flex flex-col items-center text-center max-w-4xl mx-auto gap-8">
                <h1 className="text-5xl font-bold text-white leading-tight">Build Beautiful UIs Faster</h1>
                <p className="text-xl text-slate-400 max-w-2xl">The declarative DSL that transforms how you build React applications.</p>
                <div className="flex flex-row items-center gap-4">
                    <button className="bg-blue-500 text-white px-8 py-4 rounded-xl text-lg font-semibold hover:bg-blue-600">Start Building Free</button>
                    <button className="bg-white/10 text-white px-8 py-4 rounded-xl text-lg hover:bg-white/20">View Documentation</button>
        </div>

      </div>

    </div>

            <div className="bg-white py-16 px-6">
            <div className="flex flex-row grid grid-cols-3 gap-8 max-w-5xl mx-auto">
                <div className="text-center p-8">
                    <h1 className="text-4xl font-bold text-gray-900">10K+</h1>
                    <p className="text-gray-500 mt-2">Active Developers</p>
        </div>

                <div className="text-center p-8">
                    <h1 className="text-4xl font-bold text-gray-900">50M+</h1>
                    <p className="text-gray-500 mt-2">Components Generated</p>
        </div>

                <div className="text-center p-8">
                    <h1 className="text-4xl font-bold text-gray-900">99.9%</h1>
                    <p className="text-gray-500 mt-2">Uptime SLA</p>
        </div>

      </div>

    </div>

            <div className="bg-slate-900 py-12 px-6">
            <div className="flex flex-row items-center justify-between max-w-6xl mx-auto">
                <p className="text-slate-500 text-sm">2024 NWL. All rights reserved.</p>
                <div className="flex flex-row gap-4">
                    <p className="text-slate-500 text-sm hover:text-white">Twitter</p>
                    <p className="text-slate-500 text-sm hover:text-white">GitHub</p>
        </div>

      </div>

    </div>

      </div>
    </>
  );
}

