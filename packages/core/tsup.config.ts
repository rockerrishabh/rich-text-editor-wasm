// packages/core/tsup.config.ts
// Configuration for tsup to handle WASM files

import { defineConfig } from "tsup";
import { copyFileSync, mkdirSync, existsSync, readdirSync } from "fs";
import { join } from "path";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["cjs", "esm"],
  dts: true,
  clean: true,
  sourcemap: true,
  // Don't bundle the WASM JS glue code - keep it as external
  external: [],
  // Copy WASM files after build
  onSuccess: async () => {
    console.log("Copying WASM files to dist...");

    const wasmDir = join(__dirname, "wasm");
    const distWasmDir = join(__dirname, "dist", "wasm");

    // Create dist/wasm directory
    if (!existsSync(distWasmDir)) {
      mkdirSync(distWasmDir, { recursive: true });
    }

    // Copy all files from the wasm directory into dist/wasm
    try {
      const files = readdirSync(wasmDir);
      for (const file of files) {
        const src = join(wasmDir, file);
        const dest = join(distWasmDir, file);
        if (existsSync(src)) {
          copyFileSync(src, dest);
          console.log(`Copied ${file}`);
        }
      }
    } catch (err) {
      const error = err as Error;
      console.warn("No wasm files found to copy:", error?.message || err);
    }

    console.log("WASM files copied successfully!");
  },
  // Handle .wasm imports
  loader: {
    ".wasm": "copy",
  },
});
