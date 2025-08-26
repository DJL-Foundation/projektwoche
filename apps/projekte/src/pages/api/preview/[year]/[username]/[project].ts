import type { APIRoute } from 'astro';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import captureWebsite from 'capture-website';
import { getProjectUrl } from '~/lib/projects';

export const GET: APIRoute = async ({ params, redirect }) => {
  const { year, username, project } = params;
  
  if (!year || !username || !project) {
    return redirect('/logo.png', 302);
  }
  
  // Check if we have a custom preview image in the public/screenshots directory
  const screenshotPath = join(process.cwd(), 'public', 'screenshots', year.toString(), username, `${project}.png`);
  
  if (existsSync(screenshotPath)) {
    return redirect(`/screenshots/${year}/${username}/${project}.png`, 302);
  }
  
  try {
    // Generate screenshot dynamically using capture-website
    const projectUrl = getProjectUrl(parseInt(year, 10), username, project);
    
    const screenshotBuffer = await captureWebsite.buffer(projectUrl, {
      width: 1200,
      height: 800,
      timeout: 10,
      delay: 2,
      quality: 0.8,
      type: 'png',
      clip: {
        x: 0,
        y: 0,
        width: 1200,
        height: 800
      },
      // Handle errors gracefully
      overwrite: true,
      launchOptions: {
        args: [
          '--no-sandbox',
          '--disable-setuid-sandbox',
          '--disable-dev-shm-usage',
          '--disable-web-security'
        ]
      }
    });
    
    return new Response(screenshotBuffer, {
      headers: {
        'Content-Type': 'image/png',
        'Cache-Control': 'public, max-age=86400', // Cache for 24 hours
      },
    });
  } catch (error) {
    console.error(`Failed to generate screenshot for ${projectUrl}:`, error);
    // Fallback to logo
    return redirect('/logo.png', 302);
  }
};

export function getStaticPaths() {
  // We can't pre-generate all possible preview paths
  // This will be handled dynamically
  return [];
}