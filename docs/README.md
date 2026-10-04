# Documentation site

A static Astro site for `artifacthub-mcp`.

- `src/content/docs/` contains the Markdown pages.
- `src/content.config.ts` defines their metadata schema.
- `src/pages/[...slug].astro` generates routes and navigation from the collection and contains the page layout.
- `src/styles/global.css` imports Tailwind and styles rendered Markdown.
- `astro.config.ts` configures the static build and Tailwind through PostCSS.

Each page has `title`, `description`, `navTitle`, and `order` metadata. The `index` entry renders at `/`; other entries use their collection IDs as paths. Navigation and previous/next links follow `order`.

## Workspace

The root pnpm workspace includes `docs/` as `@artifacthub-rs/docs`. It follows the reference project's private ESM package setup, named catalogs with exact versions, root scripts that delegate by package name, Node 24 or newer, and `pnpm@12.4.2`.

The workspace retains the security settings used by the reference environment: a four-day minimum release age, disabled dependency lifecycle scripts, strict dependency build checks, and blocked exotic transitive dependencies.

Astro's checker accepts TypeScript 5 or 6, so this site's tooling catalog pins TypeScript 6 rather than the reference project's TypeScript 7.

## Development

After dependencies have been installed separately, use these commands from the repository root:

```sh
pnpm dev
pnpm check
pnpm build
pnpm preview
```

The build produces `docs/dist/`. It does not build or start the Rust server.

No dependencies have been installed for this setup, and no pnpm lockfile has been generated. When installation is authorized, use `sfw pnpm install`. Keep the security policy in place and commit the generated lockfile before configuring reproducible CI builds.

## Hosting

The static build needs no Cloudflare adapter or Worker. For a git-based static deployment, the build command is `pnpm build` at the repository root and the output directory is `docs/dist/`.

Deployment configuration is left to the repository owner. No account, domain, or deployment integration is configured here.
