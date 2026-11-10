# Task 07: React 19 Frontend & Quick Action UI

## Objective
Build the desktop user interface using React 19, TypeScript, Tailwind CSS, and Lucide icons.

## Acceptance Criteria
- [x] Implement responsive virtualized or optimized table for socket listings.
- [x] Search & Filter input with real-time fuzzy matching across Port, PID, Process Name, and CWD.
- [x] 1-Click Action buttons on each row:
  - **Kill**: Terminate process with visual confirmation, force, and elevation buttons.
  - **Browse**: Open `http://localhost:<port>` in default browser.
  - **Terminal**: Open user's terminal inside the process working directory.
  - **Editor**: Open project root in Cursor, VS Code, Zed, or Sublime.
  - **Folder**: Reveal directory in File Explorer / Finder.
- [x] Port Conflict warning banners when multiple listeners compete.
- [x] Auto-refresh Live toggle (2s interval) and manual refresh button.
