import { useEffect, useState } from "react";
import { useStore, type Tab } from "./store/useStore";
import { SettingsPanel } from "./components/SettingsPanel";
import { ProfileEditor } from "./components/ProfileEditor";
import { ImportResume } from "./components/ImportResume";
import { GeneratePanel } from "./components/GeneratePanel";
import { HistoryList } from "./components/HistoryList";
import { Settings, User, Upload, Zap, History, Moon, Sun, Loader2, AlertCircle, Download, Code, FileText, Edit3 } from "lucide-react";

const TABS: { id: Tab; label: string; icon: React.ComponentType<{ size?: number; className?: string }> }[] = [
  { id: "settings", label: "Settings", icon: Settings },
  { id: "profile", label: "Profile", icon: User },
  { id: "import", label: "Import", icon: Upload },
  { id: "generate", label: "Generate", icon: Zap },
  { id: "history", label: "History", icon: History },
];

interface UpdateInfo {
  available: boolean;
  version?: string;
  body?: string;
}

function App() {
  const {
    activeTab,
    setTab,
    darkMode,
    setDarkMode,
    loading,
    error,
    setError,
    initApp,
    apiKey,
    tinytexInstalled,
    settings,
  } = useStore();

  const [updateInfo, setUpdateInfo] = useState<UpdateInfo>({ available: false });
  const [updating, setUpdating] = useState(false);

  useEffect(() => {
    initApp();
  }, [initApp]);

  useEffect(() => {
    const checkUpdates = async () => {
      try {
        const { check } = await import("@tauri-apps/plugin-updater");
        const update = await check();
        if (update) {
          setUpdateInfo({ available: true, version: update.version, body: update.body });
        }
      } catch {
        // Updater not configured yet or running in dev mode - ignore
      }
    };
    checkUpdates();
  }, []);

  const handleUpdate = async () => {
    setUpdating(true);
    try {
      const { check } = await import("@tauri-apps/plugin-updater");
      const update = await check();
      if (update) {
        await update.downloadAndInstall();
        await import("@tauri-apps/plugin-process").then(({ relaunch }) => relaunch());
      }
    } catch {
      // Ignore update errors
    }
    setUpdating(false);
  };

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-background text-foreground">
      <aside className="flex w-56 flex-col border-r border-border bg-card">
        <div className="flex items-center gap-2 px-4 py-5">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary text-primary-foreground text-sm font-bold">
            TR
          </div>
          <span className="text-lg font-bold tracking-tight">TailorResume</span>
        </div>

        <nav className="flex flex-1 flex-col gap-1 px-2 py-2">
          {TABS.map((tab) => {
            const Icon = tab.icon;
            const active = activeTab === tab.id;
            const locked = tab.id !== "settings" && !apiKey;
            return (
              <button
                key={tab.id}
                onClick={() => !locked && setTab(tab.id)}
                className={`flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                  active
                    ? "bg-primary text-primary-foreground"
                    : locked
                    ? "text-muted-foreground cursor-not-allowed opacity-50"
                    : "text-foreground hover:bg-accent"
                }`}
              >
                <Icon size={18} />
                {tab.label}
                {locked && <span className="ml-auto text-xs">lock</span>}
              </button>
            );
          })}
        </nav>

        <div className="border-t border-border p-3">
          <div className="mb-2 space-y-1">
            <div className="flex items-center justify-between text-xs">
              <span className="text-muted-foreground">API Key</span>
              <span className={apiKey ? "text-green-500" : "text-red-500"}>
                {apiKey ? "Set" : "Not set"}
              </span>
            </div>
            <div className="flex items-center justify-between text-xs">
              <span className="text-muted-foreground">Compiler</span>
              <span className={settings.compileMode === "online" ? "text-green-500" : tinytexInstalled ? "text-green-500" : "text-red-500"}>
                {settings.compileMode === "online" ? "Online" : tinytexInstalled ? "Offline" : "Offline (no TinyTeX)"}
              </span>
            </div>
            <div className="flex items-center justify-between text-xs">
              <span className="text-muted-foreground">Resume Source</span>
              <span className="flex items-center gap-1 text-foreground">
                {settings.importSource === "pdf" && <><FileText size={11} /> PDF</>}
                {settings.importSource === "latex" && <><Code size={11} /> LaTeX</>}
                {(!settings.importSource || settings.importSource === "manual") && <><Edit3 size={11} /> Manual</>}
              </span>
            </div>
          </div>
          <button
            onClick={() => setDarkMode(!darkMode)}
            className="flex w-full items-center gap-2 rounded-md px-3 py-2 text-sm font-medium hover:bg-accent"
          >
            {darkMode ? <Sun size={16} /> : <Moon size={16} />}
            {darkMode ? "Light mode" : "Dark mode"}
          </button>
        </div>
      </aside>

      <main className="flex flex-1 flex-col overflow-hidden">
        {updateInfo.available && (
          <div className="flex items-center gap-2 border-b border-primary/30 bg-primary/10 px-4 py-2 text-sm">
            <Download size={16} className="text-primary" />
            <span className="flex-1 text-primary">
              Update available: v{updateInfo.version} {updating ? "(installing...)" : ""}
            </span>
            <button
              onClick={handleUpdate}
              disabled={updating}
              className="rounded-md bg-primary px-3 py-1 text-xs font-medium text-primary-foreground hover:opacity-90"
            >
              {updating ? "Installing..." : "Update Now"}
            </button>
          </div>
        )}
        {error && (
          <div className="flex items-center gap-2 border-b border-destructive/30 bg-destructive/10 px-4 py-2 text-sm text-destructive">
            <AlertCircle size={16} />
            <span className="flex-1">{error}</span>
            <button onClick={() => setError(null)} className="text-destructive hover:opacity-70">
              x
            </button>
          </div>
        )}

        <div className="flex-1 overflow-y-auto">
          {activeTab === "settings" && <SettingsPanel />}
          {activeTab === "profile" && <ProfileEditor />}
          {activeTab === "import" && <ImportResume />}
          {activeTab === "generate" && <GeneratePanel />}
          {activeTab === "history" && <HistoryList />}
        </div>
      </main>

      {loading && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
          <div className="flex flex-col items-center gap-3">
            <Loader2 size={32} className="animate-spin text-primary" />
            <p className="text-sm text-muted-foreground">Processing...</p>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
