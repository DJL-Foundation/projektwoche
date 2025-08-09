import type { NextRequest } from "next/server";
import { runMicrofrontendsMiddleware } from "@vercel/microfrontends/next/middleware";

export async function middleware(request: NextRequest) {
  const response = await runMicrofrontendsMiddleware({
    request,
    flagValues: {},
  });
  if (response) {
    return response;
  }
}

// Define routes or paths where this middleware should apply
export const config = {
  matcher: [
    "/.well-known/vercel/microfrontends/client-config", // For prefetch optimizations for flagged paths
    "/flagged/path",
  ],
};
