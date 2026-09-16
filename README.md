# DuckVideo Studio

DuckVideo Studio is a Vue + TypeScript media tool with a static web interface and a desktop-first Tauri app. The desktop version is designed to run locally with `yt-dlp` for media downloads and FFmpeg for compression, without requiring a custom server.

Copyright (c) 2026 Ahmad Ilham Kurniawan.

Licensed under GPL-3.0-only.

## Features

- Multi-page static website: `index.html`, `download.html`, `compress.html`, `about.html`, and `settings.html`
- Vue 3 + TypeScript frontend
- Tabler UI styling and Tabler Icons
- Simple Icons for platform logos
- Tauri desktop shell
- Local desktop commands for:
  - checking `yt-dlp` and FFmpeg availability
  - downloading media with `yt-dlp`
  - compressing local video files with FFmpeg
- GitHub Actions workflow for building installers on Windows, macOS, and Linux
- App icon and website favicon generated from `duckvideo.svg`

## Project Structure

```text
.
|-- index.html
|-- download.html
|-- compress.html
|-- about.html
|-- settings.html
|-- duckvideo.svg
|-- public/
|-- src/
|   |-- App.vue
|   |-- desktop.ts
|   |-- mediaTools.ts
|   `-- components/
|-- src-tauri/
|   |-- src/main.rs
|   |-- tauri.conf.json
|   |-- icons/
|   `-- binaries/
`-- .github/workflows/tauri-build.yml
```

## Requirements

For web development:

- Node.js 20+
- npm

For desktop development:

- Rust stable
- Tauri system dependencies for your OS
- Optional locally installed `yt-dlp` and FFmpeg for testing outside bundled builds

GitHub Actions downloads `yt-dlp` and FFmpeg automatically before building installers.

## Install

```bash
npm install
```

## Run Web Development Server

```bash
npm run dev
```

Open:

```text
http://127.0.0.1:5173/
```

Static pages:

```text
http://127.0.0.1:5173/index.html
http://127.0.0.1:5173/download.html
http://127.0.0.1:5173/compress.html
http://127.0.0.1:5173/about.html
http://127.0.0.1:5173/settings.html
```

## Build Static Web Version

```bash
npm run build
```

The static output is generated in:

```text
dist/
```

You can deploy `dist/` to static hosting.

## Run Desktop App

```bash
npm run tauri:dev
```

The desktop app can use local `yt-dlp` and FFmpeg from:

1. bundled Tauri sidecars
2. the system `PATH`

## Build Desktop Installer Locally

```bash
npm run tauri:build
```

Installer bundles are generated under:

```text
src-tauri/target/release/bundle/
```

## Sidecar Tools

The Tauri config expects sidecar binaries in:

```text
src-tauri/binaries/
```

The expected names follow Tauri's target triple convention, for example:

```text
yt-dlp-x86_64-pc-windows-msvc.exe
ffmpeg-x86_64-pc-windows-msvc.exe
yt-dlp-x86_64-unknown-linux-gnu
ffmpeg-x86_64-unknown-linux-gnu
yt-dlp-x86_64-apple-darwin
ffmpeg-x86_64-apple-darwin
```

The repository keeps `src-tauri/binaries/.gitkeep`, but real binaries are ignored by Git. GitHub Actions downloads them automatically.

## GitHub Actions

The workflow at:

```text
.github/workflows/tauri-build.yml
```

builds installers for:

- Windows
- Linux
- macOS

It also downloads:

- `yt-dlp`
- FFmpeg

before running:

```bash
npm run tauri:build
```

Build artifacts are uploaded from Tauri's bundle output.

## Desktop Commands

The frontend calls Tauri commands through `src/desktop.ts`.

Available Rust commands:

- `check_tools`
- `download_media`
- `compress_media`

These are implemented in:

```text
src-tauri/src/main.rs
```

## Legal Notice

DuckVideo Studio provides local tooling integration for user-controlled media workflows. Downloading content from third-party platforms may be subject to platform terms, copyright law, account permissions, and regional rules. Users are responsible for ensuring they have the right to download and process any media.

## License

GPL-3.0-only. See [LICENSE](LICENSE).
