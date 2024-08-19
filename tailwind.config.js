/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {},
        colors: {
            'bison-hide': '#A28B55',
            'siam-sage': '#86AB89',
            'satin-linen': '#CBE2B5',
            'off-white': '#E7FBE6',
            'blue': '#1fb6ff',
            'purple': '#7e5bef',
            'pink': '#ff49db',
            'orange': '#ff7849',
            'green': '#13ce66',
            'yellow': '#ffc82c',
            'gray-dark': '#273444',
            'gray': '#8492a6',
            'gray-light': '#d3dce6',
            'white': '#ffffff',
            'black': '#000000'
        },
        screens: {
            'sm': '640px',
            'md': '768px',
            'lg': '1024px',
            'xl': '1280px',
        }
    },
    plugins: [],
}
