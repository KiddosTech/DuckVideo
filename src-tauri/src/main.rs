#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tauri::{AppHandle, Manager};

const TARGET_TRIPLE: &str = env!("TARGET_TRIPLE");

#[derive(Debug, Serialize)]
struct ToolInfo {
    available: bool,
    bundled: bool,
    path: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Serialize)]
struct ToolStatus {
    yt_dlp: ToolInfo,
    ffmpeg: ToolInfo,
    output_dir: String,
}

#[derive(Debug, Deserialize)]
struct DownloadRequest {
    url: String,
    format: String,
    quality: String,
    output_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CompressRequest {
    input_path: String,
    quality: Option<u8>,
    max_width: Option<u32>,
    output_dir: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProcessResult {
    success: bool,
    command: String,
    output_dir: String,
    stdout: String,
    stderr: String,
}

#[tauri::command]
fn check_tools(app: AppHandle) -> Result<ToolStatus, String> {
    Ok(ToolStatus {
        yt_dlp: inspect_tool(&app, "yt-dlp"),
        ffmpeg: inspect_tool(&app, "ffmpeg"),
        output_dir: default_output_dir()?.to_string_lossy().to_string(),
    })
}

#[tauri::command]
fn download_media(app: AppHandle, request: DownloadRequest) -> Result<ProcessResult, String> {
    let yt_dlp = find_tool(&app, "yt-dlp").ok_or("yt-dlp was not found. Bundle it with the app or install it on PATH.")?;
    let output_dir = resolve_output_dir(request.output_dir)?;
    fs::create_dir_all(&output_dir).map_err(|error| error.to_string())?;

    let mut args = vec![
        "--newline".to_string(),
        "--no-playlist".to_string(),
        "-o".to_string(),
        output_dir.join("%(title).200B.%(ext)s").to_string_lossy().to_string(),
    ];

    if request.format.eq_ignore_ascii_case("MP3") {
        args.extend(["-x".to_string(), "--audio-format".to_string(), "mp3".to_string()]);
    } else {
        args.extend(["-f".to_string(), format_selector(&request.format, &request.quality)]);
        if !request.format.eq_ignore_ascii_case("Original") {
            args.extend(["--merge-output-format".to_string(), request.format.to_lowercase()]);
        }
    }

    args.push(request.url);
    run_process(&yt_dlp, &args, output_dir)
}

#[tauri::command]
fn compress_media(app: AppHandle, request: CompressRequest) -> Result<ProcessResult, String> {
    let ffmpeg = find_tool(&app, "ffmpeg").ok_or("FFmpeg was not found. Bundle it with the app or install it on PATH.")?;
    let input_path = PathBuf::from(&request.input_path);
    if !input_path.exists() {
        return Err("Input file does not exist.".to_string());
    }

    let output_dir = resolve_output_dir(request.output_dir)?;
    fs::create_dir_all(&output_dir).map_err(|error| error.to_string())?;

    let stem = input_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("duckvideo-output");
    let output_path = output_dir.join(format!("{stem}-compressed.mp4"));
    let quality = request.quality.unwrap_or(72).clamp(1, 100);
    let crf = 36_u8.saturating_sub(quality / 5).clamp(18, 35).to_string();
    let max_width = request.max_width.unwrap_or(1920).max(320);
    let scale_filter = format!("scale='min({max_width},iw)':-2");

    let args = vec![
        "-y".to_string(),
        "-i".to_string(),
        input_path.to_string_lossy().to_string(),
        "-vf".to_string(),
        scale_filter,
        "-c:v".to_string(),
        "libx264".to_string(),
        "-preset".to_string(),
        "medium".to_string(),
        "-crf".to_string(),
        crf,
        "-c:a".to_string(),
        "aac".to_string(),
        "-b:a".to_string(),
        "128k".to_string(),
        output_path.to_string_lossy().to_string(),
    ];

    run_process(&ffmpeg, &args, output_dir)
}

fn inspect_tool(app: &AppHandle, name: &str) -> ToolInfo {
    match find_tool(app, name) {
        Some(path) => {
            let version = Command::new(&path)
                .arg("--version")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .ok()
                .and_then(|output| {
                    first_line(&String::from_utf8_lossy(&output.stdout))
                        .or_else(|| first_line(&String::from_utf8_lossy(&output.stderr)))
                });
            ToolInfo {
                available: true,
                bundled: is_bundled_path(app, &path),
                path: Some(path.to_string_lossy().to_string()),
                version,
            }
        }
        None => ToolInfo {
            available: false,
            bundled: false,
            path: None,
            version: None,
        },
    }
}

fn find_tool(app: &AppHandle, name: &str) -> Option<PathBuf> {
    bundled_candidates(app, name)
        .into_iter()
        .find(|path| path.is_file())
        .or_else(|| find_on_path(name))
}

fn bundled_candidates(app: &AppHandle, name: &str) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(resource_dir) = app.path().resource_dir() {
        roots.push(resource_dir);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            roots.push(parent.to_path_buf());
            roots.push(parent.join("resources"));
        }
    }

    let mut names = vec![
        name.to_string(),
        format!("{name}-{TARGET_TRIPLE}"),
    ];
    if cfg!(windows) {
        names.push(format!("{name}.exe"));
        names.push(format!("{name}-{TARGET_TRIPLE}.exe"));
    }

    roots
        .into_iter()
        .flat_map(|root| {
            names.iter().flat_map(move |candidate| {
                [
                    root.join(candidate),
                    root.join("binaries").join(candidate),
                    root.join("resources").join(candidate),
                ]
            })
        })
        .collect()
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .flat_map(|dir| {
                if cfg!(windows) {
                    vec![dir.join(format!("{name}.exe")), dir.join(name)]
                } else {
                    vec![dir.join(name)]
                }
            })
            .find(|path| path.is_file())
    })
}

fn is_bundled_path(app: &AppHandle, path: &Path) -> bool {
    app.path()
        .resource_dir()
        .ok()
        .and_then(|resource_dir| path.canonicalize().ok().zip(resource_dir.canonicalize().ok()))
        .is_some_and(|(path, resource_dir)| path.starts_with(resource_dir))
}

fn resolve_output_dir(output_dir: Option<String>) -> Result<PathBuf, String> {
    match output_dir {
        Some(path) if !path.trim().is_empty() => Ok(PathBuf::from(path)),
        _ => default_output_dir(),
    }
}

fn default_output_dir() -> Result<PathBuf, String> {
    let home = env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
        .ok_or("Could not resolve the user home directory.")?;
    Ok(home.join("Downloads").join("DuckVideo Studio"))
}

fn format_selector(format: &str, quality: &str) -> String {
    let height = quality.trim_end_matches('p');
    if format.eq_ignore_ascii_case("Original") || quality.eq_ignore_ascii_case("Original") {
        return "bestvideo+bestaudio/best".to_string();
    }
    if height.chars().all(|character| character.is_ascii_digit()) {
        return format!("bestvideo[height<={height}]+bestaudio/best[height<={height}]/best");
    }
    "bestvideo+bestaudio/best".to_string()
}

fn run_process(program: &Path, args: &[String], output_dir: PathBuf) -> Result<ProcessResult, String> {
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| error.to_string())?;

    Ok(ProcessResult {
        success: output.status.success(),
        command: format!("{} {}", program.to_string_lossy(), args.join(" ")),
        output_dir: output_dir.to_string_lossy().to_string(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

fn first_line(value: &str) -> Option<String> {
    value
        .lines()
        .next()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_tools, download_media, compress_media])
        .run(tauri::generate_context!())
        .expect("error while running DuckVideo Studio");
}
