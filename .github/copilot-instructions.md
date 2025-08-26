# Projektwoche Monorepo Development Guide

**ALWAYS follow these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Bootstrap and Setup
Bootstrap the development environment with these exact steps:

```bash
# Install Bun package manager (if not available)
curl -fsSL https://bun.sh/install | bash
export PATH="$HOME/.bun/bin:$PATH"

# Install dependencies
bun install
# Takes ~50 seconds. NEVER CANCEL. Set timeout to 120+ seconds.

# Create environment files for development
cat > .env << EOF
SKIP_ENV_VALIDATION=true
NODE_ENV=development
NEXT_PUBLIC_POSTHOG_KEY=dummy_key
NEXT_PUBLIC_POSTHOG_HOST=https://dummy.host
NEXT_PUBLIC_POSTHOG_PROJECT_ID=dummy_project_id
EOF

cat > apps/web/.env << EOF
SKIP_ENV_VALIDATION=true
NODE_ENV=development
NEXT_PUBLIC_POSTHOG_KEY=dummy_key
NEXT_PUBLIC_POSTHOG_HOST=https://dummy.host
NEXT_PUBLIC_POSTHOG_PROJECT_ID=dummy_project_id
EOF
```

### Build and Test Commands
Always run these commands with proper timeouts:

```bash
# Typecheck (fastest validation)
bunx turbo run typecheck
# Takes ~6 seconds. Set timeout to 30+ seconds.

# Lint all packages
bunx turbo run lint  
# Takes ~8 seconds. Set timeout to 60+ seconds.

# Quick validation build (recommended for development)
bunx turbo run chill-build
# Takes ~1 second when cached. Set timeout to 30+ seconds.

# Full production build (may fail due to network dependencies)
bunx turbo run build
# WARNING: May fail due to Google Fonts network issues. Use chill-build instead.
# Takes ~17 seconds when working. NEVER CANCEL. Set timeout to 120+ seconds.

# Format code
bunx turbo run format

# Combined lint + typecheck
bunx turbo run check
```

### Development Servers
Run development servers for each app individually:

```bash
# Main web application (Next.js)
cd apps/web && bun run dev
# Runs on http://localhost:3900
# Takes ~2 seconds to start. Ready when you see "✓ Ready in XXXXms"

# Projects microfrontend (Astro)  
cd apps/projekte && bun run dev
# Runs on http://localhost:3901
# Takes ~1 second to start. Ready when you see "astro ready in XXXms"
```

**DO NOT** run `bunx turbo run dev` from the root - it has microfrontend configuration issues.

### Setup Tests
Validate your environment setup:

```bash
bun scripts/setup-tests.ts projektwoche
# Takes ~0.02 seconds. Tests Node.js, Bun, and VSCode availability.
# VSCode test will fail in CI environments - this is expected.
```

### Rust Tools (Setup CLI)
Build and test the Rust setup CLI:

```bash
# Build Rust tools (bypass workspace issues)
cd rust/projektwoche-setup && cargo build --release
# Takes ~7 seconds. NEVER CANCEL. Set timeout to 300+ seconds.

# Test the binary (currently minimal)
./target/release/projektwoche-setup --help
# Currently outputs "Hello, world!" - this is expected.
```

## Known Issues and Workarounds

### Build Issues
- **Full build may fail**: Google Fonts network dependency in Next.js can cause build failures. Use `bunx turbo run chill-build` instead.
- **Microfrontend dev server**: `bunx turbo run dev` fails due to microfrontend configuration. Run apps individually instead.
- **Rust workspace**: Root cargo commands fail. Build Rust projects from their individual directories.

### Environment Variables  
**CRITICAL**: Always set these environment variables or builds will fail:

```bash
SKIP_ENV_VALIDATION=true
NEXT_PUBLIC_POSTHOG_KEY=dummy_key (for development)
NEXT_PUBLIC_POSTHOG_HOST=https://dummy.host (for development) 
NEXT_PUBLIC_POSTHOG_PROJECT_ID=dummy_project_id (for development)
```

## Validation Scenarios

### After Making Changes - ALWAYS Run These
1. **Quick validation**: `bunx turbo run chill-build && bunx turbo run check`
2. **Test individual apps**: Start each dev server and verify they load without errors
3. **Environment test**: `bun scripts/setup-tests.ts projektwoche` to verify core tools

### Manual Testing Scenarios
When making changes to the web applications:

1. **Web App Testing**:
   - Start: `cd apps/web && bun run dev`
   - Navigate to http://localhost:3900
   - Verify: Landing page loads without errors
   - Check: Console for any JavaScript errors

2. **Projects Microfrontend Testing**:
   - Start: `cd apps/projekte && bun run dev`  
   - Navigate to http://localhost:3901
   - Verify: Project listing page loads
   - Check: Individual project pages (e.g., /projekte/jack/boba-beispiel/)

3. **Package Changes Testing**:
   - Run `bunx turbo run typecheck` after modifying shared packages
   - Restart dependent apps to pick up changes

## Repository Structure

### Key Directories
- **`apps/web/`** - Main Next.js website (prowo.hackclub-stade.de)
- **`apps/projekte/`** - Astro microfrontend for student projects
- **`packages/`** - Shared React components, ESLint configs, TypeScript configs
- **`rust/projektwoche-setup/`** - Rust CLI tool for environment setup
- **`scripts/`** - Build and test automation scripts

### Important Files
- **`turbo.json`** - Turborepo configuration with task dependencies
- **`AGENTS.md`** - Development workflow and code style guidelines
- **`package.json`** - Root workspace configuration (Bun package manager)
- **`.env`** files - Environment variables for development

## Build Performance
- **Dependencies install**: ~50 seconds
- **Typecheck**: ~6 seconds  
- **Lint**: ~8 seconds
- **Chill-build**: ~1 second (cached)
- **Rust build**: ~7 seconds
- **Setup tests**: ~0.02 seconds

## Technology Stack
- **Package Manager**: Bun (not npm/yarn)
- **Build System**: Turborepo
- **Frontend**: Next.js 15, Astro, React 19, TypeScript
- **Styling**: TailwindCSS 4.1
- **Backend Tools**: Rust for CLI utilities
- **Environment**: Node.js 18+, Bun 1.2+

## Troubleshooting

### Command Not Found Errors
- Ensure Bun is in PATH: `export PATH="$HOME/.bun/bin:$PATH"`
- Use `bunx` prefix for all turbo commands: `bunx turbo run lint`

### Build Failures
- Check environment variables are set correctly
- Try `bunx turbo run chill-build` instead of full build
- Clear cache: `rm -rf .turbo node_modules/.cache`

### Development Server Issues
- Don't use `bunx turbo run dev` - run apps individually
- Check ports 3900 (web) and 3901 (projekte) are available
- Verify .env files exist in root and apps/web/

**Always build and exercise your changes manually before considering them complete.**