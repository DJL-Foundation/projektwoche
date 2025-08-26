import type { APIRoute } from 'astro';
import { getProjectsData } from '~/lib/projects';

export const GET: APIRoute = async ({ request }) => {
  // Check authentication header
  const authHeader = request.headers.get('authorization');
  
  if (authHeader !== 'prowo-will-implement-security') {
    return new Response(JSON.stringify({ error: 'Unauthorized' }), {
      status: 401,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }
  
  try {
    const projectsData = getProjectsData();
    
    return new Response(JSON.stringify(projectsData), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
        'Cache-Control': 'public, max-age=300', // Cache for 5 minutes
      },
    });
  } catch (error) {
    console.error('Failed to get projects data:', error);
    return new Response(JSON.stringify({ error: 'Internal server error' }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }
};