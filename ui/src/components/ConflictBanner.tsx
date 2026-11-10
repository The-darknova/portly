import React, { useState } from "react";
import { AlertTriangle, ChevronDown, ChevronUp } from "lucide-react";
import { PortConflict } from "../types";

interface ConflictBannerProps {
  conflicts: PortConflict[];
}

export const ConflictBanner: React.FC<ConflictBannerProps> = ({ conflicts }) => {
  const [expanded, setExpanded] = useState(false);

  if (conflicts.length === 0) return null;

  return (
    <div className="bg-amber-950/70 border-b border-amber-800/80 px-4 py-2.5 text-amber-200 text-xs">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <AlertTriangle className="w-4 h-4 text-amber-400 shrink-0" />
          <span className="font-semibold">
            {conflicts.length} Port Conflict{conflicts.length > 1 ? "s" : ""} Detected:
          </span>
          <span className="text-amber-300">
            {conflicts.map((c) => `Port ${c.port} (${c.protocol.toUpperCase()})`).join(", ")}
          </span>
        </div>
        <button
          onClick={() => setExpanded(!expanded)}
          className="flex items-center gap-1 text-amber-400 hover:text-amber-200 font-medium px-2 py-0.5 rounded hover:bg-amber-900/40 transition-all"
        >
          {expanded ? "Hide Details" : "Show Details"}
          {expanded ? <ChevronUp className="w-3.5 h-3.5" /> : <ChevronDown className="w-3.5 h-3.5" />}
        </button>
      </div>

      {expanded && (
        <div className="mt-2.5 pt-2 border-t border-amber-800/50 space-y-2">
          {conflicts.map((c) => (
            <div key={`${c.protocol}-${c.port}`} className="bg-amber-900/30 p-2 rounded border border-amber-800/40 font-mono">
              <div className="font-bold text-amber-300 mb-1">
                Port {c.port} &bull; Competing PIDs: {c.distinct_pids.join(", ") || "Unknown (Root)"}
              </div>
              <div className="space-y-0.5 text-[11px] text-amber-200/90">
                {c.sockets.map((s, idx) => (
                  <div key={idx} className="flex items-center gap-2">
                    <span className="text-amber-400">&bull;</span>
                    <span>{s.local_address}</span>
                    <span>-</span>
                    <span className="text-white font-semibold">{s.process_name || "Unknown"}</span>
                    <span>(PID: {s.pid ?? "-"})</span>
                    {s.cwd && <span className="text-amber-400/80 truncate max-w-sm">[{s.cwd}]</span>}
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
