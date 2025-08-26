import type { ProjectsData, WorkshopYear, Participant, Project } from '../types/projects';
import projectsData from '../assets/projects.json';

export function getProjectsData(): ProjectsData {
  return projectsData as ProjectsData;
}

export function getActiveYear(): number {
  return getProjectsData().activeYear;
}

export function getAvailableYears(): number[] {
  const data = getProjectsData();
  return Object.keys(data.workshops).map(year => parseInt(year, 10)).sort((a, b) => b - a);
}

export function getWorkshopYear(year: number): WorkshopYear | undefined {
  const data = getProjectsData();
  return data.workshops[year.toString()];
}

export function getParticipants(year: number): Array<{ username: string; participant: Participant }> {
  const workshopYear = getWorkshopYear(year);
  if (!workshopYear) return [];
  
  return Object.entries(workshopYear)
    .filter(([username]) => username !== '!')
    .map(([username, participant]) => ({ username, participant: participant as Participant }));
}

export function getParticipant(year: number, username: string): Participant | undefined {
  const workshopYear = getWorkshopYear(year);
  if (!workshopYear || username === '!') return undefined;
  
  return workshopYear[username] as Participant;
}

export function getProjects(year: number, username: string): Array<{ projectName: string; project: Project }> {
  const participant = getParticipant(year, username);
  if (!participant?.projects) return [];
  
  return Object.entries(participant.projects).map(([projectName, project]) => ({
    projectName,
    project
  }));
}

export function getProject(year: number, username: string, projectName: string): Project | undefined {
  const participant = getParticipant(year, username);
  if (!participant?.projects) return undefined;
  
  return participant.projects[projectName];
}

export function getAllProjectsForYear(year: number): Array<{
  username: string;
  displayName: string;
  projectName: string;
  project: Project;
}> {
  const participants = getParticipants(year);
  const allProjects: Array<{
    username: string;
    displayName: string;
    projectName: string;
    project: Project;
  }> = [];
  
  participants.forEach(({ username, participant }) => {
    if (participant.projects) {
      Object.entries(participant.projects).forEach(([projectName, project]) => {
        allProjects.push({
          username,
          displayName: participant.displayName,
          projectName,
          project
        });
      });
    }
  });
  
  return allProjects;
}

export function getProjectUrl(year: number, username: string, projectName: string): string {
  // Generate GitHub Pages URL based on username and project name
  return `https://${username}.github.io/${projectName}`;
}

export function getPreviewImageUrl(year: number, username: string, projectName: string): string {
  // First try direct screenshot path, fallback to API if needed
  return `/screenshots/${year}/${username}/${projectName}.png`;
}