import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "path";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@rockerrishabh/rich-text-editor-core": resolve(__dirname, "../core/src/index.ts"),
    },
  },
  assetsInclude: ["**/*.wasm"],
  optimizeDeps: {
    exclude: ["@rockerrishabh/rich-text-editor-core"],
  },
  // Basic lib build settings are handled by `tsup` in this package.
  // Vite config supports local dev and WASM asset handling.
});
