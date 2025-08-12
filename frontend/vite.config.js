import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [sveltekit()],
  server: {
    proxy: {
      "/api": {
        target: "http://mock:4010",
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
});
