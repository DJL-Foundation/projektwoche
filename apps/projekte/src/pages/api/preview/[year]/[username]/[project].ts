import type { APIRoute } from "astro";
import { ImageResponse } from "@vercel/og";
import { createElement } from "react";
import {
  getProjectUrl,
  getAvailableYears,
  getParticipants,
  getProjects,
} from "~/lib/projects";

export const config = {
  runtime: "edge",
};

export const GET: APIRoute = async ({ params, redirect }) => {
  const { year, username, project } = params;

  if (!year || !username || !project) {
    return redirect("/logo.png", 302);
  }

  try {
    const projectUrl = getProjectUrl(parseInt(year, 10), username, project);

    return new ImageResponse(
      createElement(
        "div",
        {
          style: {
            height: "100%",
            width: "100%",
            display: "flex",
            position: "relative",
          },
        },
        createElement("iframe", {
          src: projectUrl,
          style: {
            width: "100%",
            height: "100%",
            border: "none",
            borderRadius: "8px",
          },
        }),
      ),
      {
        width: 1200,
        height: 800,
      },
    );
  } catch (error) {
    console.error(
      `Failed to generate preview for ${year}/${username}/${project}:`,
      error instanceof Error ? error.message : error,
    );
    return redirect("/logo.png", 302);
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
