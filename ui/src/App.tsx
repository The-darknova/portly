import React, { useState, useEffect, useCallback, useMemo } from "react";
import { Activity, AlertCircle } from "lucide-react";
import { ListeningSocket, PortConflict, Protocol } from "./types";
import { SearchBar } from "./components/SearchBar";
import { PortTable } from "./components/PortTable";
import { ConflictBanner } from "./components/ConflictBanner";

// Check if running inside Tauri
const isTauri = typeof window !== "undefined" && ("__TAURI_INTERNALS__" in window || "__TAURI__" in window);

export const App: React.FC = () => {
  const [sockets, setSockets] = useState<ListeningSocket[]>([]);
  const [conflicts, setConflicts] = useState<PortConflict[]>([]);
  const [query, setQuery] = useState("");
  const [protocolFilter, setProtocolFilter] = useState<Protocol | "all">("all");
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const fetchSockets = useCallback(async () => {
    setIsRefreshing(true);
    setErrorMessage(null);
    try {
      if (isTauri) {
        const { invoke } = await import("@tauri-apps/api/core");
        const [data, conflictData] = await Promise.all([
          invoke<ListeningSocket[]>("scan_ports"),
          invoke<PortConflict[]>("detect_conflicts"),
        ]);
        setSockets(data);
        setConflicts(conflictData);
      } else {
        // Fallback demo dataset for browser-only development previews
        setSockets([
          {
            protocol: "tcp",
            local_address: "127.0.0.1",
            local_port: 3000,
            pid: 14201,
            process_name: "node",
            process_path: "/usr/bin/node",
            cwd: "/home/danny/projects/my-web-app",
            user: "danny",
            state: "LISTEN",
            is_elevated: false,
          },
          {
            protocol: "tcp",
            local_address: "127.0.0.1",
            local_port: 5173,
            pid: 18452,
            process_name: "vite",
            process_path: "/usr/bin/vite",
            cwd: "/home/danny/allprojects/portly/ui",
            user: "danny",
            state: "LISTEN",
            is_elevated: false,
          },
          {
            protocol: "tcp",
            local_address: "0.0.0.0",
            local_port: 8080,
            pid: 24510,
            process_name: "caddy",
            process_path: "/usr/bin/caddy",
            cwd: "/etc/caddy",
            user: "root",
            state: "LISTEN",
            is_elevated: true,
          },
        ]);
      }
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setErrorMessage(`Failed to scan ports: ${msg}`);
    } finally {
      setIsRefreshing(false);
    }
  }, []);

  useEffect(() => {
    fetchSockets();
  }, [fetchSockets]);

  // Periodic background refresh
  useEffect(() => {
    if (!autoRefresh) return;
    const timer = setInterval(() => {
      fetchSockets();
    }, 2000);
    return () => clearInterval(timer);
  }, [autoRefresh, fetchSockets]);

  const handleOpenBrowser = async (port: number) => {
    if (isTauri) {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("execute_action", {
        req: { port, action: "browser" },
      });
    } else {
      window.open(`http://localhost:${port}`, "_blank");
    }
  };

  const handleOpenTerminal = async (path: string) => {
    if (isTauri) {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("execute_action", {
        req: { port: 0, action: "terminal", target_path: path },
      });
    } else {
      alert(`Terminal command: cd ${path}`);
    }
  };

  const handleOpenFolder = async (path: string) => {
    if (isTauri) {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("execute_action", {
        req: { port: 0, action: "file_manager", target_path: path },
      });
    } else {
      alert(`File manager reveal: ${path}`);
    }
  };

  const handleOpenEditor = async (path: string) => {
    if (isTauri) {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("execute_action", {
        req: { port: 0, action: "editor", target_path: path },
      });
    } else {
      alert(`Open editor in: ${path}`);
    }
  };

  const handleKill = async (pid: number, force: boolean, escalate: boolean) => {
    try {
      if (isTauri) {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("kill_process", {
          req: { pid, force, escalate },
        });
      }
      // Re-scan after short delay
      setTimeout(fetchSockets, 500);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      alert(`Failed to kill process: ${msg}`);
    }
  };

  // Filter sockets
  const filteredSockets = useMemo(() => {
    return sockets.filter((s) => {
      // Protocol filter
      if (protocolFilter !== "all" && s.protocol.toLowerCase() !== protocolFilter) {
        return false;
      }
      // Query filter
      if (!query.trim()) return true;
      const q = query.toLowerCase();
      const portMatch = s.local_port.toString().includes(q);
      const nameMatch = s.process_name?.toLowerCase().includes(q) ?? false;
      const pidMatch = s.pid?.toString().includes(q) ?? false;
      const cwdMatch = s.cwd?.toLowerCase().includes(q) ?? false;
      return portMatch || nameMatch || pidMatch || cwdMatch;
    });
  }, [sockets, protocolFilter, query]);

  const activeTcpCount = useMemo(
    () => sockets.filter((s) => s.protocol.toLowerCase() === "tcp").length,
    [sockets]
  );
  const activeUdpCount = useMemo(
    () => sockets.filter((s) => s.protocol.toLowerCase() === "udp").length,
    [sockets]
  );

  return (
    <div className="flex flex-col h-screen bg-slate-950 text-slate-100 select-none overflow-hidden font-sans">
      {/* Top Header */}
      <header className="flex items-center justify-between px-5 py-3 bg-slate-900 border-b border-slate-800">
        <div className="flex items-center gap-2.5">
          <div className="w-8 h-8 rounded-lg bg-blue-600 flex items-center justify-center shadow-lg shadow-blue-500/20">
            <Activity className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="text-base font-bold tracking-tight text-white flex items-center gap-2">
              Portly
              <span className="text-[10px] font-semibold uppercase px-1.5 py-0.2 bg-blue-950 text-blue-400 border border-blue-800/60 rounded">
                v0.1.0
              </span>
            </h1>
            <p className="text-[11px] text-slate-400">Cross-Platform Port & Process Manager</p>
          </div>
        </div>

        <div className="flex items-center gap-4 text-xs font-mono">
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2 h-2 rounded-full bg-blue-500" />
            TCP: <span className="text-slate-200 font-semibold">{activeTcpCount}</span>
          </div>
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2 h-2 rounded-full bg-purple-500" />
            UDP: <span className="text-slate-200 font-semibold">{activeUdpCount}</span>
          </div>
          {!isTauri && (
            <div className="px-2.5 py-1 bg-amber-950/60 border border-amber-800/60 text-amber-300 rounded text-[11px] flex items-center gap-1">
              <AlertCircle className="w-3.5 h-3.5" />
              Web Preview
            </div>
          )}
        </div>
      </header>

      {/* Search & Filter Toolbar */}
      <SearchBar
        query={query}
        onQueryChange={setQuery}
        protocolFilter={protocolFilter}
        onProtocolChange={setProtocolFilter}
        onRefresh={fetchSockets}
        isRefreshing={isRefreshing}
        autoRefresh={autoRefresh}
        onToggleAutoRefresh={() => setAutoRefresh(!autoRefresh)}
      />

      {/* Error Alert */}
      {errorMessage && (
        <div className="px-4 py-2 bg-rose-950/80 border-b border-rose-800/80 text-rose-300 text-xs flex items-center gap-2">
          <AlertCircle className="w-4 h-4 shrink-0" />
          {errorMessage}
        </div>
      )}

      {/* Port Conflict Alert Banner */}
      <ConflictBanner conflicts={conflicts} />

      {/* Main Port Table */}
      <main className="flex-1 overflow-y-auto">
        <PortTable
          sockets={filteredSockets}
          onOpenBrowser={handleOpenBrowser}
          onOpenTerminal={handleOpenTerminal}
          onOpenFolder={handleOpenFolder}
          onOpenEditor={handleOpenEditor}
          onKill={handleKill}
        />
      </main>

      {/* Status Bar */}
      <footer className="flex items-center justify-between px-4 py-2 bg-slate-900/90 border-t border-slate-800 text-[11px] text-slate-400 font-mono">
        <div className="flex items-center gap-3">
          <span>Total Listeners: <strong className="text-slate-200">{sockets.length}</strong></span>
          <span>Showing: <strong className="text-slate-200">{filteredSockets.length}</strong></span>
        </div>
        <div className="flex items-center gap-2">
          <span>CLI: <code className="text-blue-400">portly list</code></span>
        </div>
      </footer>
    </div>
  );
};
