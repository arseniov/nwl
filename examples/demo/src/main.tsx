import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import './index.css'

import Home from './home';
import Landing from './landing';
import Counter from './counter';
import Interactive from './interactive';


ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />

        <Route path="/landing" element={<Landing />} />

        <Route path="/counter" element={<Counter />} />

        <Route path="/interactive" element={<Interactive />} />

      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
)
