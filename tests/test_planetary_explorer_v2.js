/**
 * Automated Verification Suite for Planetary Explorer v2
 * Tests coordinate math, timeline parsing, field calculations, epistemic contracts,
 * and transport model data integrity.
 *
 * Strict requirement: zero em-dashes and zero en-dashes
 */

const assert = require("assert");
const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

const SpatialEngine = require("../viewer/js/spatial_engine.js");
const ProvenanceHUD = require("../viewer/js/provenance_hud.js");
const Stage1Globe = require("../viewer/js/stage1_globe.js");
const Stage2Cutaway = require("../viewer/js/stage2_cutaway.js");
const Stage3Fields = require("../viewer/js/stage3_fields.js");
const Stage4Transport = require("../viewer/js/stage4_transport.js");

console.log("=== Planetary Explorer v2 Automated Verification Suite ===");

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    console.log(`PASS: ${name}`);
    passed++;
  } catch (err) {
    console.error(`FAIL: ${name}`);
    console.error(err);
    failed++;
  }
}

// 1. Coordinate Transformation Unit Tests
test("Spatial: WGS84 to Cartesian and back (round-trip error < 1e-6)", () => {
  const testCoords = [
    [-1.4000, 13.1833],   // Oklo
    [14.2417, 40.3000],   // Dallol
    [44.7283, -110.7042], // Yellowstone
    [30.1167, -42.1167],  // Lost City
    [0.0, 0.0],           // Equator / Prime Meridian
    [89.9, 0.0],          // Near North Pole
    [-89.9, 0.0]          // Near South Pole
  ];

  for (const [lat, lon] of testCoords) {
    const [x, y, z] = SpatialEngine.wgs84ToCartesian(lat, lon, 1.0);
    const r = Math.hypot(x, y, z);
    assert(Math.abs(r - 1.0) < 1e-12, `Unit sphere radius violated: ${r}`);

    const back = SpatialEngine.cartesianToWgs84(x, y, z);
    const errLat = Math.abs(back.lat - lat);
    const errLon = Math.abs(back.lon - lon);
    assert(errLat < 1e-6, `Latitude round-trip error too large: ${errLat}`);
    assert(errLon < 1e-6, `Longitude round-trip error too large: ${errLon}`);
  }
});

test("Spatial: Coordinate boundary rejection", () => {
  assert.throws(() => SpatialEngine.wgs84ToCartesian(91.0, 0.0), /out of valid bounds/);
  assert.throws(() => SpatialEngine.wgs84ToCartesian(-90.1, 0.0), /out of valid bounds/);
  assert.throws(() => SpatialEngine.wgs84ToCartesian(0.0, 180.1), /out of valid bounds/);
  assert.throws(() => SpatialEngine.wgs84ToCartesian(0.0, -180.1), /out of valid bounds/);
  assert.throws(() => SpatialEngine.wgs84ToCartesian(NaN, 0.0), /Invalid geographic coordinates/);
});

test("Spatial: Camera pitch clamping to [-1.45, 1.45] radians", () => {
  const basis1 = SpatialEngine.computeCameraBasis(0, 2.0, 2.0);
  assert(Math.abs(basis1.pos[1] - (2.0 * Math.sin(1.45))) < 1e-6);

  const basis2 = SpatialEngine.computeCameraBasis(0, -2.0, 2.0);
  assert(Math.abs(basis2.pos[1] - (-2.0 * Math.sin(1.45))) < 1e-6);
});

// 2. Timeline Parsing Tests
test("Timeline: Non-merging of literature definitions in paleo_intervals_v1.json", () => {
  const p = path.join(__dirname, "../data/timeline/paleo_intervals_v1.json");
  const data = JSON.parse(fs.readFileSync(p, "utf-8"));

  assert.strictEqual(data.entries.length, 4);
  const ids = data.entries.map((e) => e.id);
  assert(ids.includes("goe_gumsley_2017"));
  assert(ids.includes("goe_as_framed_by_craig_2023"));
  assert(ids.includes("oklo_reactors"));
  assert(ids.includes("eukaryogenesis_craig_2023"));

  // Check Gumsley end vs Craig older bound are distinct
  const gumsley = data.entries.find((e) => e.id === "goe_gumsley_2017");
  const craig = data.entries.find((e) => e.id === "goe_as_framed_by_craig_2023");
  assert.strictEqual(gumsley.end_ga, 2.060);
  assert.strictEqual(craig.older_ga, 2.43);
  assert.strictEqual(craig.younger_ga, 2.22);

  // Check Oklo dated marker uncertainty
  const oklo = data.entries.find((e) => e.id === "oklo_reactors");
  assert.strictEqual(oklo.value_ga, 1.95);
  assert.strictEqual(oklo.uncertainty_ga, 0.04);
});

// 3. Field Slicing and Bit-Identical Hash Tests
test("Fields: Frame 9 bit-identical SHA-256 hashes against frozen SPEC", () => {
  const SPEC = {
    temperature: "89d85fc802e4f4d638af529f66bf7f3eb73b14e704afaf31dc652b68c9cb8e47",
    hydration: "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
    h2_concentration: "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
    u235_ratio: "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
    polymer_yield: "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
    compartments: "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d"
  };

  const buf = new Float32Array(32 * 32 * 32);
  const names = ["temperature", "hydration", "h2_concentration", "u235_ratio", "polymer_yield", "compartments"];

  for (let fid = 0; fid < 6; fid++) {
    Stage3Fields.computeFieldNative(fid, 0.15, false, 0.17, buf);
    const hash = crypto.createHash("sha256").update(Buffer.from(buf.buffer)).digest("hex");
    assert.strictEqual(hash, SPEC[names[fid]], `Hash mismatch on field ${names[fid]}`);
  }
});

