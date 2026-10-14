// ZPL playground: re-renders the label in a worker on every edit. Only the newest edit
// is rendered; edits made while a render runs collapse into one follow-up render.
//
// Clicking an element on the label selects its source in the editor, and moving the
// editor cursor into a field outlines it on the label. Dragging a field placed by ^FO/^FT
// moves it: the preview follows the drag and the drop rewrites the command's coordinates. Input with several labels (or ^PQ
// copies) shows one page at a time, picked in the header.

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
const DPMM_KEY = "zpl-playground:dpmm";
const STAGE_PADDING = 24; // #preview-pane padding, px

const editor = document.getElementById("zpl");
const canvas = document.getElementById("label");
const ctx = canvas.getContext("2d");
const pane = document.getElementById("preview-pane");
const hoverBox = document.getElementById("hover");
const selectedBox = document.getElementById("selected");
const problem = document.getElementById("problem");
const status = document.getElementById("status");
const size = document.getElementById("size");
const cursor = document.getElementById("cursor");
const debug = document.getElementById("debug");
const dpmm = document.getElementById("dpmm");
const zoom = document.getElementById("zoom");
const pager = document.getElementById("pager");
const pageLabel = document.getElementById("page-label");
const prevPage = document.getElementById("prev-page");
const nextPage = document.getElementById("next-page");

function load(key, fallback) {
  try { return localStorage.getItem(key) ?? fallback; } catch { return fallback; }
}
function save(key, value) {
  try { localStorage.setItem(key, value); } catch { /* storage unavailable */ }
}

editor.value = load(STORAGE_KEY, SAMPLE);
// Printer resolution in dots/mm; an unknown stored value leaves the default (8) selected.
dpmm.value = load(DPMM_KEY, dpmm.value);
if (!dpmm.value) dpmm.value = "8";

// --- Worker lifecycle -------------------------------------------------------------

let worker;
let nextId = 1;
let inflight = null; // id of the preview render running in the worker
let dirty = false; // an edit arrived while `inflight` was running
const requests = new Map(); // id -> resolve, for "pdf" and "units" requests
let draft = null; // the source with a dragged field moved; previewed instead of the editor's

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
  for (const resolve of requests.values()) resolve({ ok: false, message: `Renderer crashed: ${reason}` });
  requests.clear();
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
  const zpl = draft ?? editor.value;
  worker.postMessage({ id: inflight, kind: "rgba", zpl, debug: debug.checked, page, dpmm: Number(dpmm.value) });
}

function request(kind, message) {
  const id = nextId++;
  return new Promise((resolve) => {
    requests.set(id, resolve);
    worker.postMessage({ id, kind, zpl: editor.value, dpmm: Number(dpmm.value), ...message });
  });
}

