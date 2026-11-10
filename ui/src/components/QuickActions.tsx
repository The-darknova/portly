import React, { useState } from "react";
import { Globe, Terminal, FolderOpen, Code2, Trash2, ShieldCheck } from "lucide-react";
import { ListeningSocket } from "../types";

interface QuickActionsProps {
  socket: ListeningSocket;
  onOpenBrowser: (port: number) => void;
  onOpenTerminal: (path: string) => void;
  onOpenFolder: (path: string) => void;
  onOpenEditor: (path: string) => void;
  onKill: (pid: number, force: boolean, escalate: boolean) => Promise<void>;
}

export const QuickActions: React.FC<QuickActionsProps> = ({
  socket,
  onOpenBrowser,
  onOpenTerminal,
  onOpenFolder,
  onOpenEditor,
  onKill,
}) => {
  const [confirmKill, setConfirmKill] = useState(false);
  const [isKilling, setIsKilling] = useState(false);

  const handleKill = async (force: boolean, escalate: boolean) => {
    if (!socket.pid) return;
    setIsKilling(true);
    try {
      await onKill(socket.pid, force, escalate);
    } finally {
      setIsKilling(false);
      setConfirmKill(false);
    }
  };

  return (
    <div className="flex items-center gap-1.5 justify-end">
      {/* Open in Browser */}
      <button
        onClick={() => onOpenBrowser(socket.local_port)}
        title={`Open http://localhost:${socket.local_port} in browser`}
        className="p-1.5 rounded-md text-slate-400 hover:text-blue-400 hover:bg-blue-950/40 border border-transparent hover:border-blue-800/40 transition-all"
      >
        <Globe className="w-4 h-4" />
      </button>

      {/* Open in Terminal */}
      {socket.cwd && (
        <button
          onClick={() => onOpenTerminal(socket.cwd!)}
          title={`Open terminal in ${socket.cwd}`}
          className="p-1.5 rounded-md text-slate-400 hover:text-emerald-400 hover:bg-emerald-950/40 border border-transparent hover:border-emerald-800/40 transition-all"
        >
          <Terminal className="w-4 h-4" />
        </button>
      )}

      {/* Open in Code Editor */}
      {socket.cwd && (
        <button
          onClick={() => onOpenEditor(socket.cwd!)}
          title={`Open project root in Code Editor (Cursor / VS Code / Zed)`}
          className="p-1.5 rounded-md text-slate-400 hover:text-cyan-400 hover:bg-cyan-950/40 border border-transparent hover:border-cyan-800/40 transition-all"
        >
          <Code2 className="w-4 h-4" />
        </button>
      )}

      {/* Open in Folder Explorer */}
      {socket.cwd && (
        <button
          onClick={() => onOpenFolder(socket.cwd!)}
          title={`Reveal folder in File Manager`}
          className="p-1.5 rounded-md text-slate-400 hover:text-amber-400 hover:bg-amber-950/40 border border-transparent hover:border-amber-800/40 transition-all"
        >
          <FolderOpen className="w-4 h-4" />
        </button>
      )}

      {/* Kill Process */}
      {socket.pid && (
        <div className="relative">
          {!confirmKill ? (
            <button
              onClick={() => setConfirmKill(true)}
              title="Terminate process"
              className="p-1.5 rounded-md text-slate-400 hover:text-rose-400 hover:bg-rose-950/40 border border-transparent hover:border-rose-800/40 transition-all"
            >
              <Trash2 className="w-4 h-4" />
            </button>
          ) : (
            <div className="absolute right-0 top-1/2 -translate-y-1/2 z-30 flex items-center gap-1 bg-slate-900 border border-rose-800/60 p-1 rounded-lg shadow-xl animate-in fade-in zoom-in-95 duration-100">
              <span className="text-[11px] text-rose-300 font-medium px-1">Kill?</span>
              <button
                disabled={isKilling}
                onClick={() => handleKill(false, false)}
                title="Graceful SIGTERM"
                className="px-2 py-0.5 text-xs bg-rose-600 hover:bg-rose-500 text-white rounded font-medium transition-all"
              >
                Term
              </button>
              <button
                disabled={isKilling}
                onClick={() => handleKill(true, false)}
                title="Forced SIGKILL (-9)"
                className="px-2 py-0.5 text-xs bg-red-800 hover:bg-red-700 text-white rounded font-semibold transition-all"
              >
                Force
              </button>
              <button
                disabled={isKilling}
                onClick={() => handleKill(true, true)}
                title="Kill with elevated privileges (pkexec / UAC)"
                className="px-2 py-0.5 text-xs bg-amber-600 hover:bg-amber-500 text-white rounded font-semibold flex items-center gap-0.5 transition-all"
              >
                <ShieldCheck className="w-3 h-3" /> Root
              </button>
              <button
                disabled={isKilling}
                onClick={() => setConfirmKill(false)}
                className="px-1.5 py-0.5 text-xs text-slate-400 hover:text-slate-200"
              >
                ✕
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
