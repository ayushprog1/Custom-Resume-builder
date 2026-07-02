import { useState } from "react";
import { useStore } from "../store/useStore";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "./ui/Button";
import { Textarea } from "./ui/Textarea";
import { Label } from "./ui/Label";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";
import { FileText, Code, Upload, Loader2, ArrowRight } from "lucide-react";

export function ImportResume() {
  const { importFromPdf, importFromLatex, loading } = useStore();
  const [latexInput, setLatexInput] = useState("");
  const [mode, setMode] = useState<"pdf" | "latex">("pdf");

  const handleFilePick = async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: "PDF", extensions: ["pdf"] }],
    });
    if (selected && typeof selected === "string") {
      await importFromPdf(selected);
    }
  };

  const handleLatexImport = async () => {
    if (latexInput.trim()) {
      await importFromLatex(latexInput);
    }
  };

  return (
    <div className="mx-auto max-w-2xl space-y-6 p-6">
      <h1 className="text-2xl font-bold">Import Resume</h1>
      <p className="text-sm text-muted-foreground">
        Upload your existing resume (PDF or LaTeX) and the AI will extract it into a structured profile you can edit.
      </p>

      <div className="flex gap-2">
        <Button
          variant={mode === "pdf" ? "default" : "outline"}
          onClick={() => setMode("pdf")}
        >
          <FileText size={16} /> PDF Upload
        </Button>
        <Button
          variant={mode === "latex" ? "default" : "outline"}
          onClick={() => setMode("latex")}
        >
          <Code size={16} /> LaTeX Paste
        </Button>
      </div>

      {mode === "pdf" ? (
        <Card>
          <CardHeader><CardTitle>Upload PDF Resume</CardTitle></CardHeader>
          <CardContent className="space-y-4">
            <div
              className="flex flex-col items-center justify-center gap-3 rounded-lg border-2 border-dashed border-border p-12 text-center"
              onClick={handleFilePick}
              style={{ cursor: "pointer" }}
            >
              {loading ? (
                <Loader2 size={32} className="animate-spin text-primary" />
              ) : (
                <Upload size={32} className="text-muted-foreground" />
              )}
              <div>
                <p className="font-medium">Click to select a PDF file</p>
                <p className="text-sm text-muted-foreground">Your resume PDF will be sent to Gemini for extraction</p>
              </div>
            </div>
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader><CardTitle>Paste LaTeX Source</CardTitle></CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label>Paste your .tex source code</Label>
              <Textarea
                value={latexInput}
                onChange={(e) => setLatexInput(e.target.value)}
                placeholder="\documentclass{article}..."
                rows={12}
                className="font-mono text-xs"
              />
            </div>
            <Button onClick={handleLatexImport} disabled={!latexInput.trim() || loading}>
              {loading ? <Loader2 size={16} className="animate-spin" /> : <ArrowRight size={16} />}
              Use This LaTeX
            </Button>
          </CardContent>
        </Card>
      )}

      <div className="rounded-lg border border-border bg-muted/30 p-4 text-sm text-muted-foreground">
        {mode === "pdf" ? (
          "After extraction, you'll be taken to the Profile tab where you can review and edit the extracted data. The AI does its best to parse everything, but always review the results."
        ) : (
          "Your LaTeX is stored as-is and used as the template for generating tailored resumes. No AI extraction needed — the information is already in the LaTeX. You'll be taken to the Generate tab."
        )}
      </div>
    </div>
  );
}
