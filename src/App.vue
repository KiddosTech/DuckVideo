<template>
  <div class="page">
    <aside class="navbar navbar-vertical navbar-expand-lg app-sidebar">
      <div class="container-fluid">
        <a class="navbar-brand navbar-brand-autodark" href="./index.html">
          <img class="brand-logo me-2" :src="logoUrl" alt="DuckVideo Studio logo" />
          DuckVideo Studio
        </a>

        <div class="navbar-nav pt-lg-3">
          <a v-for="item in navItems" :key="item.page" class="nav-link" :class="{ active: page === item.page }" :href="item.href">
            <span class="nav-link-icon d-md-none d-lg-inline-block"><component :is="item.icon" size="20" /></span>
            <span class="nav-link-title">{{ item.label }}</span>
          </a>
        </div>

        <div class="platform-list mt-4">
          <div class="text-secondary text-uppercase fw-bold small mb-2">Platform</div>
          <button
            v-for="platform in brandPlatforms"
            :key="platform.id"
            class="btn platform-button justify-content-start"
            type="button"
            :style="{ '--brand': platform.color }"
          >
            <span class="brand-icon" :style="{ color: platform.color }">
              <SimpleBrandIcon :icon="platform.icon" />
            </span>
            {{ platform.name }}
          </button>
        </div>
      </div>
    </aside>

    <div class="page-wrapper">
      <header class="navbar navbar-expand-md d-print-none top-nav">
        <div class="container-xl">
          <div>
            <div class="text-secondary text-uppercase fw-bold small">{{ currentPage.eyebrow }}</div>
            <h1 class="page-title">{{ currentPage.title }}</h1>
          </div>
          <div class="navbar-nav flex-row order-md-last ms-auto">
            <a class="btn btn-outline-primary btn-icon me-2" href="./settings.html" title="Settings">
              <IconSettings size="19" />
            </a>
            <button class="btn btn-primary" type="button" @click="addSampleJob">
              <IconPlayerPlay size="18" class="me-2" />
              Demo job
            </button>
          </div>
        </div>
      </header>

      <main class="page-body">
        <div class="container-xl">
          <div v-if="page === 'home'" class="row row-deck row-cards">
            <MetricCard :icon="IconDownload" color="primary" :value="jobs.length" label="Download jobs" />
            <MetricCard :icon="IconPhoto" color="teal" :value="compressedCount" label="Compressed assets" />
            <MetricCard :icon="IconBrandSpeedtest" color="green" :value="savedBytesLabel" label="Saved size" />
            <div class="col-12">
              <section class="card hero-card">
                <div class="card-body">
                  <span class="badge bg-primary-lt mb-3">Static hosting ready</span>
                  <h2 class="hero-title">One interface for static web hosting and a Tauri desktop app.</h2>
                  <p class="text-secondary hero-copy">
                    Use Download to prepare media jobs, Compress to shrink local image files, and Settings to connect
                    an API adapter when platform media requires a backend resolver.
                  </p>
                  <div class="btn-list">
                    <a class="btn btn-primary" href="./download.html">Start downloading</a>
                    <a class="btn btn-outline-primary" href="./compress.html">Compress file</a>
                  </div>
                </div>
              </section>
            </div>
            <div class="col-12"><PlatformPanel :platforms="platforms" /></div>
            <div class="col-12"><DesktopPanel /></div>
          </div>

          <div v-else-if="page === 'download'" class="row row-cards">
            <div class="col-lg-8">
              <DownloadPlanner />
            </div>
            <div class="col-lg-4">
              <PlatformPanel :platforms="platforms" />
            </div>
            <div class="col-12">
              <DesktopPanel />
            </div>
            <div class="col-12">
              <QueueTable />
            </div>
          </div>

          <div v-else-if="page === 'compress'" class="row row-cards">
            <div class="col-lg-7">
              <OptimizerPanel />
            </div>
            <div class="col-lg-5">
              <SettingsCard />
            </div>
            <div class="col-12">
              <DesktopCompressPanel />
            </div>
            <div class="col-12">
              <QueueTable />
            </div>
          </div>

          <div v-else-if="page === 'settings'" class="row row-cards">
            <div class="col-lg-6"><SettingsCard /></div>
            <div class="col-lg-6"><ApiCard /></div>
            <div class="col-12"><DesktopPanel /></div>
          </div>

          <div v-else class="row row-cards">
            <div class="col-lg-7">
              <section class="card">
                <div class="card-header"><h3 class="card-title">About DuckVideo</h3></div>
                <div class="card-body">
                  <p class="text-secondary">
                    DuckVideo is built as a static web app with Vue, TypeScript, and Tabler, and it can also be bundled
                    as a desktop application with Tauri.
                  </p>
                  <div class="row g-3 mb-3">
                    <div class="col-sm-6">
                      <div class="border rounded p-3 h-100">
                        <div class="text-secondary small text-uppercase fw-bold mb-1">License</div>
                        <div class="h3 mb-1">GPL-3.0-only</div>
                        <div class="text-secondary">GNU General Public License version 3.</div>
                      </div>
                    </div>
                    <div class="col-sm-6">
                      <div class="border rounded p-3 h-100">
                        <div class="text-secondary small text-uppercase fw-bold mb-1">Copyright</div>
                        <div class="h3 mb-1">2026</div>
                        <div class="text-secondary">Ahmad Ilham Kurniawan</div>
                      </div>
                    </div>
                  </div>
                  <div class="alert alert-warning mb-0">
                    Downloads from platforms such as YouTube or TikTok require official APIs or a backend adapter that
                    follows each platform's rules. Static hosting cannot bypass CORS, login walls, or DRM.
                  </div>
                </div>
              </section>
            </div>
            <div class="col-lg-5">
              <section class="card">
                <div class="card-header"><h3 class="card-title">Build targets</h3></div>
                <div class="list-group list-group-flush">
                  <div class="list-group-item"><IconShieldCheck size="20" class="me-2 text-green" />Static hosting: folder dist</div>
                  <div class="list-group-item"><IconBrandGithub size="20" class="me-2 text-primary" />GitHub Actions: installer Tauri</div>
                  <div class="list-group-item"><IconVideo size="20" class="me-2 text-azure" />Desktop app: src-tauri</div>
                </div>
              </section>
            </div>
          </div>
        </div>
      </main>
      <footer class="footer footer-transparent d-print-none">
        <div class="container-xl text-secondary">
          DuckVideo Studio is licensed under GPL-3.0-only. Copyright © 2026 Ahmad Ilham Kurniawan.
        </div>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, defineComponent, onMounted, ref } from 'vue';
