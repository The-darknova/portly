import React from "react";
import { Search, RefreshCw, X } from "lucide-react";
import { Protocol } from "../types";

interface SearchBarProps {
  query: string;
  onQueryChange: (q: string) => void;
  protocolFilter: Protocol | "all";
  onProtocolChange: (proto: Protocol | "all") => void;
  onRefresh: () => void;
  isRefreshing: boolean;
  autoRefresh: boolean;
  onToggleAutoRefresh: () => void;
}

export const SearchBar: React.FC<SearchBarProps> = ({
  query,
  onQueryChange,
  protocolFilter,
  onProtocolChange,
  onRefresh,
  isRefreshing,
  autoRefresh,
  onToggleAutoRefresh,
}) => {
  return (
    <div className="flex flex-col sm:flex-row items-center justify-between gap-3 p-4 bg-slate-900/80 border-b border-slate-800 backdrop-blur-md">
      <div className="relative flex-1 w-full">
        <Search className="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
        <input
          type="text"
          value={query}
          onChange={(e) => onQueryChange(e.target.value)}
          placeholder="Filter by port, PID, process name, or directory..."
          className="w-full pl-10 pr-9 py-2 bg-slate-950 border border-slate-700/60 rounded-lg text-sm text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 transition-all"
        />
        {query && (
          <button
            onClick={() => onQueryChange("")}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-200"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>

      <div className="flex items-center gap-2 w-full sm:w-auto justify-between sm:justify-start">
        <div className="flex bg-slate-950 p-1 rounded-lg border border-slate-800 text-xs font-medium">
          {(["all", "tcp", "udp"] as const).map((proto) => (
            <button
              key={proto}
              onClick={() => onProtocolChange(proto)}
              className={`px-3 py-1 rounded-md transition-all ${
                protocolFilter === proto
                  ? "bg-blue-600 text-white font-semibold shadow-sm"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              {proto.toUpperCase()}
            </button>
          ))}
        </div>

        <button
          onClick={onToggleAutoRefresh}
          title={autoRefresh ? "Auto-refresh active (2s)" : "Auto-refresh paused"}
          className={`px-3 py-1.5 rounded-lg border text-xs font-medium flex items-center gap-1.5 transition-all ${
            autoRefresh
              ? "bg-emerald-950/40 border-emerald-700/60 text-emerald-400"
              : "bg-slate-950 border-slate-800 text-slate-400 hover:text-slate-200"
          }`}
        >
          <span
            className={`w-2 h-2 rounded-full ${
              autoRefresh ? "bg-emerald-400 animate-pulse" : "bg-slate-500"
            }`}
          />
          Live
        </button>

        <button
          onClick={onRefresh}
          disabled={isRefreshing}
          className="p-2 bg-slate-800/80 hover:bg-slate-700 text-slate-200 rounded-lg border border-slate-700/50 transition-all active:scale-95 disabled:opacity-50"
          title="Refresh sockets now"
        >
          <RefreshCw className={`w-4 h-4 ${isRefreshing ? "animate-spin text-blue-400" : ""}`} />
        </button>
      </div>
    </div>
  );
};
