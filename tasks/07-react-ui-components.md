# Task 07: React 19 Frontend & Quick Action UI

## Objective
Build the desktop user interface using React 19, TypeScript, Tailwind CSS, and Lucide icons.

## Acceptance Criteria
- [ ] Implement responsive virtualized or optimized table for socket listings.
- [ ] Search & Filter input with real-time fuzzy matching across Port, PID, Process Name, and CWD.
- [ ] 1-Click Action buttons on each row:
  - **Kill**: Terminate process with visual confirmation and escalation status indicator.
  - **Browse**: Open `http://localhost:<port>` in default browser.
  - **Terminal**: Open user's terminal inside the process working directory.
  - **Folder**: Reveal directory in File Explorer / Finder.
- [ ] Port Conflict warning banners when multiple listeners compete.
- [ ] Polling interval toggle (e.g. 1s, 3s, 5s, manual pause).
