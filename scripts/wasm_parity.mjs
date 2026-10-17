// Compare the wasm engine with the native series, frame by frame, and write the
// native data-block hashes the viewer uses for its in-page check.
//
//   cargo build --release --target wasm32-unknown-unknown -p oklo_web
//   cargo run --release --example export_series -- out/series/b0.17 0.17
//   cargo run --release --example export_series -- out/series/b0.1667 0.1667
//   node scripts/wasm_parity.mjs
//
// Differences are reported, never tuned away (README, M0 policy).

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { createHash } from "node:crypto";

const FIELDS = ["temperature", "hydration", "h2_concentration", "u235_ratio", "polymer_yield", "compartments"];
const BOUNDARIES = ["0.17", "0.1667"];
const FRAMES = 60;
const VOXELS = 32 * 32 * 32;

const wasm = await WebAssembly.instantiate(readFileSync("target/wasm32-unknown-unknown/release/oklo_web.wasm"));
const { memory, oklo_alloc, oklo_compute } = wasm.instance.exports;
const ptr = oklo_alloc(VOXELS);

const hashes = {};
let identical = 0, total = 0;
const mismatches = [];
for (const b of BOUNDARIES) {
  hashes[b] = {};
  for (const [fid, name] of FIELDS.entries()) {
    hashes[b][name] = [];
    for (let i = 0; i < FRAMES; i++) {
      const native = readFileSync(`out/series/b${b}/${name}_f${String(i).padStart(2, "0")}.mrc`).subarray(1024);
      hashes[b][name].push(createHash("sha256").update(native).digest("hex"));
      if (oklo_compute(fid, i / FRAMES, 0, Number(b), ptr) !== 0) throw new Error(`oklo_compute failed: ${name} frame ${i} b ${b}`);
      const ours = new Uint8Array(memory.buffer, ptr, VOXELS * 4);
      total++;
      if (Buffer.compare(Buffer.from(ours), native) === 0) { identical++; continue; }
      const a = new Float32Array(memory.buffer, ptr, VOXELS);
      const r = new Float32Array(native.buffer.slice(native.byteOffset, native.byteOffset + VOXELS * 4));
      let diffs = 0, maxAbs = 0;
      for (let v = 0; v < VOXELS; v++) if (a[v] !== r[v]) { diffs++; maxAbs = Math.max(maxAbs, Math.abs(a[v] - r[v])); }
      mismatches.push({ b, name, frame: i, voxels_differing: diffs, max_abs_diff: maxAbs });
    }
  }
}

mkdirSync("viewer/data", { recursive: true });
writeFileSync("viewer/data/native_hashes.json", JSON.stringify({ frames: FRAMES, note: "SHA-256 of native oklo_core MRC data blocks, little-endian float32, X fastest", hashes }));
console.log(`wasm vs native: ${identical} of ${total} frames byte-identical`);
if (mismatches.length) {
  console.log(`${mismatches.length} frames differ:`);
  for (const m of mismatches.slice(0, 20)) console.log(JSON.stringify(m));
}
