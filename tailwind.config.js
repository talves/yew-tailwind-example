const colors = require("tailwindcss/colors");

module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  content: ["./styles.css", "./index.html", "./src/**/*.{html,rs,css}"],
  theme: {
    colors: {
      transparent: "transparent",
      gray: {
        100: "#f7fafc",
        200: "#edf2f7",
        300: "#e2e8f0",
        400: "#cbd5e0",
        500: "#a0aec0",
        600: "#718096",
        700: "#4a5568",
        800: "#2d3748",
        900: "#1a202c",
      },
      primary: {
        50: "#f5fafb",
        100: "#ecf5f7",
        200: "#cfe7ec",
        300: "#b2d9e0",
        400: "#79bcc9",
        500: "#3f9fb2",
        600: "#398fa0",
        700: "#2f7786",
        800: "#265f6b",
        900: "#1f4e57",
      },
      secondary: {
        50: "#fef8f4",
        100: "#fdf1e9",
        200: "#f9dcc9",
        300: "#f6c7a8",
        400: "#ef9e67",
        500: "#e87426",
        600: "#d16822",
        700: "#ae571d",
        800: "#8b4617",
        900: "#723913",
      },
    },
  },
  variants: {},
  plugins: [],
};
