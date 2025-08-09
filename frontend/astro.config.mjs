import { defineConfig } from "astro/config";
import path from "node:path";

export default defineConfig({
  srcDir: path.resolve(),
  vite: {
    server: {
      proxy: {
        "/api": "http://mock:4010",
      },
    },
  },
});
