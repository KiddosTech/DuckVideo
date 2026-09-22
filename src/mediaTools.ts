import type { DownloadJob, LocalAsset, Platform } from './types';

export const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  const index = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / 1024 ** index).toFixed(index === 0 ? 0 : 1)} ${units[index]}`;
};

export const detectPlatform = (url: string, platforms: Platform[]): Platform => {
  try {
    const host = new URL(url).hostname.replace(/^www\./, '').toLowerCase();
    return platforms.find((platform) => platform.domains.some((domain) => host.includes(domain))) ?? platforms[0];
  } catch {
    return platforms[0];
  }
};

export const createDownloadJob = (
  url: string,
  platforms: Platform[],
  format: string,
  quality: string
): DownloadJob => {
  const platform = detectPlatform(url, platforms);
  const needsDesktopTool = platform.id !== 'direct';

  return {
    id: Date.now(),
    url,
    platform,
    format,
    quality,
    status: needsDesktopTool ? 'Desktop Ready' : 'Ready',
    estimatedSize: needsDesktopTool ? 'Desktop job' : quality === 'Original' ? 'Original' : '12-48 MB',
    note: needsDesktopTool
      ? 'The desktop app can process this through the local download engine.'
      : 'Direct files can be processed by the browser if CORS allows access.'
  };
};

export const fileToAsset = (file: File): LocalAsset => ({
  id: Date.now() + Math.random(),
  file,
  name: file.name,
  type: file.type.startsWith('image/') ? 'image' : file.type.startsWith('video/') ? 'video' : 'other',
  originalSize: file.size,
  status: 'Queued'
});

export const compressImage = async (asset: LocalAsset, quality: number, maxWidth: number): Promise<LocalAsset> => {
  const bitmap = await createImageBitmap(asset.file);
  const scale = Math.min(1, maxWidth / bitmap.width);
  const width = Math.max(1, Math.round(bitmap.width * scale));
  const height = Math.max(1, Math.round(bitmap.height * scale));

  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const context = canvas.getContext('2d');
  if (!context) throw new Error('Canvas tidak tersedia di browser ini.');

  context.drawImage(bitmap, 0, 0, width, height);
  const blob = await new Promise<Blob>((resolve, reject) => {
    canvas.toBlob(
      (result) => {
        if (result) resolve(result);
        else reject(new Error('Failed to create the image output.'));
      },
      'image/webp',
      quality
    );
  });

  return {
    ...asset,
    outputSize: blob.size,
    outputUrl: URL.createObjectURL(blob),
    status: 'Compressed'
  };
};
