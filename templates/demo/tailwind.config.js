/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
    './themes/**/*.css',
  ],
  theme: {
    extend: {
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        'slide-in': {
          '0%': { opacity: '0', transform: 'translate(-50%, -48%)' },
          '100%': { opacity: '1', transform: 'translate(-50%, -50%)' },
        },
        'spin': {
          'to': { transform: 'rotate(360deg)' },
        },
      },
      animation: {
        'fade-in': 'fade-in 150ms ease-out',
        'slide-in': 'slide-in 200ms ease-out',
        'spin': 'spin 600ms linear infinite',
      },
    },
  },
  plugins: [],
}
