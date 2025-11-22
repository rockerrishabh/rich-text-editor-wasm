# Contributing to Rich Text Editor

Thank you for your interest in contributing! This document provides guidelines for contributing to this monorepo.

## Development Setup

### Prerequisites

- Node.js >= 16
- pnpm >= 8
- Rust (latest stable)
- wasm-pack

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/rockerrishabh/rich-text-editor-wasm.git
cd rich-text-editor-wasm

# Install dependencies
pnpm install

# Build all packages
pnpm run build

# Run tests
pnpm run test
```

## Project Structure

```
rich-text-editor-wasm/
├── packages/
│   ├── wasm/          # Rust WASM core
│   ├── core/          # TypeScript wrapper
│   ├── react/         # React bindings
│   ├── solid/         # Solid bindings
│   └── svelte/        # Svelte bindings
├── examples/          # Example applications
└── scripts/           # Build and publish utilities
```

## Development Workflow

### Making Changes

1. Create a new branch from `main`:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and ensure:

   - Code follows existing style
   - Tests pass: `pnpm run test`
   - Type checking passes: `pnpm run typecheck`
   - Linting passes: `pnpm run lint`

3. Build to verify:
   ```bash
   pnpm run build
   ```

### Running in Development Mode

```bash
# Watch mode for all packages
pnpm run dev

# Watch specific package
cd packages/core
pnpm run dev
```

### Testing

```bash
# Run all tests
pnpm run test

# Run tests in watch mode
pnpm run test:watch

# Run tests for specific package
cd packages/react
pnpm run test
```

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body (optional)

footer (optional)
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**

```
feat(core): add support for inline code formatting
fix(react): resolve memory leak in useEditor hook
docs: update installation instructions
chore: bump dependencies
```

## Pull Request Process

1. **Fork and Clone**: Fork the repo and clone your fork

2. **Create Branch**: Create a feature branch from `main`

3. **Make Changes**: Implement your changes with tests

4. **Run Checks**: Ensure all checks pass:

   ```bash
   pnpm run lint
   pnpm run typecheck
   pnpm run test
   pnpm run build
   ```

5. **Commit**: Use conventional commit messages

6. **Push**: Push to your fork

7. **Create PR**: Open a pull request with:

   - Clear description of changes
   - Link to related issues
   - Screenshots/videos for UI changes

8. **Code Review**: Address review feedback

9. **Merge**: Maintainers will merge when approved

## Code Style

- **TypeScript**: Use TypeScript for type safety
- **Formatting**: Will be handled by prettier (if configured)
- **Linting**: Follow ESLint rules
- **Naming**: Use descriptive, camelCase names

## Package-Specific Guidelines

### WASM (Rust)

- Follow Rust best practices
- Add tests for new functionality
- Update Cargo.toml if adding dependencies

### Core (TypeScript)

- Maintain browser compatibility
- Keep bundle size small
- Export only necessary APIs

### Framework Bindings (React/Solid/Svelte)

- Follow framework conventions
- Provide TypeScript definitions
- Include usage examples

## Documentation

- Update README.md if adding features
- Add JSDoc comments for public APIs
- Update examples if behavior changes

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues first

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
