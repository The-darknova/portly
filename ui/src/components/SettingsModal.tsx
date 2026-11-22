import React, { useEffect, useState } from "react";
import { X, Settings2, Check, Save, Terminal, Link2, Unlink } from "lucide-react";
import { CliLinkStatus, PortlyConfig } from "../types";

const isTauri =
  typeof window !== "undefined" &&
  ("__TAURI_INTERNALS__" in window || "__TAURI__" in window);

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  config: PortlyConfig;
  onSave: (config: PortlyConfig) => Promise<void>;
}

export const SettingsModal: React.FC<SettingsModalProps> = ({
  isOpen,
  onClose,
  config,
  onSave,
}) => {
  const [editor, setEditor] = useState<string>(config.preferred_editor || "auto");
  const [terminal, setTerminal] = useState<string>(config.preferred_terminal || "auto");
  const [refreshInterval, setRefreshInterval] = useState<number>(config.auto_refresh_secs);
  const [useHttps, setUseHttps] = useState<boolean>(config.use_https);
  const [isSaving, setIsSaving] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [cliStatus, setCliStatus] = useState<CliLinkStatus | null>(null);
  const [isCliBusy, setIsCliBusy] = useState(false);
  const [cliError, setCliError] = useState<string | null>(null);

  useEffect(() => {
    if (!isOpen || !isTauri) return;
    let active = true;
    (async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const status = await invoke<CliLinkStatus>("get_cli_link_status");
        if (active) {
          setCliStatus(status);
          setCliError(null);
        }
      } catch (err: unknown) {
        if (active) {
          setCliError(err instanceof Error ? err.message : String(err));
        }
      }
    })();
    return () => {
      active = false;
    };
  }, [isOpen]);

  if (!isOpen) return null;

  const handleInstallCli = async () => {
    if (!isTauri) return;
    setIsCliBusy(true);
    setCliError(null);
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const status = await invoke<CliLinkStatus>("install_cli_link");
      setCliStatus(status);
    } catch (err: unknown) {
      setCliError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsCliBusy(false);
    }
  };

  const handleUninstallCli = async () => {
    if (!isTauri) return;
    setIsCliBusy(true);
    setCliError(null);
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const status = await invoke<CliLinkStatus>("uninstall_cli_link");
      setCliStatus(status);
    } catch (err: unknown) {
      setCliError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsCliBusy(false);
    }
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await onSave({
        ...config,
        preferred_editor: editor === "auto" ? null : editor,
        preferred_terminal: terminal === "auto" ? null : terminal,
        auto_refresh_secs: refreshInterval,
        use_https: useHttps,
      });
      setSaveSuccess(true);
      setTimeout(() => {
        setSaveSuccess(false);
        onClose();
      }, 600);
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div className="bg-slate-900 border border-slate-800 rounded-xl max-w-md w-full shadow-2xl p-6 text-slate-100 animate-in fade-in zoom-in-95 duration-150">
        <div className="flex items-center justify-between pb-4 border-b border-slate-800">
          <div className="flex items-center gap-2">
            <Settings2 className="w-5 h-5 text-indigo-400" />
            <h2 className="text-base font-semibold">Portly Preferences</h2>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-md text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="py-4 space-y-4 text-sm">
          {/* Preferred Code Editor */}
          <div>
            <label className="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1.5">
              Preferred Code Editor
            </label>
            <select
              value={editor}
              onChange={(e) => setEditor(e.target.value)}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-200 focus:outline-none focus:border-indigo-500"
            >
              <option value="auto">Auto-detect (Cursor, Code, Zed, Subl)</option>
              <option value="cursor">Cursor (cursor)</option>
              <option value="code">Visual Studio Code (code)</option>
              <option value="zed">Zed (zed)</option>
              <option value="subl">Sublime Text (subl)</option>
            </select>
          </div>

          {/* Preferred Terminal */}
          <div>
            <label className="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1.5">
              Preferred Terminal
            </label>
            <select
              value={terminal}
              onChange={(e) => setTerminal(e.target.value)}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-200 focus:outline-none focus:border-indigo-500"
            >
              <option value="auto">Auto-detect (Ghostty, Kitty, Alacritty)</option>
              <option value="ghostty">Ghostty</option>
              <option value="kitty">Kitty</option>
              <option value="alacritty">Alacritty</option>
              <option value="foot">Foot</option>
              <option value="wt.exe">Windows Terminal (wt.exe)</option>
              <option value="powershell">PowerShell</option>
            </select>
          </div>

          {/* Auto-Refresh Interval */}
          <div>
            <label className="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1.5">
              Background Scan Cadence
            </label>
            <select
              value={refreshInterval}
              onChange={(e) => setRefreshInterval(Number(e.target.value))}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2 text-slate-200 focus:outline-none focus:border-indigo-500"
            >
              <option value={1}>Every 1 second</option>
              <option value={2}>Every 2 seconds (Default)</option>
              <option value={5}>Every 5 seconds</option>
              <option value={10}>Every 10 seconds</option>
            </select>
          </div>

          {/* HTTPS Toggle */}
          <div className="flex items-center justify-between pt-1">
            <div>
              <span className="font-medium text-slate-200">Default to HTTPS</span>
              <p className="text-xs text-slate-400">Open localhost URLs with https:// by default</p>
            </div>
            <input
              type="checkbox"
              checked={useHttps}
              onChange={(e) => setUseHttps(e.target.checked)}
              className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-indigo-600 focus:ring-indigo-500 cursor-pointer"
            />
          </div>

          {/* Command-Line Integration */}
          <div className="pt-3 border-t border-slate-800 space-y-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-1.5">
                <Terminal className="w-3.5 h-3.5 text-indigo-400" />
                <span className="text-xs font-semibold text-slate-300 uppercase tracking-wider">
                  CLI Command (<code className="normal-case text-indigo-300">portly</code>)
                </span>
              </div>
              {cliStatus && (
                <span
                  className={`text-[11px] font-medium px-2 py-0.5 rounded border ${
                    cliStatus.state === "Linked" || cliStatus.state === "SystemManaged"
                      ? "bg-emerald-950/60 text-emerald-300 border-emerald-800/60"
                      : cliStatus.state === "LinkedNotInPath"
                      ? "bg-amber-950/60 text-amber-300 border-amber-800/60"
                      : "bg-slate-950 text-slate-400 border-slate-800"
                  }`}
                >
                  {cliStatus.state === "SystemManaged"
                    ? "System Managed"
                    : cliStatus.state === "Linked"
                    ? "Linked"
                    : cliStatus.state === "LinkedNotInPath"
                    ? "Not in PATH"
                    : "Not Linked"}
                </span>
              )}
            </div>

            {isTauri ? (
              <>
                <p className="text-xs text-slate-400 leading-relaxed">
                  {cliStatus ? cliStatus.message : "Checking CLI PATH registration..."}
                </p>
                {cliError && (
                  <p className="text-xs text-rose-400">{cliError}</p>
                )}
                {cliStatus && cliStatus.state !== "SystemManaged" && (
                  <div className="flex items-center gap-2 pt-1">
                    {cliStatus.state !== "Linked" && (
                      <button
                        type="button"
                        onClick={handleInstallCli}
                        disabled={isCliBusy}
                        className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-100 border border-slate-700 transition disabled:opacity-50"
                      >
                        <Link2 className="w-3.5 h-3.5 text-indigo-400" />
                        {isCliBusy ? "Linking..." : "Install portly in PATH"}
                      </button>
                    )}
                    {cliStatus.can_uninstall && (
                      <button
                        type="button"
                        onClick={handleUninstallCli}
                        disabled={isCliBusy}
                        className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-slate-950 hover:bg-slate-800 text-slate-300 border border-slate-800 transition disabled:opacity-50"
                      >
                        <Unlink className="w-3.5 h-3.5 text-slate-400" />
                        Remove CLI Link
                      </button>
                    )}
                  </div>
                )}
              </>
            ) : (
              <p className="text-xs text-slate-400">
                CLI linking is managed inside the desktop application.
              </p>
            )}
          </div>
        </div>

        {/* Footer actions */}
        <div className="pt-4 border-t border-slate-800 flex items-center justify-end gap-2">
          <button
            onClick={onClose}
            className="px-3.5 py-1.5 rounded-lg text-xs font-medium text-slate-300 hover:bg-slate-800 transition"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-medium bg-indigo-600 hover:bg-indigo-500 text-white shadow transition disabled:opacity-50"
          >
            {saveSuccess ? (
              <>
                <Check className="w-3.5 h-3.5 text-white" />
                Saved
              </>
            ) : (
              <>
                <Save className="w-3.5 h-3.5" />
                {isSaving ? "Saving..." : "Save Preferences"}
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
};
