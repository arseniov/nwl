import React from 'react';
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

export default function Home() {
  return (
    <>
      <div className="bg-white">
            <div className="bg-gray-900 text-white py-20">
            <div className="flex flex-col items-center max-w-4xl mx-auto px-6">
                <h1 className="text-5xl font-bold mb-6 text-center">NWL - Natural Web Language</h1>
                <p className="text-xl text-gray-300 text-center mb-8">A DSL that compiles declarative YAML to production-ready React and React Native code</p>
                <div className="flex flex-row gap-4">
                    <Button className="Button bg-blue-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-700" onClick={() => window.location.href = "/playground"}>Get Started</Button>
                    <Button className="Button bg-gray-800 text-white px-8 py-3 rounded-lg font-semibold border border-gray-700 hover:bg-gray-700">View on GitHub</Button>
        </div>

                <div className="flex flex-row gap-8 mt-12">
                    <div className="flex flex-col items-center">
                        <p className="text-3xl font-bold text-blue-400">YAML</p>
                        <p className="text-gray-400 text-sm">Declarative Syntax</p>
          </div>

                    <div className="flex flex-col items-center">
                        <p className="text-3xl font-bold text-orange-500">Rust</p>
                        <p className="text-gray-400 text-sm">Blazing Fast</p>
          </div>

                    <div className="flex flex-col items-center">
                        <p className="text-3xl font-bold text-cyan-400">React</p>
                        <p className="text-gray-400 text-sm">Production Ready</p>
          </div>

        </div>

      </div>

    </div>

            <div className="py-16 px-6">
            <div className="flex flex-col max-w-4xl mx-auto">
                <h1 className="text-3xl font-bold text-gray-900 mb-8 text-center">Why NWL?</h1>
                <div className="flex flex-row gap-8 flex-wrap">
                    <div className="Card flex-1 p-6 border border-gray-200 rounded-xl min-w-280">
                        <p className="text-xl font-semibold text-gray-900 mb-3">Simple & Declarative</p>
                        <p className="text-gray-600">Write UI in plain YAML. No JavaScript knowledge required. Focus on what to build, not how to build it.</p>
          </div>

                    <div className="Card flex-1 p-6 border border-gray-200 rounded-xl min-w-280">
                        <p className="text-xl font-semibold text-gray-900 mb-3">Type-Safe Output</p>
                        <p className="text-gray-600">Generate production-ready TypeScript/React code. Full type inference and compile-time validation.</p>
          </div>

                    <div className="Card flex-1 p-6 border border-gray-200 rounded-xl min-w-280">
                        <p className="text-xl font-semibold text-gray-900 mb-3">Fast Compilation</p>
                        <p className="text-gray-600">Built with Rust for instant compilation. Regenerate your entire UI in milliseconds.</p>
          </div>

        </div>

      </div>

    </div>

            <div className="bg-gray-50 py-16 px-6">
            <div className="flex flex-col max-w-4xl mx-auto">
                <h1 className="text-3xl font-bold text-gray-900 mb-8 text-center">How It Works</h1>
                <div className="Card bg-gray-900 rounded-xl p-6 overflow-x-auto">
                    <div className="flex flex-col gap-2">
                        <p className="text-green-400 font-mono text-sm">1. Write simple YAML</p>
                        <p className="text-blue-400 font-mono text-sm">2. Run the NWL compiler</p>
                        <p className="text-yellow-400 font-mono text-sm">3. Get production React code</p>
          </div>

        </div>

                <p className="text-gray-600 text-center mt-6 max-w-2xl">The compiler automatically generates useState hooks, event handlers, and proper JSX from your declarative YAML.</p>
      </div>

    </div>

            <div className="py-16 px-6">
            <div className="flex flex-col max-w-4xl mx-auto items-center">
                <h1 className="text-3xl font-bold text-gray-900 mb-4">Try It Yourself</h1>
                <p className="text-gray-600 mb-8">Explore our interactive playground with all available components</p>
                <Button className="Button bg-blue-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-700" onClick={() => window.location.href = "/playground"}>Open Playground</Button>
      </div>

    </div>

            <div className="bg-gray-900 text-white py-12">
            <div className="flex flex-col max-w-4xl mx-auto px-6 items-center">
                <p className="text-gray-400">Built with Rust + React</p>
                <p className="text-gray-500 text-sm mt-2">NWL Project - Open Source on GitHub</p>
      </div>

    </div>

      </div>
    </>
  );
}