import {
  IconBrandGithub,
  IconBrandSpeedtest,
  IconCloudUpload,
  IconDownload,
  IconFileZip,
  IconHome,
  IconInfoCircle,
  IconLink,
  IconPhoto,
  IconPhotoVideo,
  IconPlayerPlay,
  IconSettings,
  IconShieldCheck,
  IconTrash,
  IconVideo,
  IconWorldDownload
} from '@tabler/icons-vue';
import { siDailymotion, siFacebook, siInstagram, siTiktok, siVimeo, siYoutube } from 'simple-icons';
import type { Component, PropType } from 'vue';
import type { SimpleIcon } from 'simple-icons';
import SimpleBrandIcon from './components/SimpleBrandIcon.vue';
import logoUrl from '../duckvideo.svg';
import { checkDesktopTools, compressDesktopMedia, downloadDesktopMedia, isDesktopRuntime } from './desktop';
import type { ProcessResult, ToolStatus } from './desktop';
import { compressImage, createDownloadJob, fileToAsset, formatBytes } from './mediaTools';
import type { DownloadJob, LocalAsset, Platform } from './types';

type PageId = 'home' | 'download' | 'compress' | 'about' | 'settings';
type BrandPlatform = Omit<Platform, 'icon'> & { icon: SimpleIcon };

