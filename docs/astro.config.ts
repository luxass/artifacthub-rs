import { defineConfig } from "astro/config";
import cloudflare from "@astrojs/cloudflare";
import tailwindcss from "@tailwindcss/postcss";

export default defineConfig({
  site: "https://artifacthub-mcp.luxass.dev",
  output: "static",
  adapter: cloudflare({ imageService: "passthrough" }),
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
