// Renders labels off the main thread. Messages in: { id, kind: "rgba" | "pdf", zpl, debug }.
// Messages out: { id, kind, ok, ...result } or { id, crashed } after a Rust panic, which
// leaves the wasm instance unusable; the page then starts a fresh worker.
import init, { renderRgba, renderPdf } from "./pkg/zpl_wasm.js";

const ready = init();

// The printer clock for ^FC fields: the browser's local wall time as Unix seconds.
function localClock() {
  const now = new Date();
  return Math.floor(now.getTime() / 1000) - now.getTimezoneOffset() * 60;
}

self.onmessage = async ({ data: { id, kind, zpl, debug } }) => {
  await ready;
  const start = performance.now();
  try {
    if (kind === "pdf") {
      const bytes = renderPdf(zpl, localClock());
      self.postMessage({ id, kind, ok: true, bytes, ms: performance.now() - start }, [bytes.buffer]);
    } else {
      const frame = renderRgba(zpl, localClock(), debug);
      const { width, height } = frame;
      const pixels = frame.takePixels();
      frame.free();
      self.postMessage({ id, kind, ok: true, width, height, pixels, ms: performance.now() - start }, [pixels.buffer]);
    }
  } catch (e) {
    if (e && typeof e.message === "string" && "start" in e) {
      const { message, start: from, end: to } = e;
      e.free();
      self.postMessage({ id, kind, ok: false, message, start: from, end: to });
    } else {
      self.postMessage({ id, kind, crashed: String(e) });
    }
  }
};
