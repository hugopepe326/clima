/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        navy: {
          950: "#050b18",
          900: "#071126",
          850: "#0a1730",
          800: "#0d1d3a",
          750: "#102444",
          700: "#153052",
          600: "#1d4472",
          500: "#2a5d96",
        },
        ice: {
          50: "#f2f8ff",
          100: "#dfedff",
          200: "#c2ddff",
          300: "#93c5fd",
          400: "#60a5fa",
          500: "#3b82f6",
        },
        aurora: "#67e8f9",
        mint: "#6ee7b7",
        sun: "#fbbf24",
      },
      fontFamily: {
        sans: ["Inter", "Roboto Flex", "system-ui", "sans-serif"],
        display: ["Roboto Flex", "Inter", "system-ui", "sans-serif"],
      },
      borderRadius: {
        "4xl": "2rem",
      },
      boxShadow: {
        glass: "0 24px 80px -24px rgba(2, 8, 23, 0.9)",
        glow: "0 0 60px -12px rgba(96, 165, 250, 0.45)",
        card: "inset 0 1px 0 0 rgba(255,255,255,0.06), 0 20px 55px -28px rgba(0,0,0,0.9)",
      },
      keyframes: {
        drift: {
          "0%, 100%": { transform: "translateX(-2px)" },
          "50%": { transform: "translateX(3px)" },
        },
        bob: {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(-4px)" },
        },
        spinSlow: {
          to: { transform: "rotate(360deg)" },
        },
        rain: {
          "0%": { transform: "translateY(-6px)", opacity: "0" },
          "20%": { opacity: "1" },
          "100%": { transform: "translateY(14px)", opacity: "0" },
        },
        snow: {
          "0%": { transform: "translate(0,-4px)", opacity: "0" },
          "30%": { opacity: "1" },
          "100%": { transform: "translate(3px,12px)", opacity: "0" },
        },
        flash: {
          "0%, 84%, 100%": { opacity: "0.25" },
          "86%, 90%": { opacity: "1" },
          "88%": { opacity: "0.4" },
        },
        float: {
          "0%, 100%": { transform: "translate3d(0,0,0) scale(1)" },
          "50%": { transform: "translate3d(0,-18px,0) scale(1.04)" },
        },
        shimmer: {
          "100%": { transform: "translateX(100%)" },
        },
        fadeUp: {
          "0%": { opacity: "0", transform: "translateY(10px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
        pulseSoft: {
          "0%, 100%": { opacity: "0.6" },
          "50%": { opacity: "1" },
        },
      },
      animation: {
        drift: "drift 7s ease-in-out infinite",
        bob: "bob 5s ease-in-out infinite",
        "spin-slow": "spinSlow 24s linear infinite",
        rain: "rain 1.4s linear infinite",
        snow: "snow 3s linear infinite",
        flash: "flash 4s ease-in-out infinite",
        float: "float 14s ease-in-out infinite",
        shimmer: "shimmer 1.6s infinite",
        "fade-up": "fadeUp 0.5s ease-out both",
        "pulse-soft": "pulseSoft 2.6s ease-in-out infinite",
      },
    },
  },
  plugins: [],
};
