<p align="center">
  <img src="./duckvideo.svg" alt="DuckVideo Studio logo" width="140" height="140" />
</p>

<h1 align="center">DuckVideo Studio</h1>

<p align="center">
  A desktop media downloader and optimizer built with Vue, TypeScript, Tabler, Tauri, yt-dlp, and FFmpeg.
</p>

<p align="center">
  <img alt="License: GPL-3.0-only" src="https://img.shields.io/badge/License-GPL--3.0--only-blue.svg" />
  <img alt="Vue" src="https://img.shields.io/badge/Vue-3-42b883.svg" />
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-5-3178c6.svg" />
  <img alt="Tauri" src="https://img.shields.io/badge/Tauri-2-24c8db.svg" />
  <img alt="yt-dlp + FFmpeg" src="https://img.shields.io/badge/yt--dlp%20%2B%20FFmpeg-local-ffb703.svg" />
</p>

## Overview

DuckVideo Studio is a desktop media utility for downloading and optimizing media locally. It uses Tauri to connect a Vue interface with native commands, allowing the app to run `yt-dlp` and FFmpeg directly on the user's machine without a custom server.

The app is focused on a local-first workflow: paste a media URL, choose the output format and quality, download with `yt-dlp`, then optionally compress or convert media with FFmpeg.

## Highlights

- Vue 3 + TypeScript frontend
- Tabler UI styling and Tabler Icons
- Simple Icons for supported platform branding
- Tauri desktop app with local commands:
  - `check_tools`
  - `download_media`
  - `compress_media`
- `yt-dlp` support for desktop media download workflows
- FFmpeg support for local compression workflows
- App icon based on `duckvideo.svg`
- GPL-3.0-only license

## Desktop Workflow

```text
Vue UI
  -> Tauri command
    -> yt-dlp downloads media locally
    -> FFmpeg compresses or converts media locally
  -> output is saved to the user's machine
```

The desktop app checks for tools in this order:

1. Bundled Tauri sidecar binaries
2. System `PATH`

This means development can use locally installed tools, while release builds can bundle the required binaries with the app.

## Tech Stack

| Layer | Technology |
| --- | --- |
| Frontend | Vue 3, TypeScript, Vite |
| UI | Tabler Core, Tabler Icons |
| Brand icons | Simple Icons |
| Desktop | Tauri 2, Rust |
| Media download | yt-dlp |
| Compression | FFmpeg |

## Project Structure

```text
.
|-- duckvideo.svg
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
`-- package.json
```

## Getting Started

Install dependencies:

```bash
npm install
```

Run the Tauri desktop app:

```bash
npm run tauri:dev
```

## Build

Build the desktop app locally:

```bash
npm run tauri:build
```

Desktop installers are generated in:

```text
src-tauri/target/release/bundle/
```

## Sidecar Binaries

Tauri expects sidecar binaries in:

```text
src-tauri/binaries/
```

Expected binary naming follows Tauri's target triple convention:

```text
yt-dlp-x86_64-pc-windows-msvc.exe
ffmpeg-x86_64-pc-windows-msvc.exe
yt-dlp-x86_64-unknown-linux-gnu
ffmpeg-x86_64-unknown-linux-gnu
yt-dlp-x86_64-apple-darwin
ffmpeg-x86_64-apple-darwin
```

The real binaries are ignored by Git. The repository keeps only:

```text
src-tauri/binaries/.gitkeep
```

## Legal Notice

DuckVideo Studio provides local tooling integration for user-controlled media workflows. Downloading content from third-party platforms may be subject to platform terms, copyright law, account permissions, and regional rules.

Users are responsible for ensuring they have the right to download, store, convert, or process any media.

## Author

Copyright (c) 2026 Ahmad Ilham Kurniawan.

## License

DuckVideo Studio is licensed under **GPL-3.0-only**.

See [LICENSE](./LICENSE) for details.
