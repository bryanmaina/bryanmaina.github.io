# Product Requirements Document (PRD)

## Overview

This document outlines the requirements for a lightweight, performant blog website hosted on GitHub Pages. The site will leverage a modern Rust-based tech stack for client-side rendering, styling, and content processing, with an interactive 3D animation feature in the About section.

## Goals

*   Deliver a fast, responsive, and visually appealing blog platform.
*   Enable easy content creation and rendering of Markdown-based blog posts.
*   Provide an engaging user experience with minimal 3D animations in the About section.
*   Ensure compatibility with GitHub Pages' static hosting environment.

## Tech Stack

*   **Leptos:** Rust framework for client-side rendering (CSR) of the blog UI.
*   **Leptos-Use:** Utility library for Leptos to enhance reactivity and state management.
*   **Thaw UI:** Component library for reusable UI elements.
*   **Tailwind CSS:** Utility-first CSS framework for styling.
*   **Bevy:** Rust game engine for 3D animations (About section only).
*   **pulldown-cmark:** Markdown parser for rendering blog post content.
*   **syntect:** Syntax highlighting for code snippets in blog posts.

## 1. Functional Requirements

### 1.1. Blog Post Rendering

*   **1.1.1 Markdown Rendering:** The website must render Markdown files (`.md`) as HTML using `pulldown-cmark`.
*   **1.1.2 Syntax Highlighting:** Blog posts must support syntax highlighting for code blocks using `syntect`.
*   **1.1.3 Post Storage:** Posts must be stored in a designated folder (e.g., `/posts`) and dynamically listed on the homepage.
*   **1.1.4 Post Structure:** Each post must display a title, date, and content, with optional metadata (e.g., tags, author).

### 1.2. Homepage

*   **1.2.1 Post List:** Display a list of blog posts with titles, dates, and short previews (first 100 characters of content).
*   **1.2.2 Navigation Bar:** Include a navigation bar with links to "Home," "About," and "Posts."
*   **1.2.3 UI Components:** Use Thaw UI components for consistent styling of the post list and navigation.

### 1.3. About Section

*   **1.3.1 3D Animation:** Include a 3D animation powered by Bevy, such as a rotating globe or simple interactive object.
*   **1.3.2 Animation Optimization:** Animation must be lightweight and optimized for WebAssembly (WASM) to ensure fast loading on GitHub Pages.
*   **1.3.3 Animation Toggle:** Provide a toggle to disable the animation for accessibility and performance preferences.

### 1.4. Styling

*   **1.4.1 Tailwind CSS:** Use Tailwind CSS for responsive, utility-based styling across all pages.
*   **1.4.2 Responsive Design:** Ensure a mobile-friendly design with breakpoints at 640px, 768px, and 1024px.
*   **1.4.3 Design System:** Maintain a consistent color scheme and typography defined in a Tailwind configuration file.

### 1.5. Client-Side Rendering

*   **1.5.1 Leptos CSR:** Use Leptos for CSR to handle dynamic rendering of blog posts and navigation.
*   **1.5.2 Reactive State:** Leverage Leptos-Use for reactive state management (e.g., tracking the current page or animation state).
*   **1.5.3 Static Assets:** Ensure all dynamic content is pre-rendered or hydrated as static assets for GitHub Pages compatibility.

### 1.6. Deployment

*   **1.6.1 Static Files:** The site must compile to static HTML, CSS, and WASM files deployable on GitHub Pages.
*   **1.6.2 GitHub Actions:** Include a GitHub Actions workflow to automate building and deployment on push to the main branch.
*   **1.6.3 Hosting Location:** Serve the site from the `/docs` folder or root directory, per GitHub Pages requirements.

### 1.7. Other Features

*   **1.7.1 Social Links:** Prominent links to my LinkedIn, X (Twitter), and GitHub profiles.
*   **1.7.2 Career Timeline:** A visually appealing timeline showcasing my career history, including:
    *   Company Name
    *   Company Logo
    *   Start and End Dates
    *   Role/Position
    *   Customizable theme for each company entry.
*   **1.7.3 Personal Photo:** A professional photo of myself.
*   **1.7.4 Passions:** A brief description of my personal interests and passions.
*   **1.7.5 Responsive Design:** The website will be responsive and work well on all screen sizes.
*   **1.7.6 Fast Loading:** The website will be optimized for performance and fast loading times.
*   **1.7.7 Markdown Support:** Blog posts will be written in Markdown for easy formatting and content creation.
*   **1.7.8 Publishing:** A straightforward process for adding new blog posts to the site.
*   **1.7.9 Blog Post Structure:** Each blog post will include:
    *   Title
    *   Publication Date
    *   Content (written in Markdown)
    *   Optional: Tags/Categories
*   **1.7.10 Pagination:** The blog will implement pagination to handle a growing number of posts.
*   **1.7.11 Search:** A search feature will allow users to find specific blog posts.
*   **1.7.12 Categories/Tags:** Blog posts will be categorized and tagged for better organization and discoverability.
*   **1.7.13 Comments:** A comment section may be added in the future, potentially using a third-party service.

## 2. Non-Functional Requirements

### 2.1. Performance

*   **2.1.1 Page Load Time:** Page load time must be under 2 seconds on a 4G connection.
*   **2.1.2 WASM Bundle Size:** Minimize WASM bundle size for Bevy animations to under 500 KB.
*   **2.1.3 Optimization:** Optimize Markdown parsing and syntax highlighting to avoid runtime delays.

### 2.2. Accessibility

*   **2.2.1 Text Contrast:** Ensure all text meets WCAG 2.1 AA contrast ratios.
*   **2.2.2 Keyboard Navigation:** Support keyboard navigation for all interactive elements (e.g., nav links, animation toggle).
*   **2.2.3 Animation Description:** Provide alt text or descriptions for the 3D animation.

### 2.3. Compatibility

*   **2.3.1 Browser Support:** Support modern browsers (Chrome, Firefox, Safari, Edge) with WASM and CSR capabilities.
*   **2.3.2 Graceful Degradation:** Gracefully degrade functionality (e.g., static content) on browsers without WASM support.

## 3. User Stories

*   As a reader, I want to browse a list of blog posts on the homepage so I can quickly find content that interests me.
*   As a developer, I want code snippets in posts to be syntax-highlighted so I can easily read and understand examples.
*   As a visitor, I want to see a unique 3D animation on the About page to make the site stand out, with an option to disable it if it slows my device.
*   As an author, I want to write posts in Markdown and have them automatically rendered on the site without manual HTML conversion.

## 4. Constraints

*   GitHub Pages only supports static hosting, so all dynamic behavior must be client-side via WASM and CSR.
*   Bevy’s 3D features must be limited to the About section to avoid excessive bundle sizes.
*   No server-side processing or database; content must be file-based (e.g., Markdown files in the repo).

## 5. Future Considerations

*   Add a search bar for blog posts using client-side filtering with Leptos.
*   Support RSS feed generation from the Markdown files.
*   Expand Bevy usage for interactive blog post visualizations (if performance allows).
*   **Contact Form:** A form for visitors to contact me.
*   **Newsletter Signup:** An option for users to subscribe to a newsletter.
*   **Dark Mode:** A dark mode theme for the website.
*   **Improved Animations:** More complex and interactive 3D animations.
*   **CMS Integration:** Consider using a headless CMS to manage blog content.