const page = ((document.body.dataset.page as PageId | undefined) ?? 'home');

const navItems: Array<{ page: PageId; label: string; href: string; icon: Component }> = [
  { page: 'home', label: 'Home', href: './index.html', icon: IconHome },
  { page: 'download', label: 'Downloader', href: './download.html', icon: IconWorldDownload },
  { page: 'compress', label: 'Optimizer', href: './compress.html', icon: IconFileZip },
  { page: 'about', label: 'About', href: './about.html', icon: IconInfoCircle },
  { page: 'settings', label: 'Settings', href: './settings.html', icon: IconSettings }
];

const pageMeta: Record<PageId, { eyebrow: string; title: string }> = {
  home: { eyebrow: 'Static and desktop media suite', title: 'DuckVideo Studio' },
  download: { eyebrow: 'Download planner', title: 'Install video, images, and audio' },
  compress: { eyebrow: 'Local optimizer', title: 'Reduce file size without changing the content' },
  about: { eyebrow: 'Architecture', title: 'Static hosting plus Tauri app' },
  settings: { eyebrow: 'Output policy', title: 'Settings and API adapter' }
};

const directPlatform: Platform = {
  id: 'direct',
  name: 'Direct file',
  domains: ['mp4', 'webm', 'jpg', 'jpeg', 'png', 'webp'],
  color: '#206bc4',
  icon: IconWorldDownload,
  media: ['video', 'image', 'audio']
};

const brandPlatforms: BrandPlatform[] = [
  { id: 'youtube', name: 'YouTube', domains: ['youtube.com', 'youtu.be'], color: '#ff0033', icon: siYoutube, media: ['video', 'audio'] },
  { id: 'tiktok', name: 'TikTok', domains: ['tiktok.com'], color: '#111827', icon: siTiktok, media: ['video'] },
  { id: 'instagram', name: 'Instagram', domains: ['instagram.com'], color: '#e4405f', icon: siInstagram, media: ['video', 'image'] },
  { id: 'facebook', name: 'Facebook', domains: ['facebook.com', 'fb.watch'], color: '#1877f2', icon: siFacebook, media: ['video', 'image'] },
  { id: 'vimeo', name: 'Vimeo', domains: ['vimeo.com'], color: '#1ab7ea', icon: siVimeo, media: ['video'] },
  { id: 'dailymotion', name: 'Dailymotion', domains: ['dailymotion.com', 'dai.ly'], color: '#0a7cff', icon: siDailymotion, media: ['video'] }
];

const platforms: Platform[] = [directPlatform, ...brandPlatforms];
const currentPage = computed(() => pageMeta[page] ?? pageMeta.home);
const mediaUrl = ref('');
const selectedFormat = ref('MP4');
const selectedQuality = ref('720p');
const apiEndpoint = ref('/api/media-resolver');
const imageQuality = ref(78);
const maxWidth = ref(1600);
const keepMetadata = ref(true);
const avoidUpscale = ref(true);
const autoName = ref(true);
const jobs = ref<DownloadJob[]>([]);
const assets = ref<LocalAsset[]>([]);
const desktopStatus = ref<ToolStatus | null>(null);
const desktopMessage = ref('Desktop tools have not been checked yet.');
const desktopOutputDir = ref('');
const desktopInputPath = ref('');
const desktopBusy = ref(false);
const desktopLastResult = ref<ProcessResult | null>(null);
const desktopAvailable = isDesktopRuntime();

const imageAssets = computed(() => assets.value.filter((asset) => asset.type === 'image'));
const compressedCount = computed(() => assets.value.filter((asset) => asset.status === 'Compressed').length);
const savedBytesLabel = computed(() => {
  const total = assets.value.reduce((sum, asset) => {
    if (!asset.outputSize) return sum;
    return sum + Math.max(0, asset.originalSize - asset.outputSize);
  }, 0);
  return formatBytes(total);
});

