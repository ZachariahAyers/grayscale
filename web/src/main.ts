import '@fontsource/pacifico/latin-400.css';
import './styles.css';
import init, { grayscale } from '../../pkg/grayscale.js';

function byId<T extends HTMLElement>(id: string, type: abstract new () => T): T {
  const element = document.getElementById(id);
  if (!(element instanceof type)) {
    throw new Error(`#${id} is missing from the page`);
  }
  return element;
}

const upload = byId('upload', HTMLInputElement);
const status = byId('status', HTMLParagraphElement);
const result = byId('result', HTMLImageElement);

let resultUrl: string | undefined;
let latestRequest = 0;

function report(message: string, isError = false): void {
  status.textContent = message;
  status.classList.toggle('error', isError);
}

/** Decodes an image file, grayscales its pixels in Rust, and re-encodes it as PNG. */
async function toGrayscalePng(file: Blob): Promise<Blob> {
  const bitmap = await createImageBitmap(file);
  const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
  const context = canvas.getContext('2d');
  if (!context) {
    throw new Error('2D canvas is not available');
  }
  context.drawImage(bitmap, 0, 0);
  bitmap.close();

  const image = context.getImageData(0, 0, canvas.width, canvas.height);
  const { data } = image;
  data.set(grayscale(new Uint8Array(data.buffer, data.byteOffset, data.byteLength)));
  context.putImageData(image, 0, 0);
  return canvas.convertToBlob({ type: 'image/png' });
}

async function convert(file: File): Promise<void> {
  const request = ++latestRequest;
  report('Converting...');
  try {
    const png = await toGrayscalePng(file);
    if (request !== latestRequest) {
      return;
    }
    if (resultUrl) {
      URL.revokeObjectURL(resultUrl);
    }
    resultUrl = URL.createObjectURL(png);
    result.src = resultUrl;
    result.alt = `${file.name} in grayscale`;
    result.hidden = false;
    report('');
  } catch (error) {
    if (request === latestRequest) {
      console.error(error);
      report('That file could not be read as an image.', true);
    }
  }
}

upload.addEventListener('change', () => {
  const file = upload.files?.[0];
  if (file) {
    void convert(file);
  }
});

init().then(
  () => {
    upload.disabled = false;
  },
  (error: unknown) => {
    console.error(error);
    report('The image engine failed to load. Reload the page to try again.', true);
  },
);
