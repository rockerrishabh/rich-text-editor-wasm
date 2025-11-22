# Rich Text Editor - Rust + WASM

A high-performance rich text editor built with Rust and compiled to WebAssembly, with bindings for React, Solid, and Svelte.

## Features

- 🚀 High performance text editing with Rust
- 📦 Small bundle size (< 100KB gzipped)
- 🎨 Rich text formatting (bold, italic, underline, etc.)
- 📝 Block-level formatting (headings, lists, quotes)
- ↩️ Undo/redo support
- 🔍 Search and replace
- 💾 Multiple export formats (JSON, Markdown, HTML)
- 🌐 Modern browser support
- 📱 Mobile-friendly
- フレームワーク Framework-agnostic core with bindings for React, Solid, and Svelte

## Project Structure

```
.
├── packages/
│   ├── core/              # Core editor logic (TypeScript)
│   ├── wasm/              # Rust source code for the WASM module
│   ├── react/             # React bindings
│   ├── solid/             # Solid bindings
│   └── svelte/            # Svelte bindings
├── examples/
│   ├── react/             # React example
│   ├── solid/             # Solid example
│   ├── svelte/            # Svelte example
│   └── vanilla/           # Vanilla JS example
├── Cargo.toml            # Rust dependencies
└── package.json          # Build scripts
```

## Development

### Prerequisites

- Rust (latest stable)
- wasm-pack
- Node.js (v18+)
- pnpm

### Building the Project

```bash
# Install dependencies
pnpm install

# Build all packages (including WASM)
pnpm run build
```

### Running the Examples

To run any of the examples, navigate to the example's directory and run the `dev` script.

For example, to run the React example:
```bash
cd examples/react
pnpm install
pnpm run dev
```

The examples will be available at `http://localhost:5173` (the port may vary).

## Browser Support

The Rich Text Editor has been tested and verified to work on the following browsers:

### Desktop Browsers

| Browser | Minimum Version |
| ------- | --------------- |
| Chrome  | 90+             |
| Firefox | 88+             |
| Safari  | 14+             |
| Edge    | 90+             |

### Mobile Browsers

| Browser       | Minimum Version |
| ------------- | --------------- |
| iOS Safari    | 14+             |
| Chrome Mobile | 90+             |

## License

MIT OR Apache-2.0

## Status

🚧 This project is currently under development.