const queueDownload = () => {
  jobs.value.unshift(createDownloadJob(mediaUrl.value, platforms, selectedFormat.value, selectedQuality.value));
  mediaUrl.value = '';
};

const addSampleJob = () => {
  mediaUrl.value = 'https://youtu.be/sample';
  queueDownload();
};

const handleFiles = (event: Event) => {
  const input = event.target as HTMLInputElement;
  const files = Array.from(input.files ?? []);
  assets.value.unshift(...files.map(fileToAsset));
  input.value = '';
};

const compressImages = async () => {
  const updated = await Promise.all(
    assets.value.map(async (asset) => {
      if (asset.type !== 'image') return asset;
      return compressImage(asset, imageQuality.value / 100, maxWidth.value);
    })
  );
  assets.value = updated;
};

const clearQueue = () => {
  assets.value.forEach((asset) => {
    if (asset.outputUrl) URL.revokeObjectURL(asset.outputUrl);
  });
  jobs.value = [];
  assets.value = [];
};

const refreshDesktopTools = async () => {
  if (!desktopAvailable) {
    desktopMessage.value = 'Desktop commands are available only inside the Tauri app.';
    return;
  }
  try {
    desktopStatus.value = await checkDesktopTools();
    desktopOutputDir.value ||= desktopStatus.value.output_dir;
    desktopMessage.value = 'Desktop tools are ready to use.';
  } catch (error) {
    desktopMessage.value = error instanceof Error ? error.message : String(error);
  }
};

const runDesktopDownload = async () => {
  if (!mediaUrl.value) {
    desktopMessage.value = 'Enter a media URL before starting a desktop download.';
    return;
  }
  desktopBusy.value = true;
  desktopMessage.value = 'Running yt-dlp locally...';
  try {
    desktopLastResult.value = await downloadDesktopMedia({
      url: mediaUrl.value,
      format: selectedFormat.value,
      quality: selectedQuality.value,
      outputDir: desktopOutputDir.value
    });
    desktopMessage.value = desktopLastResult.value.success ? 'Download completed.' : 'Download finished with errors.';
  } catch (error) {
    desktopMessage.value = error instanceof Error ? error.message : String(error);
  } finally {
    desktopBusy.value = false;
  }
};

const runDesktopCompress = async () => {
  if (!desktopInputPath.value) {
    desktopMessage.value = 'Enter a local input file path before starting FFmpeg.';
    return;
  }
  desktopBusy.value = true;
  desktopMessage.value = 'Running FFmpeg locally...';
  try {
    desktopLastResult.value = await compressDesktopMedia({
      inputPath: desktopInputPath.value,
      quality: imageQuality.value,
      maxWidth: maxWidth.value,
      outputDir: desktopOutputDir.value
    });
    desktopMessage.value = desktopLastResult.value.success ? 'Compression completed.' : 'Compression finished with errors.';
  } catch (error) {
    desktopMessage.value = error instanceof Error ? error.message : String(error);
  } finally {
    desktopBusy.value = false;
  }
};

onMounted(refreshDesktopTools);

const MetricCard = defineComponent({
  props: {
    icon: { type: Object as PropType<Component>, required: true },
    color: { type: String, required: true },
    value: { type: [String, Number], required: true },
    label: { type: String, required: true }
  },
  template: `
    <div class="col-sm-6 col-lg-4">
      <div class="card">
        <div class="card-body d-flex align-items-center">
          <span :class="'bg-' + color + ' text-white avatar me-3'"><component :is="icon" size="22" /></span>
          <div>
            <div class="h2 mb-0">{{ value }}</div>
            <div class="text-secondary">{{ label }}</div>
          </div>
        </div>
      </div>
    </div>
  `
});

