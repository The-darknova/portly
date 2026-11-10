import React, { useState } from "react";
import { X, Settings2, Check, Save } from "lucide-react";
import { PortlyConfig } from "../types";

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

  if (!isOpen) return null;

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
