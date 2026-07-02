import { useStore } from "../store/useStore";
import { Button } from "./ui/Button";
import { Card, CardContent } from "./ui/Card";
import { History, Trash2, FileText, Calendar } from "lucide-react";

export function HistoryList() {
  const { history, deleteHistoryEntry, clearHistory } = useStore();

  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">History</h1>
        {history.length > 0 && (
          <Button variant="outline" size="sm" onClick={clearHistory}>
            <Trash2 size={14} /> Clear All
          </Button>
        )}
      </div>

      {history.length === 0 ? (
        <Card>
          <CardContent className="flex flex-col items-center gap-3 p-12 text-center text-muted-foreground">
            <History size={32} />
            <p>No generations yet.</p>
            <p className="text-sm">Generate a resume from the Generate tab to see it here.</p>
          </CardContent>
        </Card>
      ) : (
        <div className="space-y-3">
          {history.map((entry) => (
            <Card key={entry.id}>
              <CardContent className="p-4">
                <div className="flex items-start justify-between gap-3">
                  <div className="flex-1 space-y-1">
                    <div className="flex items-center gap-2 text-sm text-muted-foreground">
                      <Calendar size={14} />
                      {new Date(entry.timestamp).toLocaleString()}
                    </div>
                    <div className="flex items-center gap-2 text-sm font-medium">
                      <FileText size={14} />
                      {entry.templateName}
                    </div>
                    <p className="text-sm text-muted-foreground line-clamp-2">
                      {entry.jdSnippet}
                    </p>
                    {entry.pdfPath && (
                      <p className="text-xs text-muted-foreground">{entry.pdfPath}</p>
                    )}
                  </div>
                  <div className="flex gap-1">
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => deleteHistoryEntry(entry.id)}
                    >
                      <Trash2 size={14} />
                    </Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