const DownloadPlanner = defineComponent({
  setup() {
    return { mediaUrl, selectedFormat, selectedQuality, apiEndpoint, queueDownload, IconCloudUpload, IconInfoCircle, IconLink };
  },
  template: `
    <section class="card">
      <div class="card-header">
        <div>
          <h3 class="card-title">Download planner</h3>
          <p class="card-subtitle">Create a job from a platform URL or a direct media file.</p>
        </div>
        <span class="badge bg-primary-lt ms-auto">API adapter ready</span>
      </div>
      <div class="card-body">
        <form class="row g-3" @submit.prevent="queueDownload">
          <div class="col-12">
            <label class="form-label">Media URL</label>
            <div class="input-icon">
              <span class="input-icon-addon"><IconLink size="18" /></span>
              <input v-model.trim="mediaUrl" class="form-control" type="url" placeholder="https://..." required />
            </div>
          </div>
          <div class="col-md-4">
            <label class="form-label">Format</label>
            <select v-model="selectedFormat" class="form-select">
              <option>MP4</option><option>WEBM</option><option>MP3</option><option>WEBP</option><option>Original</option>
            </select>
          </div>
          <div class="col-md-4">
            <label class="form-label">Quality</label>
            <select v-model="selectedQuality" class="form-select">
              <option>Original</option><option>1080p</option><option>720p</option><option>480p</option><option>Small</option>
            </select>
          </div>
          <div class="col-md-4 d-flex align-items-end">
            <button class="btn btn-primary w-100" type="submit"><IconCloudUpload size="18" class="me-2" />Add to queue</button>
          </div>
        </form>
        <div class="alert alert-info mt-4 mb-0">
          <IconInfoCircle size="22" class="me-2" />Endpoint adapter: <code>{{ apiEndpoint }}</code>
        </div>
      </div>
    </section>
  `
});

const OptimizerPanel = defineComponent({
  setup() {
    return { imageAssets, imageQuality, maxWidth, handleFiles, compressImages, IconFileZip, IconPhotoVideo };
  },
  template: `
    <section class="card">
      <div class="card-header">
        <div>
          <h3 class="card-title">Local optimizer</h3>
          <p class="card-subtitle">Images are compressed directly in the browser.</p>
        </div>
      </div>
      <div class="card-body">
        <label class="dropzone">
          <IconPhotoVideo size="38" />
          <strong>Choose images or videos</strong>
          <span class="text-secondary">PNG, JPG, WebP, MP4, WebM</span>
          <input type="file" multiple accept="image/*,video/*" @change="handleFiles" />
        </label>
        <div class="row g-3 mt-2">
          <div class="col-md-7">
            <label class="form-label">Image quality: {{ imageQuality }}%</label>
            <input v-model.number="imageQuality" class="form-range" type="range" min="45" max="95" />
          </div>
          <div class="col-md-5">
            <label class="form-label">Max width</label>
            <input v-model.number="maxWidth" class="form-control" type="number" min="320" step="160" />
          </div>
        </div>
        <button class="btn btn-teal w-100 mt-3" type="button" :disabled="!imageAssets.length" @click="compressImages">
          <IconFileZip size="18" class="me-2" />Compress images
        </button>
      </div>
    </section>
  `
});

const PlatformPanel = defineComponent({
  props: { platforms: { type: Array as PropType<Platform[]>, required: true } },
  components: { SimpleBrandIcon },
  template: `
    <section class="card">
      <div class="card-header"><h3 class="card-title">Supported media</h3></div>
      <div class="list-group list-group-flush">
        <div v-for="platform in platforms" :key="platform.id" class="list-group-item">
          <div class="row align-items-center">
            <div class="col-auto">
              <span class="avatar" :style="{ backgroundColor: platform.color + '18', color: platform.color }">
                <component v-if="platform.id === 'direct'" :is="platform.icon" />
                <SimpleBrandIcon v-else :icon="platform.icon" />
              </span>
            </div>
            <div class="col">
              <div class="fw-bold">{{ platform.name }}</div>
              <div class="text-secondary small">{{ platform.media.join(', ') }}</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  `
});

