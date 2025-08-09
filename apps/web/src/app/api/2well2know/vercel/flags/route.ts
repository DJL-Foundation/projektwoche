import { createFlagsDiscoveryEndpoint } from "flags/next";
import { getProviderData as getPostHogProviderData } from "@flags-sdk/posthog";
import env from "#env";

export const GET = createFlagsDiscoveryEndpoint(() =>
  getPostHogProviderData({
    personalApiKey: env.NEXT_PUBLIC_POSTHOG_KEY,
    projectId: env.NEXT_PUBLIC_POSTHOG_PROJECT_ID,
  }),
);
