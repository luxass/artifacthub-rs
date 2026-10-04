import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/postcss";

export default defineConfig({
  output: "static",
  trailingSlash: "always",
  vite: {
    css: {
      postcss: {
        plugins: [tailwindcss()],
      },
    },
  },
  markdown: {
    shikiConfig: { theme: "github-dark" },
  },
});
