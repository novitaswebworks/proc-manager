# 🚀 Awesome TUI / Community Launch Pitch

Use the text below to easily copy and paste into submission forms like [Awesome TUI](https://awesometui.com/contribute), Reddit (`r/rust`, `r/commandline`), Hacker News, or other developer communities!

---

### Project Name
`nman` (Novitas Manager)

### GitHub Repository URL
https://github.com/novitaswebworks/proc-manager

### Short Description (Tagline)
A blazing-fast, modern TUI dashboard for managing processes, Docker containers, and system services in one place.

### Long Description
`nman` (short for Novitas Manager) is a cross-platform (macOS & Linux) TUI written in Rust that unifies system operations. Instead of juggling `htop`, `lazydocker`, and `systemctl`, `nman` brings them into a single, beautiful terminal dashboard. 

**Key Features:**
- **System Dashboard:** Real-time, color-coded gauges for CPU, Memory, and Swap.
- **Process Tree View:** Easily visualize process hierarchies (parent/child) and manage them.
- **Docker Integration:** Start, stop, restart, and stream logs for containers interactively.
- **Service Management:** Seamlessly control `systemd` (Linux) and `launchd` (macOS) services.
- **Fuzzy Command Palette:** Press `:` to instantly fuzzy-search and execute commands like `kill node` or `logs nginx`.
- **Interactive Log Filtering:** Press `/` while streaming logs to instantly filter output.
- **Full Mouse Support:** Click tabs and scroll lists/logs effortlessly.

### Tags / Categories
`tui`, `rust`, `system-monitor`, `docker`, `process-manager`, `devops`, `ratatui`

### Installation
```bash
brew install novitaswebworks/tap/proc-manager
```
*or*
```bash
curl -fsSL https://raw.githubusercontent.com/novitaswebworks/proc-manager/main/install.sh | bash
```

### Media
- **Demo GIF:** https://raw.githubusercontent.com/novitaswebworks/proc-manager/main/demo.gif

---
*Tip: When posting to Reddit, upload the `demo.gif` file directly as the post image/video so it autoplays for users scrolling by!*
