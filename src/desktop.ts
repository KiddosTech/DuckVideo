import { invoke } from '@tauri-apps/api/core';

export interface ToolInfo {
  available: boolean;
  bundled: boolean;
  path: string | null;
  version: string | null;
}

export interface ToolStatus {
  yt_dlp: ToolInfo;
  ffmpeg: ToolInfo;
  output_dir: string;
}

export interface ProcessResult {
  success: boolean;
  command: string;
  output_dir: string;
  stdout: string;
  stderr: string;
}

export const isDesktopRuntime = (): boolean => '__TAURI_INTERNALS__' in window;

export const checkDesktopTools = (): Promise<ToolStatus> => invoke<ToolStatus>('check_tools');

export const downloadDesktopMedia = (request: {
  url: string;
  format: string;
  quality: string;
  outputDir?: string;
}): Promise<ProcessResult> =>
  invoke<ProcessResult>('download_media', {
    request: {
      url: request.url,
      format: request.format,
      quality: request.quality,
      output_dir: request.outputDir
    }
  });

export const compressDesktopMedia = (request: {
  inputPath: string;
  quality?: number;
  maxWidth?: number;
  outputDir?: string;
}): Promise<ProcessResult> =>
  invoke<ProcessResult>('compress_media', {
    request: {
      input_path: request.inputPath,
      quality: request.quality,
      max_width: request.maxWidth,
      output_dir: request.outputDir
    }
  });
