import type { APIRoute } from 'astro';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

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
  
  // Fallback to logo
  return redirect('/logo.png', 302);
};

export function getStaticPaths() {
  // We can't pre-generate all possible preview paths
  // This will be handled dynamically
  return [];
}