import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default {
  preprocess: vitePreprocess({
    // Disable TypeScript checking in preprocess - tsc handles it separately
    typescript: false,
  }),
};
