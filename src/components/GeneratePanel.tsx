import { useState, useEffect } from "react";
import { useStore } from "../store/useStore";
import { Button } from "./ui/Button";
import { Textarea } from "./ui/Textarea";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";
import { LatexPreview } from "./LatexPreview";
import { Zap, Plus, Trash2, FileDown, Loader2, CheckCircle2, AlertCircle, Eye, Save, Code, FolderOpen } from "lucide-react";

export function GeneratePanel() {
  const { templates, generateLatex, generateAndCompile, compileResult, loading, apiKey, tinytexInstalled, settings, setTab, saveConsiderations } = useStore();
  const [jd, setJd] = useState("");
  const [considerations, setConsiderations] = useState<string[]>(settings.considerations.length > 0 ? settings.considerations : [""]);
  const [selectedTemplate, setSelectedTemplate] = useState(settings.sourceLatex ? "my-latex" : (templates[0]?.id || "ats-classic"));
  const [showPreview, setShowPreview] = useState(false);
  const [considerationsSaved, setConsiderationsSaved] = useState(false);

  useEffect(() => {
    if (settings.considerations.length > 0) {
      setConsiderations(settings.considerations);
    }
  }, [settings.considerations]);

  useEffect(() => {
    if (settings.sourceLatex) {
      setSelectedTemplate("my-latex");
    }
  }, [settings.sourceLatex]);

  const canGenerate = jd.trim().length > 0 && apiKey && !loading;

  const handleSaveConsiderations = async () => {
    const filtered = considerations.filter((c) => c.trim());
    await saveConsiderations(filtered);
    setConsiderationsSaved(true);
    setTimeout(() => setConsiderationsSaved(false), 2000);
  };

  const handleGenerateOnly = async () => {
    const filtered = considerations.filter((c) => c.trim());
    await saveConsiderations(filtered);
    await generateLatex(jd, filtered, selectedTemplate);
    setShowPreview(true);
  };

  const handleGenerateAndCompile = async () => {
    const filtered = considerations.filter((c) => c.trim());
    await saveConsiderations(filtered);
    const templateName = selectedTemplate === "my-latex" ? "My LaTeX (imported)" : (templates.find((t) => t.id === selectedTemplate)?.name || "Unknown");
    await generateAndCompile(jd, filtered, selectedTemplate, templateName);
  };

  if (!apiKey) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <Card>
          <CardContent className="flex flex-col items-center gap-4 p-8 text-center">
            <AlertCircle size={32} className="text-destructive" />
            <p className="font-medium">API key required</p>
            <p className="text-sm text-muted-foreground">Go to Settings to paste your Google AI Studio API key.</p>
            <Button onClick={() => setTab("settings")}>Go to Settings</Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (!tinytexInstalled && settings.compileMode === "offline") {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <Card>
          <CardContent className="flex flex-col items-center gap-4 p-8 text-center">
            <AlertCircle size={32} className="text-destructive" />
            <p className="font-medium">TinyTeX not installed</p>
            <p className="text-sm text-muted-foreground">
              Install TinyTeX in Settings, or switch to Online (free) compile mode.
            </p>
            <Button onClick={() => setTab("settings")}>Go to Settings</Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <h1 className="text-2xl font-bold">Generate Resume</h1>

      <Card>
        <CardHeader><CardTitle>Job Description</CardTitle></CardHeader>
        <CardContent>
          <Textarea
            value={jd}
            onChange={(e) => setJd(e.target.value)}
            placeholder="Paste the full job description here..."
            rows={10}
          />
          <p className="mt-2 text-xs text-muted-foreground">{jd.length} characters</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Considerations</CardTitle>
            <div className="flex gap-2">
              <Button size="sm" variant="outline" onClick={handleSaveConsiderations}>
                {considerationsSaved ? <CheckCircle2 size={14} /> : <Save size={14} />}
                {considerationsSaved ? "Saved!" : "Save"}
              </Button>
              <Button size="sm" variant="outline" onClick={() => setConsiderations([...considerations, ""])}>
                <Plus size={14} /> Add Point
              </Button>
            </div>
          </div>
        </CardHeader>
        <CardContent className="space-y-2">
          <p className="text-sm text-muted-foreground">
            Add specific points the AI should consider when tailoring your resume (e.g., "emphasize leadership", "highlight Python skills").
          </p>
          {considerations.map((c, i) => (
            <div key={i} className="flex items-center gap-2">
              <Textarea
                value={c}
                onChange={(e) => {
                  const next = [...considerations];
                  next[i] = e.target.value;
                  setConsiderations(next);
                }}
                placeholder={`Consideration ${i + 1}...`}
                rows={1}
                className="flex-1"
              />
              {considerations.length > 1 && (
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => setConsiderations(considerations.filter((_, idx) => idx !== i))}
                >
                  <Trash2 size={14} />
                </Button>
              )}
            </div>
          ))}
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>Template</CardTitle></CardHeader>
        <CardContent className="space-y-3">
          {settings.sourceLatex && (
            <label
              className={`flex cursor-pointer items-start gap-3 rounded-md border p-3 transition-colors ${
                selectedTemplate === "my-latex" ? "border-primary bg-primary/5" : "border-border hover:bg-accent"
              }`}
            >
              <input
                type="radio"
                name="template"
                checked={selectedTemplate === "my-latex"}
                onChange={() => setSelectedTemplate("my-latex")}
                className="mt-1"
              />
              <div>
                <div className="flex items-center gap-2">
                  <Code size={16} className="text-primary" />
                  <p className="font-medium">My LaTeX (imported)</p>
                </div>
                <p className="text-sm text-muted-foreground">
                  Uses your imported LaTeX as the template. The AI will fill in tailored content using your exact formatting.
                </p>
              </div>
            </label>
          )}
          {templates.map((t) => (
            <label
              key={t.id}
              className={`flex cursor-pointer items-start gap-3 rounded-md border p-3 transition-colors ${
                selectedTemplate === t.id ? "border-primary bg-primary/5" : "border-border hover:bg-accent"
              }`}
            >
              <input
                type="radio"
                name="template"
                checked={selectedTemplate === t.id}
                onChange={() => setSelectedTemplate(t.id)}
                className="mt-1"
              />
              <div>
                <p className="font-medium">{t.name}</p>
                <p className="text-sm text-muted-foreground">{t.description}</p>
              </div>
            </label>
          ))}
        </CardContent>
      </Card>

      <div className="flex flex-wrap gap-3">
        <Button onClick={handleGenerateAndCompile} disabled={!canGenerate} size="lg">
          {loading ? <Loader2 size={18} className="animate-spin" /> : <Zap size={18} />}
          Generate & Save Resume.pdf
        </Button>
        <Button variant="outline" onClick={handleGenerateOnly} disabled={!canGenerate} size="lg">
          <Eye size={18} /> Preview LaTeX Only
        </Button>
      </div>

      {compileResult && (
        <Card>
          <CardContent className="p-4">
            {compileResult.success ? (
              <div className="flex items-center gap-3">
                <CheckCircle2 size={20} className="text-green-500" />
                <div className="flex-1">
                  <p className="font-medium text-green-600">Resume.pdf saved successfully!</p>
                  <p className="text-sm text-muted-foreground">{compileResult.pdf_path}</p>
                </div>
                <div className="flex gap-2">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => {
                      if (compileResult.pdf_path) {
                        import("@tauri-apps/plugin-opener").then(({ openPath, revealItemInDir }) =>
                          openPath(compileResult.pdf_path!).catch(() => revealItemInDir(compileResult.pdf_path!))
                        );
                      }
                    }}
                  >
                    <FileDown size={14} /> Open
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => {
                      if (compileResult.pdf_path) {
                        import("@tauri-apps/plugin-opener").then(({ revealItemInDir }) =>
                          revealItemInDir(compileResult.pdf_path!)
                        );
                      }
                    }}
                  >
                    <FolderOpen size={14} /> Show in Folder
                  </Button>
                </div>
              </div>
            ) : (
              <div className="space-y-2">
                <div className="flex items-center gap-2 text-destructive">
                  <AlertCircle size={20} />
                  <p className="font-medium">Compilation failed</p>
                </div>
                <p className="text-sm text-destructive">{compileResult.error}</p>
                {compileResult.log && (
                  <details>
                    <summary className="cursor-pointer text-sm text-muted-foreground">View compile log</summary>
                    <pre className="mt-2 max-h-64 overflow-auto rounded-md bg-muted p-3 text-xs">
                      {compileResult.log.slice(-2000)}
                    </pre>
                  </details>
                )}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {showPreview && <LatexPreview />}
    </div>
  );
}
