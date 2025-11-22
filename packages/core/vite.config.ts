// packages/core/vite.config.ts
// Configuration for Vite development server and WASM asset handling
// Note: tsup handles the actual build process

import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
  resolve: {
    alias: {
      "rte-core": resolve(__dirname, "./wasm/rte_core"),
    },
  },
  assetsInclude: ["**/*.wasm"],
  optimizeDeps: {
    exclude: ["wasm"],
  },
  server: {
    fs: {
      // Allow serving files from the wasm directory
      allow: [".."],
    },
  },
});
