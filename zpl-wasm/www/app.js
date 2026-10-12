// ZPL playground: re-renders the label in a worker on every edit. Only the newest edit
// is rendered; edits made while a render runs collapse into one follow-up render.

const SAMPLE = `^XA
^CI28
^PW812
^LL1218

^FX Header
^CF0,60
^FO50,50^GB100,100,100^FS
^FO75,75^FR^GB100,100,100^FS
^FO93,93^GB40,40,40^FS
^FO220,50^FDIntershipping, Inc.^FS
^CF0,30
^FO220,115^FD1000 Shipping Lane^FS
^FO220,155^FDShelbyville TN 38102^FS
^FO50,250^GB700,3,3^FS

^FX Recipient
^CFA,30
^FO50,300^FDJohn Doe^FS
^FO50,340^FD100 Main Street^FS
^FO50,380^FDSpringfield TN 39021^FS
^CFA,15
^FO600,300^GB150,150,3^FS
^FO638,340^FDPermit^FS
^FO638,390^FD123456^FS
^FO50,500^GB700,3,3^FS

^FX Barcode
^BY5,2,270
^FO100,550^BC^FD12345678^FS

^FX QR code and reference
^FO50,900^GB700,250,3^FS
^FO400,900^GB3,250,3^FS
^CF0,40
^FO100,960^FDCtr. X34B-1^FS
^FO100,1010^FDREF1 F00B47^FS
^FO100,1060^FDREF2 BL4H8^FS
^FO470,930^BQN,2,6^FDQA,https://example.com/track/12345678^FS
^XZ`;

const STORAGE_KEY = "zpl-playground:source";

const editor = document.getElementById("zpl");
const canvas = document.getElementById("label");
const ctx = canvas.getContext("2d");
const pane = document.getElementById("preview-pane");
const problem = document.getElementById("problem");
const status = document.getElementById("status");
const size = document.getElementById("size");
const cursor = document.getElementById("cursor");
const debug = document.getElementById("debug");
const zoom = document.getElementById("zoom");

function load(key, fallback) {
  try { return localStorage.getItem(key) ?? fallback; } catch { return fallback; }
}
function save(key, value) {
  try { localStorage.setItem(key, value); } catch { /* storage unavailable */ }
}

editor.value = load(STORAGE_KEY, SAMPLE);

// --- Worker lifecycle -------------------------------------------------------------

let worker;
let nextId = 1;
let inflight = null; // id of the preview render running in the worker
let dirty = false; // an edit arrived while `inflight` was running
const pdfRequests = new Map(); // id -> resolve

function startWorker() {
  worker = new Worker(new URL("worker.js", import.meta.url), { type: "module" });
  worker.onmessage = ({ data }) => onResult(data);
  worker.onerror = (e) => {
    e.preventDefault();
    restartWorker(e.message || "worker failed to start");
  };
}

// A Rust panic poisons the wasm instance, so throw the worker away and start over.
function restartWorker(reason) {
  worker.terminate();
  inflight = null;
  for (const resolve of pdfRequests.values()) resolve({ ok: false, message: `Renderer crashed: ${reason}` });
  pdfRequests.clear();
  showProblem({ message: `Renderer crashed: ${reason}` });
  status.textContent = "Renderer crashed — restarted";
  pane.classList.remove("stale");
  startWorker();
}

function requestPreview() {
  if (inflight !== null) {
    dirty = true;
    return;
  }
  dirty = false;
  inflight = nextId++;
  pane.classList.add("stale");
  worker.postMessage({ id: inflight, kind: "rgba", zpl: editor.value, debug: debug.checked });
}

function requestPdf() {
  const id = nextId++;
  return new Promise((resolve) => {
    pdfRequests.set(id, resolve);
    worker.postMessage({ id, kind: "pdf", zpl: editor.value });
  });
}

function onResult(data) {
  if (data.kind === "pdf") {
    const resolve = pdfRequests.get(data.id);
    pdfRequests.delete(data.id);
    if (data.crashed) return restartWorker(data.crashed);
    return resolve?.(data);
  }
  if (data.id !== inflight) return;
  inflight = null;
  if (data.crashed) {
    restartWorker(data.crashed);
  } else if (data.ok) {
    draw(data);
    hideProblem();
    status.textContent = `Rendered in ${data.ms.toFixed(1)} ms`;
  } else {
    showProblem(data);
    status.textContent = "Parse error";
  }
  pane.classList.toggle("stale", dirty || !data.ok);
  if (dirty) requestPreview();
}

// --- Drawing ----------------------------------------------------------------------

function draw({ width, height, pixels }) {
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
    applyZoom();
  }
  ctx.putImageData(new ImageData(new Uint8ClampedArray(pixels.buffer), width, height), 0, 0);
  size.textContent = `${width} × ${height} dots`;
}

function applyZoom() {
  const value = zoom.value;
  const fit = value === "fit";
  pane.classList.toggle("fit", fit);
  pane.classList.toggle("pixelated", !fit && Number(value) > 1);
  canvas.style.width = fit ? "" : `${canvas.width * Number(value)}px`;
  canvas.style.height = fit ? "" : `${canvas.height * Number(value)}px`;
}

// --- Problems ---------------------------------------------------------------------

let problemRange = null;

function showProblem({ message, start, end }) {
  problemRange = start === undefined ? null : [start, Math.max(end, start + 1)];
  const where = problemRange ? ` (line ${lineOf(start)})` : "";
  problem.textContent = message + where;
  problem.classList.add("visible");
}

function hideProblem() {
  problemRange = null;
  problem.classList.remove("visible");
}

function lineOf(offset) {
  return editor.value.slice(0, offset).split("\n").length;
}

problem.addEventListener("click", () => {
  if (!problemRange) return;
  editor.focus();
  editor.setSelectionRange(...problemRange);
});

// --- Events -----------------------------------------------------------------------

let saveTimer;
editor.addEventListener("input", () => {
  requestPreview();
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => save(STORAGE_KEY, editor.value), 300);
});

function updateCursor() {
  const before = editor.value.slice(0, editor.selectionStart);
  const line = before.split("\n").length;
  const col = before.length - before.lastIndexOf("\n");
  cursor.textContent = `Ln ${line}, Col ${col}`;
}
editor.addEventListener("keyup", updateCursor);
editor.addEventListener("click", updateCursor);
editor.addEventListener("select", updateCursor);

// Tab inserts a tab instead of leaving the editor; Escape then Tab still moves focus.
let escaped = false;
editor.addEventListener("keydown", (e) => {
  if (e.key === "Escape") { escaped = true; return; }
  if (e.key === "Tab" && !escaped && !e.shiftKey) {
    e.preventDefault();
    editor.setRangeText("\t", editor.selectionStart, editor.selectionEnd, "end");
    editor.dispatchEvent(new Event("input"));
  }
  escaped = false;
});

debug.addEventListener("change", requestPreview);
zoom.addEventListener("change", applyZoom);

function download(blob, name) {
  const url = URL.createObjectURL(blob);
  const a = Object.assign(document.createElement("a"), { href: url, download: name });
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

document.getElementById("png").addEventListener("click", () => {
  canvas.toBlob((blob) => blob && download(blob, "label.png"), "image/png");
});

document.getElementById("pdf").addEventListener("click", async () => {
  const result = await requestPdf();
  if (result.ok) download(new Blob([result.bytes], { type: "application/pdf" }), "label.pdf");
  else showProblem(result);
});

startWorker();
applyZoom();
updateCursor();
requestPreview();
