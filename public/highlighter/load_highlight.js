/*
 * Stolen from @itehax here https://github.com/itehax/rust-blog/blob/f1edd7b656812425ad453f703582f9fece0e5848/public/highlighter/load_highlight.js#L18
 */

function loadScript(scriptUrl) {
  const script = document.createElement("script");
  script.src = scriptUrl;
  script.defer = true;
  document.body.appendChild(script);

  return new Promise((res, rej) => {
    script.onload = function () {
      res();
    };
    script.onerror = function () {
      rej();
    };
  });
}

// use
loadScript("/public/highlighter/highlight.min.js")
  .then(() => {
    hljs.highlightAll();
  })
  .catch(() => {
    console.error("Script loading failed! Handle this error");
  });

loadScript("/public/highlighter/katex.min.js")
  .then(() => {
    loadScript("/public/highlighter/auto-render.min.js")
      .then(() => {
        renderMathInElement(document.body, {
          // customised options
          // • auto-render specific keys, e.g.:
          delimiters: [
            { left: "$$", right: "$$", display: true },
            { left: "$", right: "$", display: false },
            { left: "\\(", right: "\\)", display: false },
            {
              left: "\\begin{equation}",
              right: "\\end{equation}",
              display: true,
            },
            { left: "\\begin{align}", right: "\\end{align}", display: true },
            {
              left: "\\begin{alignat}",
              right: "\\end{alignat}",
              display: true,
            },
            { left: "\\begin{gather}", right: "\\end{gather}", display: true },
            { left: "\\begin{CD}", right: "\\end{CD}", display: true },
            { left: "\\[", right: "\\]", display: true },
          ],
          throwOnError: false,
        });
      })
      .catch(() => {
        console.error("Script loading failed! Handle this error");
      });
  })
  .catch(() => {
    console.error("Script loading failed! Handle this error");
  });