const QueueTable = defineComponent({
  setup() {
    return { jobs, assets, clearQueue, formatBytes, IconTrash };
  },
  template: `
    <section class="card">
      <div class="card-header">
        <h3 class="card-title">Processing queue</h3>
        <button class="btn btn-outline-danger btn-icon ms-auto" type="button" @click="clearQueue"><IconTrash size="18" /></button>
      </div>
      <div class="table-responsive">
        <table class="table table-vcenter card-table">
          <thead><tr><th>Item</th><th>Source</th><th>Status</th><th>Size</th><th>Output</th></tr></thead>
          <tbody>
            <tr v-for="job in jobs" :key="job.id">
              <td class="fw-bold">{{ job.platform.name }}</td>
              <td class="text-secondary text-truncate queue-source">{{ job.url }}</td>
              <td><span class="badge bg-yellow-lt">{{ job.status }}</span></td>
              <td>{{ job.estimatedSize }}</td>
              <td>{{ job.format }} / {{ job.quality }}</td>
            </tr>
            <tr v-for="asset in assets" :key="asset.id">
              <td class="fw-bold">{{ asset.name }}</td>
              <td class="text-secondary">{{ asset.type }}</td>
              <td><span class="badge bg-blue-lt">{{ asset.status }}</span></td>
              <td>{{ formatBytes(asset.originalSize) }}</td>
              <td>
                <a v-if="asset.outputUrl" :href="asset.outputUrl" :download="asset.name.replace(/\\.[^.]+$/, '.webp')">
                  {{ formatBytes(asset.outputSize ?? 0) }}
                </a>
                <span v-else class="text-secondary">Pending</span>
              </td>
            </tr>
            <tr v-if="!jobs.length && !assets.length">
              <td colspan="5" class="text-center text-secondary py-5">The queue is empty.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  `
});

const SettingsCard = defineComponent({
  setup() {
    return { keepMetadata, avoidUpscale, autoName, imageQuality };
  },
  template: `
    <section class="card">
      <div class="card-header"><h3 class="card-title">Output policy</h3></div>
      <div class="card-body">
        <label class="form-check form-switch"><input v-model="keepMetadata" class="form-check-input" type="checkbox" /><span class="form-check-label">Keep important metadata</span></label>
        <label class="form-check form-switch mt-3"><input v-model="avoidUpscale" class="form-check-input" type="checkbox" /><span class="form-check-label">Avoid resolution upscaling</span></label>
        <label class="form-check form-switch mt-3"><input v-model="autoName" class="form-check-input" type="checkbox" /><span class="form-check-label">Generate file names automatically</span></label>
        <div class="progress progress-sm mt-4"><div class="progress-bar bg-teal" :style="{ width: imageQuality + '%' }"></div></div>
      </div>
    </section>
  `
});

const ApiCard = defineComponent({
  setup() {
    return { apiEndpoint };
  },
  template: `
    <section class="card">
      <div class="card-header"><h3 class="card-title">API adapter</h3></div>
      <div class="card-body">
        <label class="form-label">Resolver endpoint</label>
        <input v-model="apiEndpoint" class="form-control" />
        <p class="text-secondary mt-3 mb-0">For static hosting, this endpoint can point to a serverless function or a separate backend.</p>
      </div>
    </section>
  `
});

