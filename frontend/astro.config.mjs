import adapter from "@astrojs/node";
import { defineConfig } from "astro/config";
import path from "node:path";

export default defineConfig({
  srcDir: path.resolve(),
  output: "server",
  adapter: adapter({ mode: "middleware" }),
});
