# About the Project Week "Nachhaltige Webentwicklung"

This document provides a summary of the project week "Nachhaltige Webentwicklung" (Sustainable Web Development) at the Gymnasium Athenaeum Stade, organized by Hackclub Stade in cooperation with the DJL Foundation.

## Overview

The project week is designed to introduce students to the world of web development. The main goal is for each student to create two web projects: a personal website and a website focused on ideas for making IT and web technologies more sustainable.

All student projects are published under the domain [prowo.hackclub-stade.de](https://prowo.hackclub-stade.de).

## Repository Structure

This monorepo hosts all the necessary tools and applications for the project week.

### Web Applications (`/apps`)

*   **`web/`**: This is the main website for the project week, available at [prowo.hackclub-stade.de](https://prowo.hackclub-stade.de). It includes a landing page, a download page for the setup CLI, and a team page.
*   **`projekte/`**: This application serves as a microfrontend for the student projects. It uses an intelligent routing system to display the projects created by the students.

### CLI Tools

*   **Setup CLI (`/rust/projektwoche-setup`)**: A cross-platform command-line tool written in Rust. It simplifies the setup of the development environment by installing necessary tools like Node.js, Bun, and VS Code.
*   **Tutorial CLI (Planned)**: An interactive CLI to teach the basics of HTML, CSS, and JavaScript.

### Shared Packages (`/packages`)

This directory contains shared code and configurations used across the monorepo:

*   **`ui/`**: A collection of shared React components.
*   **`eslint-config/`**: Shared ESLint configurations.
*   **`typescript-config/`**: Shared TypeScript configurations.

## Technology Stack

The project utilizes a modern technology stack:

*   **Frontend**: Next.js 15, React 19, TypeScript, TailwindCSS
*   **Backend**: Vercel Microfrontends Architecture
*   **CLI Tools**: Rust (for the setup tool) and TypeScript/Node.js (for the planned tutorial tool)
*   **Package Manager**: Bun
*   **Monorepo Management**: Turbo
*   **Hosting**: Vercel

## Student Projects

The `projects.json` file contains a list of the students participating in the project week and their assigned projects. For the year 2025, the following students are listed:

*   Julian
*   Emil
*   Paul
*   Claas
*   Toni
*   Nante
*   Norwin
*   Theo