const DesktopPanel = defineComponent({
  setup() {
    return {
      desktopAvailable,
      desktopBusy,
      desktopMessage,
      desktopOutputDir,
      desktopStatus,
      desktopLastResult,
      refreshDesktopTools,
      runDesktopDownload,
      IconDownload,
      IconBrandSpeedtest
    };
  },
  template: `
    <section class="card">
      <div class="card-header">
        <div>
          <h3 class="card-title">Desktop local engine</h3>
          <p class="card-subtitle">The Tauri app can use bundled yt-dlp and FFmpeg without a server.</p>
        </div>
        <span class="badge" :class="desktopAvailable ? 'bg-green-lt' : 'bg-secondary-lt'">
          {{ desktopAvailable ? 'Tauri runtime' : 'Web preview' }}
        </span>
      </div>
      <div class="card-body">
        <div class="row g-3">
          <div class="col-md-6">
            <div class="border rounded p-3 h-100">
              <div class="text-secondary small text-uppercase fw-bold">yt-dlp</div>
              <div class="h3 mb-1">{{ desktopStatus?.yt_dlp.available ? 'Available' : 'Missing' }}</div>
              <div class="text-secondary text-truncate">{{ desktopStatus?.yt_dlp.version || desktopStatus?.yt_dlp.path || 'Not checked' }}</div>
            </div>
          </div>
          <div class="col-md-6">
            <div class="border rounded p-3 h-100">
              <div class="text-secondary small text-uppercase fw-bold">FFmpeg</div>
              <div class="h3 mb-1">{{ desktopStatus?.ffmpeg.available ? 'Available' : 'Missing' }}</div>
              <div class="text-secondary text-truncate">{{ desktopStatus?.ffmpeg.version || desktopStatus?.ffmpeg.path || 'Not checked' }}</div>
            </div>
          </div>
        </div>
        <label class="form-label mt-3">Output folder</label>
        <input v-model="desktopOutputDir" class="form-control" placeholder="Default: Downloads/DuckVideo Studio" />
        <div class="btn-list mt-3">
          <button class="btn btn-outline-primary" type="button" @click="refreshDesktopTools">
            <IconBrandSpeedtest size="18" class="me-2" />Check tools
          </button>
          <button class="btn btn-primary" type="button" :disabled="desktopBusy || !desktopAvailable" @click="runDesktopDownload">
            <IconDownload size="18" class="me-2" />Download with yt-dlp
          </button>
        </div>
        <div class="alert mt-3 mb-0" :class="desktopAvailable ? 'alert-info' : 'alert-warning'">
          {{ desktopMessage }}
        </div>
        <pre v-if="desktopLastResult" class="desktop-log mt-3">{{ desktopLastResult.stderr || desktopLastResult.stdout }}</pre>
      </div>
    </section>
  `
});

const DesktopCompressPanel = defineComponent({
  setup() {
    return {
      desktopAvailable,
      desktopBusy,
      desktopInputPath,
      desktopOutputDir,
      desktopMessage,
      desktopLastResult,
      runDesktopCompress,
      IconFileZip
    };
  },
  template: `
    <section class="card">
      <div class="card-header">
        <div>
          <h3 class="card-title">Desktop FFmpeg compressor</h3>
          <p class="card-subtitle">Use a local file path when running the Tauri app.</p>
        </div>
      </div>
      <div class="card-body">
        <div class="row g-3">
          <div class="col-lg-7">
            <label class="form-label">Input file path</label>
            <input v-model="desktopInputPath" class="form-control" placeholder="C:\\Videos\\input.mp4" />
          </div>
          <div class="col-lg-5">
            <label class="form-label">Output folder</label>
            <input v-model="desktopOutputDir" class="form-control" />
          </div>
        </div>
        <button class="btn btn-teal mt-3" type="button" :disabled="desktopBusy || !desktopAvailable" @click="runDesktopCompress">
          <IconFileZip size="18" class="me-2" />Compress with FFmpeg
        </button>
        <div class="alert alert-info mt-3 mb-0">{{ desktopMessage }}</div>
        <pre v-if="desktopLastResult" class="desktop-log mt-3">{{ desktopLastResult.stderr || desktopLastResult.stdout }}</pre>
      </div>
    </section>
  `
});
</script>
