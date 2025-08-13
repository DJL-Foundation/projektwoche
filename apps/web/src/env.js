import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

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

export const env = createEnv({
  server: {
    HOST_URL: z.url().default(hostUrl),
    NODE_ENV: z
      .enum(["development", "test", "production"])
      .default("development"),
  },
  client: {
    NEXT_PUBLIC_HOST_URL: z.url().default(hostUrl),
    // Analytics
    NEXT_PUBLIC_POSTHOG_KEY: z.string(),
    NEXT_PUBLIC_POSTHOG_HOST: z.url(),
    NEXT_PUBLIC_POSTHOG_PROJECT_ID: z.string(),
  },
  runtimeEnv: {
    NODE_ENV: process.env.NODE_ENV,
    NEXT_PUBLIC_HOST_URL: hostUrl,
    HOST_URL: hostUrl,

    // Analytics
    NEXT_PUBLIC_POSTHOG_KEY: process.env.NEXT_PUBLIC_POSTHOG_KEY,
    NEXT_PUBLIC_POSTHOG_HOST: process.env.NEXT_PUBLIC_POSTHOG_HOST,
    NEXT_PUBLIC_POSTHOG_PROJECT_ID: process.env.NEXT_PUBLIC_POSTHOG_PROJECT_ID,
  },
  skipValidation: !!process.env.SKIP_ENV_VALIDATION,
  emptyStringAsUndefined: true,
});

export default env;
