import { useState } from "react";
import { useStore } from "../store/useStore";
import { Button } from "./ui/Button";
import { Textarea } from "./ui/Textarea";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";
import { FileDown, Loader2, CheckCircle2, AlertCircle, Save, FolderOpen } from "lucide-react";

export function LatexPreview() {
  const { generatedLatex, setGeneratedLatex, compilePdf, compileResult, loading } = useStore();
  const [editedLatex, setEditedLatex] = useState(generatedLatex);

  const handleCompile = async () => {
    setGeneratedLatex(editedLatex);
    await compilePdf(editedLatex);
  };

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>LaTeX Preview (editable)</CardTitle>
          <Button onClick={handleCompile} disabled={loading || !editedLatex.trim()}>
            {loading ? <Loader2 size={16} className="animate-spin" /> : <Save size={16} />}
            Compile & Save Resume.pdf
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        <Textarea
          value={editedLatex}
          onChange={(e) => setEditedLatex(e.target.value)}
          rows={20}
          className="font-mono text-xs"
        />
        {compileResult && (
          <div className={`rounded-md p-3 ${compileResult.success ? "bg-green-500/10" : "bg-destructive/10"}`}>
            {compileResult.success ? (
              <div className="flex items-center gap-2">
                <CheckCircle2 size={16} className="text-green-500" />
                <span className="text-sm text-green-600">Saved to {compileResult.pdf_path}</span>
                <div className="ml-auto flex gap-2">
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
                    <FileDown size={14} /> Open PDF
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
              <div className="space-y-1">
                <div className="flex items-center gap-2 text-destructive">
                  <AlertCircle size={16} />
                  <span className="text-sm">{compileResult.error}</span>
                </div>
                {compileResult.log && (
                  <pre className="max-h-40 overflow-auto rounded bg-muted p-2 text-xs">
                    {compileResult.log.slice(-1500)}
                  </pre>
                )}
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
