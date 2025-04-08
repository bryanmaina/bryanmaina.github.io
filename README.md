<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Bryan Maina's Developer Blog

This is a personal developer blog built with the [Leptos](https://github.com/leptos-rs/leptos) web framework, showcasing my skills and journey as a software developer. The blog is designed to be lightweight, performant, and visually appealing, leveraging a modern Rust-based tech stack.

For a detailed list of features and project requirements, please refer to the [Product Requirements Document (PRD)](./docs/product_requirements.md#1-functional-requirements).

## Tech Stack

*   **[Leptos](https://github.com/leptos-rs/leptos):** Rust framework for client-side rendering (CSR).
*   **[Leptos-Use](https://github.com/Synphonyte/leptos-use):** Utility library for Leptos to enhance reactivity and state management.
*   **[Thaw UI](https://github.com/thaw-ui/thaw):** Component library for reusable UI elements.
*   **[Tailwind CSS](https://github.com/tailwindlabs/tailwindcss):** Utility-first CSS framework for styling, using the standalone CLI.
*   **[Bevy](https://github.com/bevyengine/bevy):** Rust game engine for 3D animations (in the About section).
*   **[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark):** Markdown parser for rendering blog post content.
*   **[syntect](https://github.com/trishume/syntect):** Syntax highlighting for code snippets in blog posts.
*   **[Trunk](https://github.com/trunk-rs/trunk):** Build tool for the project.
*   **[Make](https://www.gnu.org/software/make/):** Utility for automating build processes.

## Getting Started

### Prerequisites

*   **Rust:** Ensure you have Rust installed. If not, follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   **Cargo:** Cargo is the Rust package manager and build tool. It comes with Rust.
*   **Git:** You'll need Git to clone the repository. [https://git-scm.com/book/en/v2/Getting-Started-Installing-Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
*   **Node.js and npm:** Required for Tailwind CSS and `concurrently`. You can download them from https://nodejs.org/.
*   **Make:** A build automation tool. It's typically pre-installed on Linux and macOS. Windows users might need to install it (e.g., via Chocolatey or WSL).

### Installation

1.  **Clone the Repository:**

    ```bash
    git clone <repository_url>
    cd bryanmaina-portfolio
    ```

    Replace `<repository_url>` with the actual URL of the repository.

2.  **Install Dependencies:**

    ```bash
    make install
    ```
    This command installs Trunk, Node.js packages (including `concurrently`), and any other necessary dependencies defined in the `Makefile`.

### Running the Project

1.  **Start Development Servers:**

    ```bash
    make dev
    ```
    This command uses `concurrently` (installed via `make install`) to run both the Tailwind CSS watcher and the Trunk development server in the same terminal.

2.  **Open in Browser:**

    Open your web browser and go to `http://127.0.0.1:8080` (or the address provided by Trunk).

### Building for Production

1.  **Build the Project:**
    ```bash
    make build
    ```
    This command handles both Tailwind CSS generation (minified) and the Trunk build process. The optimized output will be placed in the `./dist` directory.

2.  **Deploy**
    The content of the `dist` folder is ready for deployment to GitHub Pages.

    **Note:** Deployment to GitHub Pages happens automatically when you push to the `main` branch via the configured GitHub Actions workflow.

## Additional Tools

*   **Rust Nightly:** This project uses Rust nightly.
*   **wasm32-unknown-unknown:** The WebAssembly target is required.
*   **cargo-generate:** Used for generating new projects.
*   **Tailwind CSS CLI:** Used for styling.
*   **concurrently:** Used by `make dev` to run processes in parallel.

## Testing

Run the tests using:
```bash
make test
