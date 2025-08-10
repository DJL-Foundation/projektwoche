import { type NextConfig } from "next";
import { withMicrofrontends } from "@vercel/microfrontends/next/config";

const nextConfig: NextConfig = {
  // transpilePackages: ["prowo-ui"],
  skipTrailingSlashRedirect: true,
};

const microfrontendsConfig: NextConfig = withMicrofrontends(nextConfig);

export default microfrontendsConfig;
