import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import { resolve } from "path";

export default defineConfig({
  plugins: [solid()],
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
