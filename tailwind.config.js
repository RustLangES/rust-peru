/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {
      colors: {
        "orange-oxided": {
          950: '#450F05',
          900: '#7F260F',
          800: '#9E2A0E',
          700: '#C73407',
          600: '#F04906',
          500: '#FF6E1F',
          400: '#FF8637',
          300: '#FFB270',
          200: '#FFD2A8',
          100: '#FFEBD4',
          50:  '#FFF6ED',
        },
        electricViolet: {
          950: '#240F66',
          900: '#3E1C96',
          800: '#4A20B7',
          700: '#5927DA',
          600: '#6637EE',
          500: '#7A5BF7',
          400: '#9A8AFB',
          300: '#BCB5FD',
          200: '#D9D6FE',
          100: '#EBE9FE',
          50:  '#F4F3FF',
        },
        sharkGray: {
          950: '#222222',
          900: '#3d3d3d',
          800: '#454545',
          700: '#4f4f4f',
          600: '#5d5d5d',
          500: '#6d6d6d',
          400: '#888888',
          300: '#b0b0b0',
          200: '#d1d1d1',
          100: '#e7e7e7',
          50:  '#f6f6f6',
        },
        palette: {
          secondary: "#FAF3E1",
          secondary2: "#F5E6C9",
          primary: "#FF6711",
          accent: "#6637EE",
          neutral: "#222222",
        }
      },
      fontFamily: {
        'akira': ['Akira Expanded', 'sans-serif'],
        'fira': ['Fira Sans', 'sans-serif'],
        'inter': ['Inter', 'sans-serif'],
      },
      backgroundImage: {
        'machu-picchu': "url('/background2.jpg')",
        'peru-flag': "url('/peru2.png')",
      }
    },
  },
  plugins: [],
}

