import { useState } from "react";
import { useStore } from "../store/useStore";
import { Button } from "./ui/Button";
import { Input } from "./ui/Input";
import { Label } from "./ui/Label";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";
import { CheckCircle2, AlertCircle, FolderOpen, Download, Key, Loader2, Zap, Cloud, RotateCcw, Trash2 } from "lucide-react";

export function SettingsPanel() {
  const {
    apiKey,
    settings,
    tinytexInstalled,
    loading,
    models,
    saveApiKey,
    testApiKey,
    pickOutputFolder,
    setCompileMode,
    setModel,
    checkTinytex,
    installTinytex,
    resetSettings,
    resetProfile,
  } = useStore();

  const [keyInput, setKeyInput] = useState(apiKey);
  const [keySaved, setKeySaved] = useState(false);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<boolean | null>(null);
  const [installing, setInstalling] = useState(false);

  const handleSaveKey = async () => {
    await saveApiKey(keyInput);
    setKeySaved(true);
    setTimeout(() => setKeySaved(false), 2000);
  };

  const handleTest = async () => {
    setTesting(true);
    setTestResult(null);
    const ok = await testApiKey(keyInput);
    setTestResult(ok);
    setTesting(false);
  };

  const handleInstall = async () => {
    setInstalling(true);
    await installTinytex();
    await checkTinytex();
    setInstalling(false);
  };

  return (
    <div className="mx-auto max-w-2xl space-y-6 p-6">
      <h1 className="text-2xl font-bold">Settings</h1>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Key size={20} /> Google AI Studio API Key
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="apikey">Paste your Gemini API key</Label>
            <Input
              id="apikey"
              type="password"
              value={keyInput}
              onChange={(e) => setKeyInput(e.target.value)}
              placeholder="AIza..."
            />
            <p className="text-xs text-muted-foreground">
              Get a free key at{" "}
              <a
                href="https://aistudio.google.com/apikey"
                target="_blank"
                rel="noreferrer"
                className="text-primary underline"
              >
                Google AI Studio
              </a>
              . Stored securely in your OS keychain.
            </p>
          </div>
          <div className="flex gap-2">
            <Button onClick={handleSaveKey} disabled={!keyInput}>
              {keySaved ? <CheckCircle2 size={16} /> : null}
              {keySaved ? "Saved!" : "Save Key"}
            </Button>
            <Button variant="outline" onClick={handleTest} disabled={!keyInput || testing}>
              {testing ? <Loader2 size={16} className="animate-spin" /> : null}
              Test Connection
            </Button>
          </div>
          {testResult !== null && (
            <div
              className={`flex items-center gap-2 text-sm ${
                testResult ? "text-green-500" : "text-destructive"
              }`}
            >
              {testResult ? <CheckCircle2 size={16} /> : <AlertCircle size={16} />}
              {testResult ? "Connection successful!" : "Connection failed. Check your key."}
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FolderOpen size={20} /> Output Folder
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center gap-2">
            <Input value={settings.outputFolder} readOnly className="flex-1" />
            <Button variant="outline" onClick={pickOutputFolder}>
              Browse
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            Generated Resume.pdf will be saved here. If the file already exists, it will be replaced.
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap size={20} /> LaTeX Compiler Mode
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => setCompileMode("online")}
              className={`rounded-md border p-4 text-left transition-colors ${
                settings.compileMode === "online"
                  ? "border-primary bg-primary/5"
                  : "border-border hover:bg-accent"
              }`}
            >
              <div className="mb-1 flex items-center gap-2">
                <Cloud size={18} className="text-primary" />
                <span className="font-medium">Online (Free)</span>
                {settings.compileMode === "online" && (
                  <CheckCircle2 size={14} className="ml-auto text-primary" />
                )}
              </div>
              <p className="text-xs text-muted-foreground">
                No download. Compiles via latexonline.cc (free, open source). Requires internet.
              </p>
            </button>
            <button
              onClick={() => setCompileMode("offline")}
              className={`rounded-md border p-4 text-left transition-colors ${
                settings.compileMode === "offline"
                  ? "border-primary bg-primary/5"
                  : "border-border hover:bg-accent"
              }`}
            >
              <div className="mb-1 flex items-center gap-2">
                <Download size={18} className="text-primary" />
                <span className="font-medium">Offline (TinyTeX)</span>
                {settings.compileMode === "offline" && (
                  <CheckCircle2 size={14} className="ml-auto text-primary" />
                )}
              </div>
              <p className="text-xs text-muted-foreground">
                Download TinyTeX (~300MB, one-time). Works without internet. Full LaTeX power.
              </p>
            </button>
          </div>
          {settings.compileMode === "online" && (
            <div className="rounded-md border border-yellow-500/30 bg-yellow-500/10 p-3 text-xs text-yellow-700 dark:text-yellow-400">
              <strong>Privacy note:</strong> When using online mode, your tailored LaTeX resume
              content is sent to latexonline.cc for compilation. Your Gemini API key is never sent there.
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Download size={20} /> TinyTeX (Offline Compiler)
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center gap-2 text-sm">
            {tinytexInstalled ? (
              <>
                <CheckCircle2 size={16} className="text-green-500" />
                <span className="text-green-500">TinyTeX is installed and ready</span>
              </>
            ) : (
              <>
                <AlertCircle size={16} className="text-muted-foreground" />
                <span className="text-muted-foreground">
                  TinyTeX not installed {settings.compileMode === "online" && "(optional - only needed for offline mode)"}
                </span>
              </>
            )}
          </div>
          {!tinytexInstalled && (
            <>
              <Button onClick={handleInstall} disabled={installing || loading}>
                {installing ? <Loader2 size={16} className="animate-spin" /> : <Download size={16} />}
                {installing ? "Installing... (~300MB download)" : "Install TinyTeX"}
              </Button>
              <p className="text-xs text-muted-foreground">
                Downloads a minimal LaTeX distribution (~300MB) to your app config folder. This is a one-time setup.
                {settings.compileMode === "online" && " Only needed if you switch to Offline mode."}
              </p>
            </>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap size={20} /> AI Model
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="model-select">Select the Gemini model for resume generation</Label>
            <select
              id="model-select"
              value={settings.model}
              onChange={(e) => setModel(e.target.value)}
              className="flex h-10 w-full rounded-md border border-border bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            >
              {models.map((m) => (
                <option key={m.id} value={m.id}>
                  {m.name} {!m.free ? "(paid)" : ""} {m.recommended ? "(Recommended)" : ""}
                </option>
              ))}
            </select>
            {models.find((m) => m.id === settings.model) && (
              <p className="text-xs text-muted-foreground">
                {models.find((m) => m.id === settings.model)!.description}
              </p>
            )}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <RotateCcw size={20} /> Reset
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex flex-wrap gap-3">
            <Button
              variant="outline"
              onClick={() => {
                if (confirm("Reset all settings (output folder, compile mode, considerations, prompts) to defaults? Your API key and profile are kept.")) {
                  resetSettings();
                }
              }}
            >
              <RotateCcw size={16} /> Reset Settings
            </Button>
            <Button
              variant="destructive"
              onClick={() => {
                if (confirm("Delete your entire resume profile? This cannot be undone.")) {
                  resetProfile();
                }
              }}
            >
              <Trash2 size={16} /> Clear Profile
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            Reset Settings: restores default output folder, compile mode, considerations, and prompts (keeps API key and profile).
            Clear Profile: deletes all your resume data (name, experience, skills, etc).
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
