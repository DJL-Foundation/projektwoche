import "./src/env.js";
import { type NextConfig } from "next";
import { withMicrofrontends } from "@vercel/microfrontends/next/config";
import { withBotId } from "botid/next/config";

const nextConfig: NextConfig = {
  transpilePackages: ["prowo-ui"],
  experimental: {
    useCache: true,
    // ppr: true,
  },
  async rewrites() {
    return [
      {
        source: "/.well-known/:path*",
        destination: "/api/2well2know/:path*",
      },
      {
        source: "/ingest/static/:path*",
        destination: "https://eu-assets.i.posthog.com/static/:path*",
      },
      {
        source: "/ingest/:path*",
        destination: "https://eu.i.posthog.com/:path*",
      },
      {
        source: "/ingest/decide",
        destination: "https://eu.i.posthog.com/decide",
      },
      {
        source: "/privacy",
        destination: "https://hackclub-stade.de/privacy",
      },
      {
        source: "/terms",
        destination: "https://hackclub-stade.de/terms",
      },
    ];
  },
  skipTrailingSlashRedirect: true,
};

import withVercelToolbar from "@vercel/toolbar/plugins/next";

const toolbarConfig = withVercelToolbar()(nextConfig);

const microfrontendsConfig = withMicrofrontends(toolbarConfig) as NextConfig;

const botIdConfig = withBotId(microfrontendsConfig) as NextConfig;

export default botIdConfig;
