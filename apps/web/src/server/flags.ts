import { flag } from "flags/next";
import { createPostHogAdapter } from "@flags-sdk/posthog";
import env from "#env";

const postHogAdapter = createPostHogAdapter({
  postHogKey: env.NEXT_PUBLIC_POSTHOG_KEY,
  postHogOptions: {
    host: env.NEXT_PUBLIC_POSTHOG_HOST,
  },
});

export const devModeFlag = flag({
  key: "dev-mode",
  defaultValue: false,
  description: "Internal Overrides",
  decide: () => {
    if (env.NODE_ENV !== "production") {
      return true;
    } else {
      return false;
    }
  },
});

export const releaseSetupCLI = flag({
  key: "release-setup-cli",
  defaultValue: false,
  description: "Show the Download / Installation Pages in /setup",
  adapter: postHogAdapter.isFeatureEnabled(),
});
