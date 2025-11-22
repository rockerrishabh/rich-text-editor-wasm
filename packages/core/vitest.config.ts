// packages/core/vitest.config.ts
// Configuration for Vitest (test runner)

import { defineConfig } from "vitest/config";
import { resolve } from "path";

export default defineConfig({
  test: {
    globals: true,
    environment: "node",
  },
  resolve: {
    alias: {
      "rte-core": resolve(__dirname, "./wasm/rte_core"),
    },
  },
  assetsInclude: ["**/*.wasm"],
});
