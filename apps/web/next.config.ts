import { type NextConfig } from "next";

const nextConfig: NextConfig = {
  transpilePackages: ["prowo-ui"],
  experimental: {
    useCache: true,
    ppr: true,
  },
  async rewrites() {
    return [
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
  turbopack: {},
};

export default nextConfig;
