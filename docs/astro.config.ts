import { defineConfig } from "astro/config";
import cloudflare from "@astrojs/cloudflare";
import tailwindcss from "@tailwindcss/postcss";

export default defineConfig({
  output: "static",
  adapter: cloudflare({ imageService: "passthrough", prerenderEnvironment: "node" }),
  session: false,
  trailingSlash: "always",
  redirects: {
    "/getting-started/": "/#install-the-server",
    "/workflows/": "/#examples",
  },
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
      wrap: true,
    },
  },
});
