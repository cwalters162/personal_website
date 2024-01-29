import type { Config } from "tailwindcss"
export default {
  content: ["./index.html", "./src/**/*.{html,js,ts,tsx,jsx}"],
  theme: {
    extend: {
      colors: {
        dark: "#242424",
        light: "#f4f3f2",
        "primary-dark": "#4ADE30"
      }
    }
  },
  plugins: []
} satisfies Config
