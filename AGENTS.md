# Agent Guidelines

This document outlines the conventions and commands for working in this repository.

## Build/Lint/Test Commands

- **Build:** `turbo run build` or `turbo run cb` (chill-build)
- **Lint:** `turbo run lint`
- **Typecheck:** `turbo run typecheck`
- **Format:** `turbo run format` or `turbo run fmt`
- **Check:** `turbo run check` (runs lint + typecheck)
- **Dev:** `turbo run dev`
- **Tests:** `bun setup-tests.ts projektwoche` (setup tests only)

## Code Style Guidelines

- **JavaScript/TypeScript:**
  - Use `eslint` for linting. Config in `packages/eslint-config/`.
  - Use `prettier` for formatting.
  - Prefer type imports: `import { type Foo } from "bar"`
  - Unused variables must start with `_`
  - No explicit `any` types allowed
  - Use bun as package manager
- **Rust:**
  - Use `rustfmt` for formatting. Config: `rustfmt.toml`
  - Use 2 spaces for indentation
- **General:**
  - Follow existing project conventions for naming and error handling
  - Environment variables validated via t3-env
