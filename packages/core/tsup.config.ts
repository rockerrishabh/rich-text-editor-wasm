// packages/core/tsup.config.ts
// Configuration for building the library with tsup

import { defineConfig } from "tsup";
import { copyFileSync, mkdirSync, existsSync, readdirSync } from "fs";
import { join } from "path";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["cjs", "esm"],
  dts: true,
  clean: true,
  sourcemap: true,
  minify: false, // Keep readable for debugging, npm will minify if needed
  treeshake: true, // Enable tree-shaking for smaller bundles
  splitting: false, // Don't split for library builds
  // Don't bundle dependencies - they should be peer deps or externals
  external: [],
  // Copy WASM files after build
  onSuccess: async () => {
    console.log("📦 Copying WASM files to dist...");

    const wasmDir = join(__dirname, "wasm");
    const distWasmDir = join(__dirname, "dist", "wasm");

    // Create dist/wasm directory
    if (!existsSync(distWasmDir)) {
      mkdirSync(distWasmDir, { recursive: true });
    }

    // Copy all files from the wasm directory into dist/wasm
    try {
      const files = readdirSync(wasmDir);
      let copiedCount = 0;
      
      for (const file of files) {
        const src = join(wasmDir, file);
        const dest = join(distWasmDir, file);
        if (existsSync(src)) {
          copyFileSync(src, dest);
          copiedCount++;
        }
      }
      
      console.log(`✅ Copied ${copiedCount} WASM file(s) successfully!`);
    } catch (err) {
      const error = err as Error;
      console.warn("⚠️  No WASM files found to copy:", error?.message || err);
    }
  },
  // Handle .wasm imports
  loader: {
    ".wasm": "copy",
  },
});
