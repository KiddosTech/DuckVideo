import type { Component } from 'vue';
import type { SimpleIcon } from 'simple-icons';

export type MediaKind = 'video' | 'image' | 'audio';
export type JobStatus = 'Ready' | 'Queued' | 'Analyzing' | 'Compressed' | 'Desktop Ready' | 'Failed';

export interface Platform {
  id: string;
  name: string;
  domains: string[];
  color: string;
  icon: Component | SimpleIcon;
  media: MediaKind[];
}

export interface DownloadJob {
  id: number;
  url: string;
  platform: Platform;
  format: string;
  quality: string;
  status: JobStatus;
  estimatedSize: string;
  note: string;
}

export interface LocalAsset {
  id: number;
  file: File;
  name: string;
  type: 'image' | 'video' | 'other';
  originalSize: number;
  outputSize?: number;
  outputUrl?: string;
  status: JobStatus;
}
