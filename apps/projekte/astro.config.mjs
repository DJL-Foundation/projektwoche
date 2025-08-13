// @ts-check
import { defineConfig } from "astro/config";

import devtoolsJson from "vite-plugin-devtools-json";
import react from "@astrojs/react";
import tailwindcss from "@tailwindcss/vite";
import { microfrontends } from "@vercel/microfrontends/experimental/vite";
import vercel from "@astrojs/vercel";
import tsconfigPaths from "vite-tsconfig-paths";

import mdx from "@astrojs/mdx";

import sitemap from "@astrojs/sitemap";

const getHost = () => {
  // Handle client-side
  if (typeof window !== "undefined" && window.location.host) {
    const protocol = window.location.protocol;
    return `${protocol}//${window.location.host}`;
  }

  // Handle server-side with Vercel environment
  if (process.env.VERCEL_URL) {
    return `https://${process.env.VERCEL_URL}`;
  }

  // Handle server-side with explicit HOST/PORT
  if (process.env.HOST || process.env.PORT) {
    const host = process.env.HOST ?? "localhost";
    const port = process.env.PORT ?? "3000";
    // Use PROTOCOL env var if available, otherwise infer from port
    const protocol =
      process.env.PROTOCOL ??
      (port === "443"
        ? "https"
        : process.env.NODE_ENV === "production"
          ? "https"
          : "http");
    // Only add port if it's non-standard for the protocol
    const needsPort =
      (protocol === "https" && port !== "443") ||
      (protocol === "http" && port !== "80");
    return `${protocol}://${host}${needsPort ? `:${port}` : ""}`;
  }

  // Development fallback
  return "http://localhost:3000";
};
const hostUrl = getHost();

process.env.HOST_URL = hostUrl;

// https://astro.build/config
export default defineConfig({
  site: hostUrl,
  integrations: [
    react(),
    mdx({
      gfm: true,
      syntaxHighlight: "shiki",
    }),
    sitemap({
      customPages: [
        // Nextjs (Main Microfrontend)
        `${hostUrl}/`,
        `${hostUrl}/about`,
      ],
    }),
  ],
  vite: {
    plugins: [
      // @ts-ignore
      microfrontends({
        // basePath: "/projekte", // not because of the devtoolsjson plugin
      }),
      // @ts-ignore
      tailwindcss(),
      // @ts-ignore
      devtoolsJson(),
      tsconfigPaths(),
    ],
  },
  adapter: vercel(),
});
