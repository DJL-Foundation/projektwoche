import type { APIRoute } from "astro";
import puppeteer from "puppeteer";
import {
  getProjectUrl,
  getAvailableYears,
  getParticipants,
  getProjects,
} from "~/lib/projects";

export const GET: APIRoute = async ({ params, redirect }) => {
  "use cache";
  const { year, username, project } = params;

  if (!year || !username || !project) {
    return redirect("/logo.png", 302);
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
