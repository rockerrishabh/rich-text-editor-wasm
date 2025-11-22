import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

export default defineConfig(({ mode }) => ({
  plugins: [
    svelte({
      compilerOptions: {
        dev: mode === "development",
      },
    }),
  ],
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
}));
