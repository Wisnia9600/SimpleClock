<div align="center">
  <img src="ui/icon.png" width="128" alt="SimpleClock Icon" />
  <h1>SimpleClock</h1>
  <p>A beautiful, ultra-lightweight desktop clock widget equipped with a Stopwatch, Timer, and Alarms.</p>
</div>

---

## 🕒 Overview
**SimpleClock** is a modern, frameless, and compact desktop widget built for Windows. It provides three core features in a single dark-themed, glass-like UI:
- **Stopwatch**: Precision tracking with Lap support and delta time calculation.
- **Timer**: Easy-to-use countdown timer with visual SVG progress ring, audio alerts, and system notifications.
- **Alarms**: Manage daily alarms with toggle switches. The window flashes and rings when triggered.

Originally built as a bulky 120MB+ Node.js web wrapper, this project was fully rewritten using **[Tauri v2](https://v2.tauri.app/)** (Rust). The new standalone Windows executable weighs exceptionally light (~8 MB), uses Native UI drop-shadows, adaptive window bounds, and requires practically zero system resources.

## 🚀 Features
✨ **Zero Web-Frameworks**: 100% Vanilla JavaScript, HTML5, and CSS3.  
✨ **Rust-Powered**: Built on top of Tauri to deliver lightning-fast boot times and minimal RAM usage.  
✨ **Frameless adaptive UI**: Fits beautifully on your desktop and resizes precisely to match its content.  
✨ **Custom Audio**: Procedural notification sounds powered by the Web Audio API without relying on external `.mp3` files.  
✨ **Compact Mode**: Sits neatly in the corner of your screen workspace.  

## 💾 Installation
Head over to the [Releases](https://github.com/Wisnia9600/SimpleClock/releases) page and download:
- `SimpleClock_1.0.1_x64_en-US.msi` (Recommended standard Windows installer)
- `SimpleClock_1.0.1_x64-setup.exe` (NSIS alternative installer)
- `simpleclock.exe` (Standalone portable execution without installation)

## 🛠️ Build from Source
If you want to tinker around and compile it yourself, you need [Rust](https://rustup.rs/) and [Tauri CLI](https://v2.tauri.app/start/prerequisites/):

```bash
# Clone the repository
git clone https://github.com/Wisnia9600/SimpleClock.git
cd SimpleClock

# Run in development mode
cargo tauri dev

# Build for release
cargo tauri build
```

## 📜 License
MIT License
