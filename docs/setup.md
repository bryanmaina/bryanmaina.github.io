# Setup Guide

This guide will help you set up the development environment for Bryan Maina's Developer Blog.

## Prerequisites

*   **Rust:** You need to have Rust installed. If you don't have it, follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   **Cargo:** Cargo is the Rust package manager and build tool. It comes with Rust.
*   **Git:** You'll need Git to clone the repository. [https://git-scm.com/book/en/v2/Getting-Started-Installing-Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
*   **Node.js and npm:** Tailwind CSS requires Node.js and npm. You can download them from [https://nodejs.org/](https://nodejs.org/).

## Installation

1.  **Clone the Repository:**
    ```bash
    git clone <repository_url>
    cd bryanmaina-portfolio
    ```
    Replace `<repository_url>` with the actual URL of your repository.

2.  **Install Dependencies:**
    ```bash
    cargo install trunk
    ```
    This will install the necessary Rust crates and Node.js packages.

## Running the Project

1.  **Start the Development Server:**
    ```bash
    trunk serve
    ```
    This command will start a development server and watch for changes in your code.

2.  **Open in Browser:**
    Open your web browser and go to `http://127.0.0.1:8080` (or the address provided by Trunk).

## Building for Production

1.  **Build the Project:**
    ```bash
    trunk build --release
    ```
    This command will build the project in release mode, optimizing it for production.

2. **Deploy**
    The content of the `dist` folder can be deployed to github pages.

## Troubleshooting

*   If you encounter any issues, please refer to the documentation for the specific tools you are using (Rust, Cargo, Trunk, Node.js, npm).
*   If you still have problems, feel free to open an issue on the GitHub repository.
