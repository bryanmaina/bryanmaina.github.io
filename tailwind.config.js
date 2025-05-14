/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["index.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  plugins: [],
};
