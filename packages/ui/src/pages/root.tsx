import { HeroSection } from "../hero-section";
import { ProjectShowcase } from "../project-showcase";
import { SustainabilitySection } from "../sustainability-section";
import { Separator } from "../ui/separator";
import projectsData from "../../../../projects.json";

interface Project {
  name: string;
  description?: string;
  url?: string;
  status?: "active" | "completed" | "paused" | "cancelled";
  tags?: string[];
}

interface Participant {
  displayName: string;
  projects?: Record<string, Project>;
}

interface WorkshopMetadata {
  displayName: string;
  site?: string;
}

interface YearData {
  "!"?: WorkshopMetadata;
  [participantId: string]: Participant | WorkshopMetadata | undefined;
}

interface ProjectsData {
  activeYear: number;
  workshops: Record<string, YearData>;
}

// Function to get random student projects
function getRandomProjects(count = 2) {
  const year = projectsData.activeYear;
  const students = Object.entries(
    (projectsData.workshops as Record<string, YearData>)[year.toString()] ?? {},
  )
    .filter(([username]) => username !== "!")
    .map(([username, data]) => ({
      username,
      displayName: data?.displayName ?? "Namenslos",
      projects: Object.entries((data as Participant)?.projects ?? {}).map(
        ([projectName, projectData]) => ({
          projectName,
          projectTitle: projectData.name,
        }),
      ),
    }));

  // Flatten all projects
  const allProjects = students.flatMap((student) =>
    student.projects.map((project) => ({
      username: student.username,
      displayName: student.displayName,
      projectName: project.projectName,
      projectTitle: project.projectTitle,
      year,
    })),
  );

  // Shuffle and return random projects
  const shuffled = allProjects.sort(() => 0.5 - Math.random());
  return shuffled.slice(0, count);
}

export default function HomePage() {
  const randomProjects = getRandomProjects(2);

  return (
    <main className="min-h-screen">
      <div className="to-background from-secondary/75 bg-radial-[at_100%_100%]">
        <HeroSection />
        <Separator />
        <ProjectShowcase projects={randomProjects} />
      </div>
      <SustainabilitySection />
    </main>
  );
}
