import type { APIRoute } from "astro";
import puppeteer from "puppeteer";
import {
  getProjectUrl,
  getAvailableYears,
  getParticipants,
  getProjects,
} from "~/lib/projects";

// Simple semaphore to limit concurrent puppeteer instances
let activeBrowsers = 0;
const MAX_CONCURRENT_BROWSERS = 1;

const waitForSlot = async (): Promise<void> => {
  while (activeBrowsers >= MAX_CONCURRENT_BROWSERS) {
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  activeBrowsers++;
};

const releaseSlot = (): void => {
  activeBrowsers = Math.max(0, activeBrowsers - 1);
};

export const GET: APIRoute = async ({ params, redirect }) => {
  "use cache";
  const { year, username, project } = params;

  if (!year || !username || !project) {
    return redirect("/logo.png", 302);
  }

  let browser;
  let projectUrl;
  try {
    // Wait for a browser slot to be available
    await waitForSlot();

    // Generate screenshot dynamically using puppeteer
    projectUrl = getProjectUrl(parseInt(year, 10), username, project);

    browser = await puppeteer.launch({
      args: [
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-dev-shm-usage",
        "--disable-web-security",
        "--single-process",
        "--no-zygote",
        "--disable-gpu",
        "--disable-software-rasterizer",
        "--disable-background-timer-throttling",
        "--disable-backgrounding-occluded-windows",
        "--disable-renderer-backgrounding",
        "--disable-features=TranslateUI",
        "--disable-ipc-flooding-protection",
      ],
      headless: true,
      executablePath: process.env.CHROME_BIN || undefined,
      timeout: 30000,
      protocolTimeout: 30000,
    });

    const page = await browser.newPage();
    await page.setViewport({ width: 1200, height: 800 });

    // Add more robust page loading with retries
    await page.goto(projectUrl, {
      waitUntil: "networkidle2",
      timeout: 15000,
    });

    // Wait for page to fully load
    await new Promise((resolve) => setTimeout(resolve, 3000));

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
      error instanceof Error ? error.message : error,
    );
    // Fallback to logo on any error (including Chrome dependency issues)
    return redirect("/logo.png", 302);
  } finally {
    if (browser) {
      await browser.close();
    }
    releaseSlot();
  }
};

export function getStaticPaths() {
  // Generate paths from projects.json for better build-time optimization
  const paths: Array<{
    params: { year: string; username: string; project: string };
  }> = [];

  const years = getAvailableYears();

  years.forEach((year) => {
    const participants = getParticipants(year);
    participants.forEach(({ username }) => {
      const projects = getProjects(year, username);
      projects.forEach(({ projectName }) => {
        paths.push({
          params: {
            year: year.toString(),
            username,
            project: projectName,
          },
        });
      });
    });
  });

  return paths;
}
