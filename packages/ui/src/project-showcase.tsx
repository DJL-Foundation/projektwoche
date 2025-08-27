"use client";

import { Card, CardContent } from "./ui/card";
import { Badge } from "./ui/badge";
import { ExternalLink } from "lucide-react";
import Link from "next/link";

interface Project {
  username: string;
  displayName: string;
  projectName: string;
  projectTitle: string;
  year: number;
}

interface ProjectShowcaseProps {
  projects: Project[];
}

export function ProjectShowcase({ projects }: ProjectShowcaseProps) {
  return (
    <section className="bg-transparent py-16">
      <div className="container mx-auto px-4">
        <div className="mb-12 text-center">
          <h2 className="mb-4 text-3xl font-bold text-balance">
            Schülerprojekte entdecken
          </h2>
          <p className="text-muted-foreground mx-auto max-w-2xl text-lg text-pretty">
            Unsere Schüler entwickeln innovative Webseiten mit Fokus auf
            Nachhaltigkeit. Hier sind einige Beispiele ihrer kreativen Arbeit.
          </p>
        </div>

        <div className="mx-auto grid max-w-4xl gap-8 md:grid-cols-2">
          {projects.map((project, _index) => (
            <Card
              key={`${project.username}-${project.projectName}`}
              className="group transition-all duration-300 hover:-translate-y-1 hover:shadow-lg"
            >
              <div className="from-primary/10 to-accent/10 aspect-[3/2] overflow-hidden rounded-t-lg bg-gradient-to-br">
                <img
                  src={`/api/preview/${project.year}/${project.username}/${project.projectName}`}
                  alt={`Screenshot von ${project.projectTitle}`}
                  className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                  onError={(e) => {
                    const target = e.target as HTMLImageElement;
                    target.src = `/placeholder.svg?height=400&width=600&query=Webseite Screenshot für ${project.projectTitle}`;
                  }}
                />
              </div>
              <CardContent className="p-6">
                <div className="mb-3 flex items-start justify-between">
                  <h3 className="text-xl font-semibold text-balance">
                    {project.projectTitle}
                  </h3>
                  <Link
                    href={`/projekte/${project.year}/${project.username}/${project.projectName}`}
                    prefetch
                  >
                    <ExternalLink className="text-muted-foreground group-hover:text-primary h-5 w-5 transition-colors" />
                  </Link>
                </div>
                <p className="text-muted-foreground mb-4">
                  Entwickelt von{" "}
                  <span className="text-foreground font-medium">
                    {project.displayName}
                  </span>
                </p>
                <div className="flex gap-2">
                  <Badge variant="secondary">Nachhaltigkeit</Badge>
                  <Badge variant="outline">Web Development</Badge>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
}
