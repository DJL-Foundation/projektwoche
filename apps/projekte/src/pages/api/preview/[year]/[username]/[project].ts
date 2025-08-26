import type { APIRoute } from "astro";
import { existsSync } from "node:fs";
import { join } from "node:path";
import puppeteer from "puppeteer";
import { getProjectUrl } from "~/lib/projects";

export const GET: APIRoute = async ({ params, redirect }) => {
  const { year, username, project } = params;

  if (!year || !username || !project) {
    return redirect("/logo.png", 302);
  }

  // Check if we have a custom preview image in the public/screenshots directory
  const screenshotPath = join(
    process.cwd(),
    "public",
    "screenshots",
    year.toString(),
    username,
    `${project}.png`,
  );

  if (existsSync(screenshotPath)) {
    return redirect(`/screenshots/${year}/${username}/${project}.png`, 302);
  }

  let browser;
  let projectUrl;
  try {
    // Generate screenshot dynamically using puppeteer
    projectUrl = getProjectUrl(parseInt(year, 10), username, project);

    browser = await puppeteer.launch({
      args: [
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-dev-shm-usage",
        "--disable-web-security",
      ],
    });

    const page = await browser.newPage();
    await page.setViewport({ width: 1200, height: 800 });
    await page.goto(projectUrl, { waitUntil: "networkidle2", timeout: 10000 });
    await new Promise((resolve) => setTimeout(resolve, 2000));

    const screenshotBuffer = await page.screenshot({
      type: "png",
      clip: {
        x: 0,
        y: 0,
        width: 1200,
        height: 800,
      },
    });

    return new Response(Buffer.from(screenshotBuffer), {
      headers: {
        "Content-Type": "image/png",
        "Cache-Control": "public, max-age=86400", // Cache for 24 hours
      },
    });
  } catch (error) {
    console.error(
      `Failed to generate screenshot for ${projectUrl || "unknown"}:`,
      error,
    );
    // Fallback to logo
    return redirect("/logo.png", 302);
  } finally {
    if (browser) {
      await browser.close();
    }
  }
};

export function getStaticPaths() {
  // We can't pre-generate all possible preview paths
  // This will be handled dynamically
  return [];
}