function onResult(data) {
  if (data.kind !== "rgba") {
    const resolve = requests.get(data.id);
    requests.delete(data.id);
    if (data.crashed) return restartWorker(data.crashed);
    return resolve?.(data);
  }
  if (data.id !== inflight) return;
  inflight = null;
  if (data.crashed) {
    restartWorker(data.crashed);
  } else if (data.ok) {
    setPageCount(data.pageCount);
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

// --- Pages ------------------------------------------------------------------------

let page = 0; // 0-based page to show
let pageCount = 1;

// Keeps the page in range when an edit drops labels. A newer page picked while the render
// ran is kept; it is rendered next.
function setPageCount(count) {
  pageCount = count;
  page = Math.min(page, count - 1);
  pager.hidden = count < 2;
  pageLabel.textContent = `${page + 1} / ${count}`;
  prevPage.disabled = page === 0;
  nextPage.disabled = page >= count - 1;
}

function showPage(next) {
  if (next < 0 || next >= pageCount || next === page) return;
  page = next;
  setPageCount(pageCount);
  requestPreview();
}

prevPage.addEventListener("click", () => showPage(page - 1));
nextPage.addEventListener("click", () => showPage(page + 1));

// --- Drawing ----------------------------------------------------------------------

let scale = 1; // CSS pixels per dot

function draw({ width, height, pixels, elements }) {
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
  }
  ctx.putImageData(new ImageData(new Uint8ClampedArray(pixels.buffer), width, height), 0, 0);
  size.textContent = `${width} × ${height} dots`;
  setElements(elements);
  applyZoom();
}

function applyZoom() {
  const value = zoom.value;
  if (value === "fit") {
    const availableWidth = pane.clientWidth - 2 * STAGE_PADDING;
    const availableHeight = pane.clientHeight - 2 * STAGE_PADDING;
    scale = Math.max(0.05, Math.min(availableWidth / canvas.width, availableHeight / canvas.height));
  } else {
    scale = Number(value);
  }
  pane.classList.toggle("pixelated", scale > 1);
  canvas.style.width = `${canvas.width * scale}px`;
  canvas.style.height = `${canvas.height * scale}px`;
  placeOutline(hoverBox, hoverField);
  // The draft's elements have moved spans; a dragged field keeps the area it was grabbed by.
  if (drag?.moved) placeArea(selectedBox, drag.area, [drag.dx, drag.dy]);
  else placeOutline(selectedBox, selectedField);
}

new ResizeObserver(() => { if (zoom.value === "fit") applyZoom(); }).observe(pane);

// --- Elements and selection -------------------------------------------------------

// One entry per drawn element; see Frame.takeElements in zpl-wasm/src/lib.rs.
const STRIDE = 8;
const NONE = 0xffffffff;
let elements = [];

function setElements(flat) {
  elements = [];
  for (let i = 0; i < flat.length; i += STRIDE) {
    elements.push({
      x: flat[i], y: flat[i + 1], width: flat[i + 2], height: flat[i + 3],
      field: [flat[i + 4], flat[i + 5]],
      data: flat[i + 6] === NONE ? null : [flat[i + 6], flat[i + 7]],
    });
  }
  // Source offsets moved if the edit was before the selection; re-find it from the editor.
  // While dragging, the elements belong to the draft, not the editor.
  if (selectedField && !drag) selectFromEditor(selectedFrom === "click");
}

// A field is identified by its source span; every element it drew shares it.
let hoverField = null;
let selectedField = null;
let selectedFrom = null; // "click" | "editor"

function sameField(a, b) {
  return a && b && a[0] === b[0] && a[1] === b[1];
}

// Union of the areas drawn by `field`, in dots.
function fieldArea(field) {
  let area = null;
  for (const e of elements) {
    if (!sameField(e.field, field)) continue;
    const right = e.x + e.width;
    const bottom = e.y + e.height;
    area = area
      ? { x: Math.min(area.x, e.x), y: Math.min(area.y, e.y), right: Math.max(area.right, right), bottom: Math.max(area.bottom, bottom) }
      : { x: e.x, y: e.y, right, bottom };
  }
  return area;
}

function placeOutline(box, field) {
  placeArea(box, field && fieldArea(field));
}

// `offset` shifts the area by [dx, dy] dots, for a field being dragged.
function placeArea(box, area, [dx, dy] = [0, 0]) {
  box.classList.toggle("visible", Boolean(area));
  if (!area) return;
  box.style.left = `${(area.x + dx) * scale}px`;
  box.style.top = `${(area.y + dy) * scale}px`;
  box.style.width = `${(area.right - area.x) * scale}px`;
  box.style.height = `${(area.bottom - area.y) * scale}px`;
}

// The element under a point in dots: the smallest one containing it, so text inside a
// frame wins over the frame; among equals, the one drawn last (on top).
function elementAt(x, y) {
  const slop = 4 / scale; // a few screen pixels, so thin lines and small text are clickable
  let best = null;
  for (const e of elements) {
    if (x < e.x - slop || y < e.y - slop || x >= e.x + e.width + slop || y >= e.y + e.height + slop) continue;
    if (!best || e.width * e.height <= best.width * best.height) best = e;
  }
  return best;
}

function pointerDots(event) {
  const rect = canvas.getBoundingClientRect();
  return [(event.clientX - rect.left) / scale, (event.clientY - rect.top) / scale];
}

canvas.addEventListener("mousemove", (event) => {
  if (drag) return;
  const element = elementAt(...pointerDots(event));
  hoverField = element?.field ?? null;
  canvas.style.cursor = !element ? "default" : originCommand(element.field) ? "move" : "pointer";
  placeOutline(hoverBox, sameField(hoverField, selectedField) ? null : hoverField);
});

canvas.addEventListener("mouseleave", () => {
  hoverField = null;
  placeOutline(hoverBox, null);
});

canvas.addEventListener("click", (event) => {
  if (dragged) {
    dragged = false;
    return;
  }
  const element = elementAt(...pointerDots(event));
  if (!element) {
    select(null, null);
    return;
  }
  select(element.field, "click");
  revealInEditor(element.data ?? element.field);
});

function select(field, from) {
  selectedField = field;
  selectedFrom = field ? from : null;
  selectedBox.classList.toggle("from-editor", from === "editor");
  placeOutline(selectedBox, field);
  placeOutline(hoverBox, sameField(hoverField, field) ? null : hoverField);
}

// Selects `range` in the editor and scrolls it into view.
function revealInEditor([start, end]) {
  editor.focus({ preventScroll: true });
  editor.setSelectionRange(start, end);
  const line = editor.value.slice(0, start).split("\n").length - 1;
  const lineHeight = parseFloat(getComputedStyle(editor).lineHeight) || 20;
  const top = line * lineHeight;
  if (top < editor.scrollTop || top > editor.scrollTop + editor.clientHeight - 2 * lineHeight) {
    editor.scrollTop = Math.max(0, top - editor.clientHeight / 3);
  }
  updateCursor();
}

// Outlines the field holding the editor cursor. `keepClick` keeps the solid "clicked"
// style while the cursor stays inside the clicked field.
function selectFromEditor(keepClick = false) {
  const at = editor.selectionStart;
  let field = null;
  for (const e of elements) {
    if (at < e.field[0] || at > e.field[1]) continue;
    // Innermost field when spans nest (a recalled format maps to its ^XZ).
    if (!field || e.field[1] - e.field[0] < field[1] - field[0]) field = e.field;
  }
  const clicked = keepClick || (selectedFrom === "click" && sameField(field, selectedField));
  select(field, clicked ? "click" : "editor");
}

// --- Dragging fields --------------------------------------------------------------

let drag = null; // { field, area, origin: [start, end], text, command, units, x, y, dx, dy, moved }
let dragged = false; // the last pointer press moved a field; swallows its click

// The ^FO/^FT command that opens `field`, as [start, end] in the source, or null when the
// field has none (it then sits where the previous one did, or comes from a recalled format).
function originCommand([start, end]) {
  const source = editor.value;
  if (!/^F[OT]$/i.test(source.slice(start + 1, start + 3))) return null;
  // Parameters run to the next command, which starts with the same prefix (^, or a ^CC one).
  const next = source.indexOf(source[start], start + 3);
  return [start, next === -1 || next > end ? end : next];
}

// `text` (a ^FO/^FT command) with its x and y moved by [dx, dy] printer dots. `units` is
// fieldUnits' answer: printer dots and format dots per parameter unit.
function moveCommand(text, [dx, dy], { dots, format }) {
  const [, head, x = "", y = "", rest = ""] = /^(.{3})([^,]*)(?:,([^,]*))?(.*)$/s.exec(text);
  const shift = (param, delta) => {
    const [, number, tail] = /^(\S*)(.*)$/s.exec(param);
    const value = Math.max(0, (Number(number) || 0) + delta / dots);
    return (format === 1 ? Math.round(value) : Math.round(value * 1000) / 1000) + tail;
  };
  return `${head}${shift(x, dx)},${shift(y, dy)}${rest}`;
}

function cancelDrag() {
  if (!drag) return;
  dragged = drag.moved;
  drag = null;
  draft = null;
  pane.classList.remove("dragging");
  placeOutline(selectedBox, selectedField);
  requestPreview();
}

function updateDraft() {
  if (!drag.units) return; // applied once fieldUnits answers
  const command = moveCommand(drag.text, [drag.dx, drag.dy], drag.units);
  const [start, end] = drag.origin;
  draft = editor.value.slice(0, start) + command + editor.value.slice(end);
  drag.command = command;
  status.textContent = command.trim();
  requestPreview();
}

canvas.addEventListener("pointerdown", (event) => {
  dragged = false;
  if (event.button !== 0) return;
  const element = elementAt(...pointerDots(event));
  const origin = element && originCommand(element.field);
  if (!origin) return;
  event.preventDefault();
  canvas.setPointerCapture(event.pointerId);
  const text = editor.value.slice(...origin);
  const area = fieldArea(element.field);
  drag = { field: element.field, area, origin, text, command: null, units: null, x: event.clientX, y: event.clientY, dx: 0, dy: 0, moved: false };
  const current = drag;
  request("units", { at: origin[0] }).then((units) => {
    if (drag !== current) return;
    drag.units = units.ok ? units : { dots: 1, format: 1 };
    if (drag.moved) updateDraft();
  });
});

canvas.addEventListener("pointermove", (event) => {
  if (!drag) return;
  const moveX = event.clientX - drag.x;
  const moveY = event.clientY - drag.y;
  if (!drag.moved && Math.hypot(moveX, moveY) < 4) return;
  if (!drag.moved) {
    drag.moved = true;
    pane.classList.add("dragging");
    select(drag.field, "click");
    placeOutline(hoverBox, null);
  }
  const dx = Math.round(moveX / scale);
  const dy = Math.round(moveY / scale);
  if (dx === drag.dx && dy === drag.dy) return;
  Object.assign(drag, { dx, dy });
  placeArea(selectedBox, drag.area, [dx, dy]);
  updateDraft();
});

canvas.addEventListener("pointerup", () => {
  if (!drag) return;
  const { field, origin: [start, end], command, moved } = drag;
  drag = null;
  draft = null;
  pane.classList.remove("dragging");
  if (!moved) return; // a click: the click handler selects the field
  dragged = true;
  if (!command) return requestPreview(); // dropped before fieldUnits answered
  // One undoable edit for the whole drag.
  editor.focus({ preventScroll: true });
  editor.setSelectionRange(start, end);
  if (!document.execCommand("insertText", false, command)) {
    editor.setRangeText(command, start, end);
    editor.dispatchEvent(new Event("input"));
  }
  // Still the clicked field, at its new span.
  select([field[0], field[1] + command.length - (end - start)], "click");
  revealInEditor([start, start + command.length]);
});

canvas.addEventListener("pointercancel", cancelDrag);

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

function onEditorCursor() {
  updateCursor();
  selectFromEditor();
}
editor.addEventListener("keyup", onEditorCursor);
editor.addEventListener("click", onEditorCursor);
editor.addEventListener("select", onEditorCursor);

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

document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && drag) cancelDrag();
  else if (e.key === "Escape" && document.activeElement !== editor) select(null, null);
});

debug.addEventListener("change", requestPreview);
dpmm.addEventListener("change", () => {
  save(DPMM_KEY, dpmm.value);
  requestPreview();
});
zoom.addEventListener("change", applyZoom);

function download(blob, name) {
  const url = URL.createObjectURL(blob);
  const a = Object.assign(document.createElement("a"), { href: url, download: name });
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

document.getElementById("png").addEventListener("click", () => {
  const name = pageCount > 1 ? `label-${page + 1}.png` : "label.png";
  canvas.toBlob((blob) => blob && download(blob, name), "image/png");
});

document.getElementById("pdf").addEventListener("click", async () => {
  const result = await request("pdf");
  if (result.ok) download(new Blob([result.bytes], { type: "application/pdf" }), "label.pdf");
  else showProblem(result);
});

startWorker();
updateCursor();
requestPreview();
