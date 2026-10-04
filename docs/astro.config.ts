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
    shikiConfig: {
      themes: { light: "github-light", dark: "github-dark" },
      defaultColor: false,
    },
  },
});
