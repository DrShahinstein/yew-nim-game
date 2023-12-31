module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "src/**/*.css", "**/*.html"],
  },
  darkMode: "class",
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
