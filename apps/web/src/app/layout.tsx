import "./globals.css";

import { GeistSans } from "geist/font/sans";
import { Ledger } from "next/font/google";
import type React from "react";
import type { Metadata } from "next";
import { ThemeProvider } from "prowo-ui/theme-provider";

import { TRPCReactProvider } from "~/trpc/react";
import { BotIdClient } from "botid/client";

import { PostHogProvider } from "~/server/providers";
import env from "~/env";
import LayoutContent from "prowo-ui/layout-content";
import { devModeFlag } from "#flags";

const ledger = Ledger({
  subsets: ["latin"],
  weight: ["400"],
});

export const metadata: Metadata = {
  metadataBase: new URL(env.HOST_URL || "https://prowo.hackclub-stade.de"),
  title: "Nachhaltige Webentwicklung - Hackclub Stade",
  description:
    "Hackclub Stade hostet das Projekt 'Nachhaltige Webentwicklung' bei der Projektwoche des Gymnasium Athenaeum Stade. Lerne Webentwicklung und nachhaltige IT.",
  icons: {
    icon: "/favicon.ico",
  },
  generator: "Next.js",
  applicationName: "Nachhaltige Webentwicklung",
  referrer: "origin-when-cross-origin",
  keywords: [
    "projektwoche",
    "webentwicklung",
    "nachhaltigkeit",
    "gymnasium athenaeum stade",
    "hackclub stade",
    "html",
    "css",
    "javascript",
    "stade",
    "projekt",
    "lernen",
  ],
  authors: [
    { name: "Jack Ruder", url: "https://jack.djl.foundation" },
    { name: "Ole Gehrmann" },
  ],
  creator: "Hackclub Stade",
  publisher: "The DJL Foundation",
  formatDetection: {
    address: false,
    email: false,
    telephone: false,
  },
  openGraph: {
    title: "Nachhaltige Webentwicklung - Hackclub Stade",
    description:
      "Hackclub Stade hostet das Projekt 'Nachhaltige Webentwicklung' bei der Projektwoche des Gymnasium Athenaeum Stade. Lerne Webentwicklung und nachhaltige IT.",
    url: "https://prowo.hackclub-stade.de",
    type: "website",
    locale: "de_DE",
    siteName: "Nachhaltige Webentwicklung",
    images: [
      {
        url: "/logo.png",
        width: 512,
        height: 512,
        alt: "Hackclub Stade Logo",
      },
    ],
  },
  robots: {
    index: true,
    follow: true,
    nocache: false,
    googleBot: {
      index: true,
      follow: true,
      noimageindex: false,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  twitter: {
    card: "summary_large_image",
    site: "@JackatDJL",
    title: "Nachhaltige Webentwicklung - Hackclub Stade",
    description:
      "Hackclub Stade hostet das Projekt 'Nachhaltige Webentwicklung' bei der Projektwoche des Gymnasium Athenaeum Stade. Lerne Webentwicklung und nachhaltige IT.",
    images: {
      url: "/logo.png",
      width: 512,
      height: 512,
      alt: "Nachhaltige Webentwicklung - Hackclub Stade Logo",
    },
  },
  category: "Education",
  other: {
    classification: "Educational Project",
  },
};

export default async function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  const beta = env.NODE_ENV === "development";
  const shouldShowVercelToolbar = await devModeFlag();

  return (
    <html lang="de" suppressHydrationWarning>
      <body className={`${GeistSans.variable} ${ledger.className} antialiased`}>
        <TRPCReactProvider>
          <BotIdClient
            protect={[
              {
                path: "/api/trpc/*",
                method: "GET",
              },
              {
                path: "/api/trpc/*",
                method: "POST",
              },
            ]}
          />
          <PostHogProvider>
            <ThemeProvider
              attribute="class"
              defaultTheme="system"
              enableSystem
              disableTransitionOnChange
            >
              <LayoutContent
                shouldShowVercelToolbar={shouldShowVercelToolbar}
                beta={beta}
              >
                {children}
              </LayoutContent>
            </ThemeProvider>
          </PostHogProvider>
        </TRPCReactProvider>
      </body>
    </html>
  );
}
