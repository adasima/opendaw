/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                theme: {
                    // Primary accent
                    skin: '#C3917D',
                    'skin-shadow': '#785046',
                    // Dark elements
                    hair: '#281E1E',
                    shirt: '#32324B',
                    // Backgrounds
                    bg: '#9BC3BE', // Teal/Blinds - accent background
                    wall: '#E6E6E6', // Main background
                    'wall-light': '#E6E6E6',
                    'wall-shadow': '#A0A09B',
                }
            }
        },
    },
    plugins: [],
}

