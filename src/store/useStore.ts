import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { Profile, Template, HistoryEntry, AppSettings, ModelInfo } from "../types";
import { EMPTY_PROFILE } from "../types";

export type Tab = "settings" | "profile" | "import" | "generate" | "history" | "prompts";

interface CompileResult {
  success: boolean;
  pdf_path: string | null;
  log: string;
  error: string | null;
}

interface AppState {
  activeTab: Tab;
  profile: Profile;
  templates: Template[];
  settings: AppSettings;
  history: HistoryEntry[];
  generatedLatex: string;
  compileResult: CompileResult | null;
  loading: boolean;
  error: string | null;
  tinytexInstalled: boolean;
  apiKey: string;
  darkMode: boolean;
  models: ModelInfo[];

  setTab: (tab: Tab) => void;
  setProfile: (profile: Profile) => void;
  updateProfile: (updater: (p: Profile) => Profile) => void;
  setGeneratedLatex: (latex: string) => void;
  setCompileResult: (result: CompileResult | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setDarkMode: (dark: boolean) => void;

  initApp: () => Promise<void>;
  saveProfile: () => Promise<void>;
  pickOutputFolder: () => Promise<void>;
  setCompileMode: (mode: "online" | "offline") => Promise<void>;
  setModel: (model: string) => Promise<void>;
  saveConsiderations: (considerations: string[]) => Promise<void>;
  savePrompts: (extractPdf: string, extractLatex: string, generate: string) => Promise<void>;
  resetPrompts: () => Promise<void>;
  resetSettings: () => Promise<void>;
  resetProfile: () => Promise<void>;
  setImportSourceManual: () => Promise<void>;
  saveApiKey: (key: string) => Promise<void>;
  testApiKey: (key: string) => Promise<boolean>;
  importFromPdf: (filePath: string) => Promise<void>;
  importFromLatex: (latex: string) => Promise<void>;
  generateLatex: (jd: string, considerations: string[], templateId: string) => Promise<void>;
  generateAndCompile: (jd: string, considerations: string[], templateId: string, templateName: string) => Promise<void>;
  compilePdf: (latexSource: string) => Promise<void>;
  loadHistory: () => Promise<void>;
  deleteHistoryEntry: (id: string) => Promise<void>;
  clearHistory: () => Promise<void>;
  checkTinytex: () => Promise<void>;
  installTinytex: () => Promise<void>;
}

export const useStore = create<AppState>((set, get) => ({
  activeTab: "settings",
  profile: EMPTY_PROFILE,
  templates: [],
  settings: {
    apiKey: "",
    outputFolder: "",
    model: "gemini-2.5-flash",
    tinytexInstalled: false,
    tinytexPath: "",
    compileMode: "online",
    considerations: [],
    sourceLatex: "",
    importSource: "manual",
    extractPromptPdf: "",
    extractPromptLatex: "",
    generatePrompt: "",
  },
  history: [],
  generatedLatex: "",
  compileResult: null,
  loading: false,
  error: null,
  tinytexInstalled: false,
  apiKey: "",
  darkMode: false,
  models: [],

  setTab: (tab) => set({ activeTab: tab }),
  setProfile: (profile) => set({ profile }),
  updateProfile: (updater) => set((state) => ({ profile: updater(state.profile) })),
  setGeneratedLatex: (latex) => set({ generatedLatex: latex }),
  setCompileResult: (result) => set({ compileResult: result }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  setDarkMode: (dark) => {
    set({ darkMode: dark });
    document.documentElement.classList.toggle("dark", dark);
  },

  initApp: async () => {
    try {
      const [profile, templates, settings, apiKey, tinytexInstalled, history, models] = await Promise.all([
        invoke<Profile>("load_profile_cmd"),
        invoke<Template[]>("list_templates_cmd"),
        invoke<AppSettings>("get_settings"),
        invoke<string>("load_api_key"),
        invoke<boolean>("tinytex_status"),
        invoke<HistoryEntry[]>("list_history_cmd"),
        invoke<ModelInfo[]>("list_models_cmd"),
      ]);
      set({
        profile: profile || EMPTY_PROFILE,
        templates: templates || [],
        settings: settings || get().settings,
        apiKey: apiKey || "",
        tinytexInstalled,
        history: history || [],
        models: models || [],
        activeTab: apiKey ? "generate" : "settings",
      });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  saveProfile: async () => {
    try {
      await invoke("save_profile_cmd", { profile: get().profile });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  pickOutputFolder: async () => {
    try {
      const selected = await open({ directory: true, multiple: false });
      if (selected && typeof selected === "string") {
        await invoke("set_output_folder", { folder: selected });
        const settings = { ...get().settings, outputFolder: selected };
        set({ settings });
      }
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setCompileMode: async (mode: "online" | "offline") => {
    try {
      await invoke("set_compile_mode", { mode });
      const settings = { ...get().settings, compileMode: mode };
      set({ settings });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setModel: async (model: string) => {
    try {
      await invoke("set_model", { model });
      const settings = { ...get().settings, model };
      set({ settings });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  saveConsiderations: async (considerations: string[]) => {
    try {
      await invoke("save_considerations", { considerations });
      const settings = { ...get().settings, considerations };
      set({ settings });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  savePrompts: async (extractPdf, extractLatex, generate) => {
    try {
      await invoke("save_prompts", {
        extractPromptPdf: extractPdf,
        extractPromptLatex: extractLatex,
        generatePrompt: generate,
      });
      const settings = { ...get().settings, extractPromptPdf: extractPdf, extractPromptLatex: extractLatex, generatePrompt: generate };
      set({ settings });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  resetPrompts: async () => {
    try {
      await invoke("reset_prompts");
      await get().initApp();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  resetSettings: async () => {
    try {
      await invoke("reset_settings");
      await get().initApp();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  resetProfile: async () => {
    try {
      await invoke("reset_profile");
      set({ profile: EMPTY_PROFILE });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setImportSourceManual: async () => {
    try {
      await invoke("set_import_source_manual");
      const settings = { ...get().settings, importSource: "manual", sourceLatex: "" };
      set({ settings });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  saveApiKey: async (key: string) => {
    try {
      await invoke("save_api_key", { key });
      set({ apiKey: key });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  testApiKey: async (key: string) => {
    try {
      const result = await invoke<boolean>("test_gemini_key", {
        key,
        model: get().settings.model || "gemini-2.5-flash",
      });
      return result;
    } catch (e) {
      set({ error: String(e) });
      return false;
    }
  },

  importFromPdf: async (filePath: string) => {
    set({ loading: true, error: null });
    try {
      const profile = await invoke<Profile>("import_pdf", { filePath });
      set({ profile, loading: false, activeTab: "profile" });
    } catch (e) {
      set({ loading: false, error: String(e) });
    }
  },

  importFromLatex: async (latex: string) => {
    set({ loading: true, error: null });
    try {
      await invoke("import_latex", { latex });
      const settings = { ...get().settings, importSource: "latex", sourceLatex: latex };
      set({ settings, loading: false, activeTab: "generate" });
    } catch (e) {
      set({ loading: false, error: String(e) });
    }
  },

  generateLatex: async (jd, considerations, templateId) => {
    set({ loading: true, error: null });
    try {
      const latex = await invoke<string>("generate_latex", {
        jd,
        considerations,
        templateId,
      });
      set({ generatedLatex: latex, loading: false });
    } catch (e) {
      set({ loading: false, error: String(e) });
    }
  },

  generateAndCompile: async (jd, considerations, templateId, templateName) => {
    set({ loading: true, error: null, compileResult: null });
    try {
      const result = await invoke<CompileResult>("generate_and_compile", {
        jd,
        considerations,
        templateId,
        templateName,
      });
      set({ generatedLatex: result.log ? get().generatedLatex : get().generatedLatex, compileResult: result, loading: false });
      await get().loadHistory();
    } catch (e) {
      set({ loading: false, error: String(e) });
    }
  },

  compilePdf: async (latexSource: string) => {
    set({ loading: true, error: null });
    try {
      const result = await invoke<CompileResult>("compile_pdf", { latexSource });
      set({ compileResult: result, loading: false });
      await get().loadHistory();
    } catch (e) {
      set({ loading: false, error: String(e) });
    }
  },

  loadHistory: async () => {
    try {
      const history = await invoke<HistoryEntry[]>("list_history_cmd");
      set({ history });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  deleteHistoryEntry: async (id: string) => {
    try {
      await invoke("delete_history_cmd", { id });
      await get().loadHistory();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  clearHistory: async () => {
    try {
      await invoke("clear_history_cmd");
      set({ history: [] });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  checkTinytex: async () => {
    try {
      const installed = await invoke<boolean>("tinytex_status");
      set({ tinytexInstalled: installed });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  installTinytex: async () => {
    set({ loading: true, error: null });
    try {
      await invoke("install_tinytex_cmd");
      set({ tinytexInstalled: true, loading: false });
    } catch (e) {
      set({ loading: false, error: String(e) });
    }
  },
}));
