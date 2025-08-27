export interface Project {
  name: string;
  description?: string;
  url?: string;
  status?: 'active' | 'completed' | 'paused' | 'cancelled';
  tags?: string[];
}

export interface Participant {
  displayName: string;
  projects?: Record<string, Project>;
}

export interface WorkshopMeta {
  displayName: string;
  site?: string;
}

export interface WorkshopYear {
  '!'?: WorkshopMeta;
  [username: string]: Participant | WorkshopMeta | undefined;
}

export interface ProjectsData {
  activeYear: number;
  workshops: Record<string, WorkshopYear>;
}