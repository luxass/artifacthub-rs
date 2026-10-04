# Documentation site

Docs for `artifacthub-mcp`.

- `src/content/docs/` contains the Markdown pages.
- `src/content.config.ts` defines their metadata schema.
- `src/components/` contains the header, footer, installation instructions, and reusable tabs.
- `src/pages/[...slug].astro` generates routes and navigation from the collection and contains the page layout.
- `src/styles/global.css` imports Tailwind and styles rendered Markdown.
- `astro.config.ts` configures the Cloudflare adapter, static build, and Tailwind through PostCSS.

The homepage contains installation, client setup, and expandable examples. `/tools/` contains the tool reference. The old `/getting-started/` and `/workflows/` URLs redirect to homepage sections.

Each page has `title`, `description`, `navTitle`, and `order` metadata. The `index` entry renders at `/`; other entries use their collection IDs as paths. Navigation and previous/next links follow `order`.

## Workspace

The root pnpm workspace includes `docs/` as `@artifacthub-rs/docs`. It follows the reference project's private ESM package setup, named catalogs with exact versions, root scripts that delegate by package name, Node 24 or newer, and `pnpm@12.4.2`.

The workspace retains the security settings used by the reference environment: a four-day minimum release age, disabled dependency lifecycle scripts, strict dependency build checks, and blocked exotic transitive dependencies.

Astro's checker accepts TypeScript 5 or 6, so this site's tooling catalog pins TypeScript 6 rather than the reference project's TypeScript 7.

## Development

From the repository root:

```sh
pnpm dev
pnpm check
pnpm build
pnpm preview
```

The build produces `docs/dist/`. It does not build or start the Rust server.

For dependency installation, use `sfw pnpm install`. Keep the security policy in place and commit the generated lockfile before configuring reproducible CI builds.

## Hosting

The Cloudflare adapter keeps pages statically generated. Build with `pnpm build`; output is in `docs/dist/`.

Deployment is managed by the repository owner.
