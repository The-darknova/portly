import React, { useState } from "react";
import { Copy, Check, ShieldAlert, Cpu } from "lucide-react";
import { ListeningSocket } from "../types";
import { QuickActions } from "./QuickActions";

interface PortTableProps {
  sockets: ListeningSocket[];
  onOpenBrowser: (port: number) => void;
  onOpenTerminal: (path: string) => void;
  onOpenFolder: (path: string) => void;
  onKill: (pid: number, force: boolean, escalate: boolean) => Promise<void>;
}

export const PortTable: React.FC<PortTableProps> = ({
  sockets,
  onOpenBrowser,
  onOpenTerminal,
  onOpenFolder,
  onKill,
}) => {
  const [copiedText, setCopiedText] = useState<string | null>(null);

  const copyToClipboard = (text: string, e: React.MouseEvent) => {
    e.stopPropagation();
    navigator.clipboard.writeText(text);
    setCopiedText(text);
    setTimeout(() => setCopiedText(null), 1500);
  };

  if (sockets.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-slate-500">
        <Cpu className="w-12 h-12 stroke-[1.2] mb-3 text-slate-600" />
        <p className="text-base font-medium">No matching sockets found</p>
        <p className="text-xs text-slate-500 mt-1">Try adjusting your search query or protocol filter</p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full text-left text-sm border-collapse">
        <thead>
          <tr className="border-b border-slate-800/80 bg-slate-900/40 text-xs font-semibold text-slate-400 uppercase tracking-wider">
            <th className="py-3 px-4 w-20">Proto</th>
            <th className="py-3 px-4 w-24">Port</th>
            <th className="py-3 px-4 w-44">Process</th>
            <th className="py-3 px-4 w-24">PID</th>
            <th className="py-3 px-4">Working Directory (CWD)</th>
            <th className="py-3 px-4 w-32">Address</th>
            <th className="py-3 px-4 w-36 text-right">Actions</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-slate-800/50">
          {sockets.map((socket, idx) => {
            const isTcp = socket.protocol.toLowerCase() === "tcp";
            const rowKey = `${socket.protocol}-${socket.local_port}-${socket.pid || idx}`;

            return (
              <tr
                key={rowKey}
                className="hover:bg-slate-800/30 transition-colors group"
              >
                {/* Protocol */}
                <td className="py-3 px-4">
                  <span
                    className={`inline-flex items-center px-2 py-0.5 rounded text-[11px] font-bold ${
                      isTcp
                        ? "bg-blue-950/60 text-blue-400 border border-blue-800/50"
                        : "bg-purple-950/60 text-purple-400 border border-purple-800/50"
                    }`}
                  >
                    {socket.protocol.toUpperCase()}
                  </span>
                </td>

                {/* Port */}
                <td className="py-3 px-4 font-mono font-bold text-emerald-400 text-sm">
                  {socket.local_port}
                </td>

                {/* Process Name */}
                <td className="py-3 px-4 font-medium text-slate-200">
                  {socket.process_name ? (
                    <span className="truncate block max-w-[160px]" title={socket.process_path || socket.process_name}>
                      {socket.process_name}
                    </span>
                  ) : (
                    <span className="inline-flex items-center gap-1 text-xs text-amber-500 font-normal">
                      <ShieldAlert className="w-3.5 h-3.5" /> System / Root
                    </span>
                  )}
                </td>

                {/* PID */}
                <td className="py-3 px-4 font-mono text-xs text-slate-400">
                  {socket.pid ?? "-"}
                </td>

                {/* Working Directory */}
                <td className="py-3 px-4 font-mono text-xs text-slate-400 max-w-md">
                  {socket.cwd ? (
                    <div className="flex items-center gap-2 group/cwd">
                      <span className="truncate text-slate-300" title={socket.cwd}>
                        {socket.cwd}
                      </span>
                      <button
                        onClick={(e) => copyToClipboard(socket.cwd!, e)}
                        title="Copy working directory"
                        className="opacity-0 group-hover/cwd:opacity-100 text-slate-500 hover:text-slate-300 transition-opacity p-0.5 rounded"
                      >
                        {copiedText === socket.cwd ? (
                          <Check className="w-3.5 h-3.5 text-emerald-400" />
                        ) : (
                          <Copy className="w-3.5 h-3.5" />
                        )}
                      </button>
                    </div>
                  ) : (
                    <span className="text-slate-600">-</span>
                  )}
                </td>

                {/* Address */}
                <td className="py-3 px-4 font-mono text-xs text-slate-400 truncate max-w-[120px]">
                  {socket.local_address}
                </td>

                {/* Actions */}
                <td className="py-3 px-4 text-right">
                  <QuickActions
                    socket={socket}
                    onOpenBrowser={onOpenBrowser}
                    onOpenTerminal={onOpenTerminal}
                    onOpenFolder={onOpenFolder}
                    onKill={onKill}
                  />
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};
