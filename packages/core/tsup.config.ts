// packages/core/tsup.config.ts
// Configuration for building the library with tsup

import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["cjs", "esm"],
  dts: false,
  clean: true,
  sourcemap: true,
  minify: false, // Keep readable for debugging, npm will minify if needed
  treeshake: true, // Enable tree-shaking for smaller bundles
  splitting: false, // Don't split for library builds
  // Don't bundle dependencies - they should be peer deps or externals
  external: [],
  // Handle .wasm imports
  loader: {
    ".wasm": "copy",
  },
});
