<div align="center">
  
# 🌌 NovaTask (proc-manager)

**The Modern, Blazing-Fast TUI System & Docker Manager.**

[![GitHub Release](https://img.shields.io/github/v/release/novitaswebworks/proc-manager?style=for-the-badge&color=cyan)](https://github.com/novitaswebworks/proc-manager/releases)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux-blue?style=for-the-badge)]()
[![Rust](https://img.shields.io/badge/Built_With-Rust-orange?style=for-the-badge&logo=rust)]()

*Stop juggling `htop`, `lazydocker`, and `systemctl`. Manage everything from a single, beautiful terminal dashboard.*

---

</div>

## ✨ Features

- **📊 Real-time Dashboard**: Instantly view global CPU, Memory, and Swap usage with color-coded gauges.
- **🌲 Process Tree View**: Toggle a hierarchical tree view (`T`) to see which processes spawned others and manage them cleanly.
- **🐳 Docker Integration**: Start, stop, restart, and stream logs for Docker containers without leaving the app.
- **⚙️ Service Manager**: Control `systemd` (Linux) and `launchd` (macOS) services with ease.
- **⚡ Command Palette**: Press `:` to open a fuzzy command palette. Type `kill node` or `restart postgres` and let NovaTask handle the rest.
- **🖱️ Mouse Support**: Full mouse support! Click tabs, scroll through lists, and navigate logs effortlessly.
- **🌐 Network Ports**: Instantly see which process is listening on which port.
- **🗂️ Workspaces**: Group related processes, containers, and services into custom workspaces to manage your entire dev stack at once.

## 🚀 Installation

### Using Homebrew (macOS & Linux)
The absolute easiest way to install and keep NovaTask updated:
```bash
brew install novitaswebworks/tap/proc-manager
```

### Using the Universal Installer (Curl)
```bash
curl -fsSL https://raw.githubusercontent.com/novitaswebworks/proc-manager/main/install.sh | bash
```

## 🎮 Quick Start

Once installed, simply type the short command to launch the dashboard:
```bash
nova
```
*(Note: You can also use `proc-manager` if you prefer!)*

### ⌨️ Keybindings
NovaTask is designed to be fully navigable by keyboard (or mouse!):

| Key / Command | Action |
|---------------|--------|
| `V` | Cycle between Views (Processes, Services, Docker, Workspaces, Ports) |
| `:` | Open Command Palette (e.g. `kill <name>`, `logs <name>`) |
| `/` | Search current list |
| `T` | Toggle Process Tree View |
| `L` | View live streaming logs for Services or Docker Containers |
| `S` | Start/Stop a Service or Container |
| `R` | Restart a Service or Container |
| `W` | Add selected item to a Workspace |
| `Enter` | View detailed Process Information & historical graphs |
| `Up/Down` or `Scroll` | Navigate lists and logs |
| `Q` or `Esc` | Quit / Go Back |

## 🛠️ Built With
- **[Rust](https://www.rust-lang.org/)**: For memory safety and blazing fast performance.
- **[Ratatui](https://ratatui.rs/)**: The premier Rust library for creating terminal user interfaces.
- **[sysinfo](https://github.com/GuillaumeGomez/sysinfo)**: Cross-platform system metrics.
- **[Bollard](https://github.com/fussybeaver/bollard)**: Async Docker API client.

## 🤝 Contributing
Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/novitaswebworks/proc-manager/issues).

---
<div align="center">
  <i>Built with ❤️ by Novitas Webworks</i>
</div>
