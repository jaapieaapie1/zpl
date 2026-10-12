// Renders labels off the main thread. Messages in: { id, kind: "rgba" | "pdf", zpl, debug, page,
// dpmm } (dpmm: printer resolution in dots/mm, 6, 8, 12 or 24).
// Messages out: { id, kind, ok, ...result } or { id, crashed } after a Rust panic, which
// leaves the wasm instance unusable; the page then starts a fresh worker. "rgba" results
// carry `elements`, laid out as described at Frame.takeElements in zpl-wasm/src/lib.rs, and
// `page` (0-based, clamped to the last page) and `pageCount`.
import init, { renderRgba, renderPdf } from "./pkg/zpl_wasm.js";

const ready = init();

// The printer clock for ^FC fields: the browser's local wall time as Unix seconds.
function localClock() {
  const now = new Date();
  return Math.floor(now.getTime() / 1000) - now.getTimezoneOffset() * 60;
}

self.onmessage = async ({ data: { id, kind, zpl, debug, page, dpmm } }) => {
  await ready;
  const start = performance.now();
  try {
    if (kind === "pdf") {
      const bytes = renderPdf(zpl, localClock(), dpmm);
      self.postMessage({ id, kind, ok: true, bytes, ms: performance.now() - start }, [bytes.buffer]);
    } else {
      const frame = renderRgba(zpl, localClock(), debug, page, dpmm);
      const { width, height, pageCount } = frame;
      const shown = frame.page;
      const pixels = frame.takePixels();
      const elements = frame.takeElements();
      frame.free();
      const ms = performance.now() - start;
      self.postMessage(
        { id, kind, ok: true, width, height, pixels, elements, page: shown, pageCount, ms },
        [pixels.buffer, elements.buffer],
      );
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