test("Fields: Non-zero variance across all 6 fields", () => {
  const buf = new Float32Array(32 * 32 * 32);
  for (let fid = 0; fid < 6; fid++) {
    Stage3Fields.computeFieldNative(fid, 0.15, false, 0.17, buf);
    let mn = Infinity, mx = -Infinity;
    for (const v of buf) {
      if (v < mn) mn = v;
      if (v > mx) mx = v;
    }
    assert(mx > mn, `Field ${fid} has zero variance (mn=${mn}, mx=${mx})`);
  }
});

// 4. Transport Model Summaries Integrity Tests
test("Transport: Darcy summary loads and contains 11 configurations with honest D3a failure note", () => {
  const p = path.join(__dirname, "../viewer/data/darcy_summary.json");
  const darcy = JSON.parse(fs.readFileSync(p, "utf-8"));

  assert.strictEqual(darcy.configurations.length, 11);
  const base = darcy.configurations[0];
  assert.strictEqual(base.name, "base");
  assert(Math.abs(base.conduit_share_top_face - 0.0681858) < 1e-4);

  // Check D3a check failed status
  const d3a = darcy.checks.find((c) => c.id === "D3a");
  assert.strictEqual(d3a.status, "fail");
});

test("Transport: Uranium summary loads 144 runs with ledger closure < 1e-10 and positivity", () => {
  const p = path.join(__dirname, "../viewer/data/uranium_summary.json");
  const u = JSON.parse(fs.readFileSync(p, "utf-8"));

  assert.strictEqual(u.runs.length, 144);
  for (const r of u.runs) {
    const closure = r.ledger.worst_closure_rel_at_outputs !== undefined ? r.ledger.worst_closure_rel_at_outputs : r.ledger.closure_rel;
    assert(closure < 1e-10, `Ledger closure failed in run ${r.label}: ${closure}`);
    assert(r.R >= 1.0, `Retardation factor invalid in run ${r.label}`);
    assert(r.breakthrough.length > 0, `Breakthrough missing in run ${r.label}`);
    assert(r.snapshots.length === 6, `Snapshots count mismatch in run ${r.label}`);
  }
});

// 5. Epistemic Separation and Schema Invariants
test("Epistemic: Models manifest strictly satisfies schema and zero calibrated models", () => {
  const pManifest = path.join(__dirname, "../viewer/data/models_manifest.json");
  const manifest = JSON.parse(fs.readFileSync(pManifest, "utf-8"));

  assert.strictEqual(manifest.models.length, 18);
  const calibrated = manifest.models.filter((m) => m.evidence_class === "empirically_calibrated_model");
  assert.strictEqual(calibrated.length, 0, "No model may be classified as empirically_calibrated_model");

  const r6a = manifest.models.find((m) => m.model_id === "surface_matrix_protection_v0");
  assert(r6a, "R6A surface_matrix_protection_v0 entry must exist in models_manifest.json");
  assert.strictEqual(r6a.evidence_class, "hypothesis_or_unvalidated_model");
  assert.strictEqual(r6a.run_receipt, null);
  assert(r6a.known_limitations.includes("Does not establish taxonomic identity"));
  assert(r6a.known_limitations.includes("general surface-matrix field M_surf"));
});

// 6. Typography and Quarantine Scan
test("Typography: Zero em-dashes, zero en-dashes, and zero quarantined terms", () => {
  const targets = [
    "docs/specs/PLANETARY_EXPLORER_ARCHITECTURE_v2.md",
    "docs/specs/SURFACE_MATRIX_PROTECTION_v0.md",
    "docs/audit/PLANETARY_EXPLORER_v2_RED_TEAM_AUDIT.md",
    "viewer/planetary_explorer.html",
    "viewer/css/explorer_layout.css",
    "viewer/css/explorer_theme.css",
    "viewer/js/app.js",
    "viewer/js/spatial_engine.js",
    "viewer/js/stage1_globe.js",
    "viewer/js/stage2_cutaway.js",
    "viewer/js/stage3_fields.js",
    "viewer/js/stage4_transport.js",
    "viewer/js/provenance_hud.js"
  ];

  const quarantined = ["radiotrophic" + " methanogens", "free-will" + " octane"];

  for (const t of targets) {
    const fullPath = path.join(__dirname, "..", t);
    const content = fs.readFileSync(fullPath);
    assert(!content.includes(Buffer.from([0xe2, 0x80, 0x94])), `Em-dash found in ${t}`);
    assert(!content.includes(Buffer.from([0xe2, 0x80, 0x93])), `En-dash found in ${t}`);
    const str = content.toString("utf-8").toLowerCase();
    for (const q of quarantined) {
      assert(!str.includes(q), `Quarantined term found in ${t}`);
    }
  }
});

console.log("\n=== Test Results Summary ===");
console.log(`Passed: ${passed}`);
console.log(`Failed: ${failed}`);

if (failed > 0) {
  process.exit(1);
} else {
  console.log("ALL V2 AUTOMATED TESTS PASSED!");
}
