/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {
      colors: {
        "lc-yellow": "#FFE100",
        "lc-green": "#153119",
      },
      aria: {
        current: "current=page",
      },
    },
    fontFamily: {
      "cooper-hewitt": ['"Cooper Hewitt"', '"Open Sans"'],
      carousel: ['"Carousel"', '"Open Sans"'],
      inter: ['"Inter"', '"Open Sans"'],
    },
  },
  plugins: [],
};
