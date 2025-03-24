/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'selector',
    mode: "all",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        extend: {
            keyframes: {
                down: {
                    '0%': { transform: 'translateY(0%)' },
                    '100%': { transform: 'translateY(calc(45vh - 8rem))' }
                },
                shimmer: {
                    '0%': {
                        transform: 'translateX(-100%)'
                    },
                    '100%': {
                        transform: 'translateX(100%)'
                    }
                }
            },
            animation: {
                'move-down': 'down 3s linear infinite',
                'shimmer': 'shimmer 2s cubic-bezier(0.4, 0, 0.6, 1) infinite'
            },
            boxShadow: {
                neumorphic: '20px 20px 60px #d9d9d9, -20px -20px 60px #ffffff, 0 4px 6px -1px rgba(255, 255, 255, 0.1), 0 2px 4px -1px rgba(255, 255, 255, 0.06), 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
            },
            backgroundImage: theme => ({
                'gradient-bg': 'linear-gradient(to right, #1A202C, black)', // bg-gray-950 is #1F2937
            }),
            colors: {
                'background': {
                    DEFAULT: '#121212',
                    'light': '#FFFFFF'
                },
                'primary': {
                    DEFAULT: '#BB86FC',
                    'light': '#60a5fa',
                    'dark': '#3b82f6',
                    'hover': '#147197253'
                },
                'secondary': {
                    DEFAULT: '#03A9F4',
                    'light': '#e5e7eb',
                    'dark': '#374151'
                },
                'accent': {
                    'purple': {
                        DEFAULT: '#C084FC',
                        'hover': '#D8B4FE'
                    }
                },
                'surface': {
                    DEFAULT: '#1F2937',
                    'light': '#374151',
                    'dark': '#111827',
                    'hover': '#111827B3'
                },
                'text': {
                    'primary': '#FFFFFF',
                    'secondary': '#D1D5DB',
                    'muted': '#9CA3AF'
                }
            }
        },
    },
    plugins: [],
};