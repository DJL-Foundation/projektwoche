import type { APIRoute } from "astro";
import {
  getAvailableYears,
  getParticipants,
  getProjects,
} from "~/lib/projects";

/**
 * Screenshot generation originally launched Chromium and loaded each external
 * GitHub Pages site. The archive deliberately avoids presenting that retired
 * infrastructure as live and serves an explicit local fallback instead.
 */
export const GET: APIRoute = ({ redirect }) =>
  redirect("/archive-project.svg", 302);

export function getStaticPaths() {
  return getAvailableYears().flatMap((year) =>
    getParticipants(year).flatMap(({ username }) =>
      getProjects(year, username).map(({ projectName }) => ({
        params: { year: year.toString(), username, project: projectName },
      })),
    ),
  );
}
