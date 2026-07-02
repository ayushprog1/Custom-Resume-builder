export interface Basics {
  name: string;
  email: string;
  phone: string;
  location: string;
  website: string;
  github: string;
  linkedin: string;
  summary: string;
}

export interface WorkExperience {
  id: string;
  company: string;
  position: string;
  startDate: string;
  endDate: string;
  highlights: string[];
}

export interface Project {
  id: string;
  name: string;
  description: string;
  tech: string[];
  url: string;
  highlights: string[];
}

export interface SkillGroup {
  id: string;
  category: string;
  items: string[];
}

export interface Education {
  id: string;
  institution: string;
  degree: string;
  field: string;
  startDate: string;
  endDate: string;
  gpa: string;
  highlights: string[];
}

export interface Certification {
  id: string;
  name: string;
  issuer: string;
  date: string;
}

export interface Award {
  id: string;
  title: string;
  date: string;
  awarder: string;
}

export interface CustomField {
  id: string;
  key: string;
  value: string;
}

export interface Profile {
  basics: Basics;
  work: WorkExperience[];
  projects: Project[];
  skills: SkillGroup[];
  education: Education[];
  certifications: Certification[];
  awards: Award[];
  custom: CustomField[];
}

export interface Template {
  id: string;
  name: string;
  description: string;
  source: string;
  builtIn: boolean;
}

export interface HistoryEntry {
  id: string;
  timestamp: string;
  jdSnippet: string;
  templateName: string;
  latex: string;
  pdfPath: string | null;
}

export interface GeminiModel {
  id: string;
  name: string;
}

export interface ModelInfo {
  id: string;
  name: string;
  description: string;
  free: boolean;
  recommended: boolean;
}

export interface AppSettings {
  apiKey: string;
  outputFolder: string;
  model: string;
  tinytexInstalled: boolean;
  tinytexPath: string;
  compileMode: "online" | "offline";
  considerations: string[];
  sourceLatex: string;
  importSource: string;
  extractPromptPdf: string;
  extractPromptLatex: string;
  generatePrompt: string;
}

export const EMPTY_PROFILE: Profile = {
  basics: {
    name: "",
    email: "",
    phone: "",
    location: "",
    website: "",
    github: "",
    linkedin: "",
    summary: "",
  },
  work: [],
  projects: [],
  skills: [],
  education: [],
  certifications: [],
  awards: [],
  custom: [],
};

export function newId(): string {
  return crypto.randomUUID();
}
