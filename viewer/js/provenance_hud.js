/* Provenance HUD and Epistemic Metadata Layer */
/* Manages evidence class badges, modal disclosures, and bibliographic citations */
/* Strict requirement: zero em-dashes and zero en-dashes */

(function (root, factory) {
  if (typeof module === "object" && module.exports) {
    module.exports = factory();
  } else {
    root.ProvenanceHUD = factory();
  }
})(typeof self !== "undefined" ? self : this, function () {
  "use strict";

  const EVIDENCE_CLASSES = {
    geographic_observation_or_locator: {
      id: "geographic_observation_or_locator",
      label: "Geographic Observation",
      badgeClass: "badge-geo",
      color: "#059669",
      description: "Direct empirical geographic measurements (WGS84 locators, basemaps). Does not depict Paleoproterozoic paleogeography."
    },
    sourced_geological_record: {
      id: "sourced_geological_record",
      label: "Sourced Geological Record",
      badgeClass: "badge-geol",
      color: "#2563eb",
      description: "Empirical measurements published in peer-reviewed scientific literature (isotopic ratios, radiometric dates, core assays)."
    },
    prescribed_analytic_field: {
      id: "prescribed_analytic_field",
      label: "Prescribed Analytic Field",
      badgeClass: "badge-field",
      color: "#d97706",
      description: "Revision-0 closed-form formulas constructed for scenario exploration. Not a physical simulation or calibrated thermal-hydraulic solution."
    },
    numerically_verified_synthetic_model: {
      id: "numerically_verified_synthetic_model",
      label: "Numerically Verified Model",
      badgeClass: "badge-synth",
      color: "#4f46e5",
      description: "Rigorous numerical differential equation solves on idealized synthetic geometries. Not calibrated to real subsurface field geology."
    },
    hypothesis_or_unvalidated_model: {
      id: "hypothesis_or_unvalidated_model",
      label: "Hypothesis / Unvalidated",
      badgeClass: "badge-hypo",
      color: "#9333ea",
      description: "Conceptual frameworks, speculative reaction mechanisms, or unvalidated biophysical conjectures that lack experimental proof."
    },
    empirically_calibrated_model: {
      id: "empirically_calibrated_model",
      label: "Empirically Calibrated Model",
      badgeClass: "badge-calib",
      color: "#dc2626",
      description: "Models calibrated against real-world field observations with formal residual audits. Strictly zero models belong to this class in this prototype."
    }
  };

  let modelsManifest = null;
  let activeModalModelId = null;

  async function loadManifest(manifestUrl = "data/models_manifest.json") {
    try {
      const res = await fetch(manifestUrl);
      if (res.ok) {
        modelsManifest = await res.json();
        return modelsManifest;
      }
    } catch (_) {
      // Fallback relative path check
      try {
        const altRes = await fetch("../data/models/manifest.json");
        if (altRes.ok) {
          modelsManifest = await altRes.json();
          return modelsManifest;
        }
      } catch (e) {
        console.warn("ProvenanceHUD: unable to load remote manifest, using default metadata registry.");
      }
    }
    return null;
  }

  function getModelMetadata(modelId) {
    if (modelsManifest && modelsManifest.models) {
      const found = modelsManifest.models.find((m) => m.model_id === modelId);
      if (found) return found;
    }
    // Minimal fallback entries for primary prototype models
    const fallbacks = {
      oklo_site_locator_wgs84: {
        model_id: "oklo_site_locator_wgs84",
        model_version: "1.0.0",
        evidence_class: "geographic_observation_or_locator",
        specification_version: "Gauthier-Lafaye et al. 1996",
        units: "degrees WGS84",
        coordinate_system: "FRAME_GEO_WGS84",
        known_limitations: "Modern geographic locator for the Oklo mine pit in Gabon (-1.4000 deg, 13.1833 deg). Not a Paleoproterozoic paleogeography reconstruction."
      },
      oklo_temperature_v0: {
        model_id: "oklo_temperature_v0",
        model_version: "0.1.0",
        evidence_class: "prescribed_analytic_field",
        specification_version: "CONFORMANCE_FIXTURE_SPEC.md v2.1",
        units: "degrees C as labelled; coefficients underived",
        coordinate_system: "FRAME_FIXTURE_GRID",
        known_limitations: "Not a thermal-hydraulic solution. Prescribed algebraic temperature function; must not be used to state historical temperatures at Oklo."
      },
      darcy_flow_v1_a4: {
        model_id: "darcy_flow_v1_a4",
        model_version: "1.0.0",
        evidence_class: "numerically_verified_synthetic_model",
        specification_version: "DARCY_FLOW_PREREG_v1.md + A1-A4",
        run_receipt: "receipts/darcy_v1_A4_run_2026-10-18.json",
        units: "m/s (Darcy flux), m (hydraulic head)",
        coordinate_system: "FRAME_LOCAL_CARTESIAN [0, 16m]^3",
        known_limitations: "HYPOTHETICAL numerical experiment; Darcy v1+A1-A3. Not a description of Oklo hydrogeology. Layer interface check D3a reached 2.58e-10 relative error exceeding 1e-10 tolerance under A4 CG stopping rule."
      },
      uranium_redox_column_v1_a1: {
        model_id: "uranium_redox_column_v1_a1",
        model_version: "1.0.0",
        evidence_class: "numerically_verified_synthetic_model",
        specification_version: "URANIUM_REDOX_TRANSPORT_PREREG_v1.md + A1",
        run_receipt: "receipts/uranium_v1_A1_run_2026-10-18.json",
        units: "mol/m^3 (aqueous U(VI)), mol/kg (sorbed/precipitated U(IV))",
        coordinate_system: "FRAME_TRANSPORT_1D [0, 16m]",
        known_limitations: "SYNTHETIC 1D column; uranium prereg v1+A1. Not a reconstruction of Oklo or the Franceville Basin. Retardation values up to R = 9.46e6 represent EPA sensitivity bounds rather than site conditions."
      }
    };
    return fallbacks[modelId] || null;
  }

  function createBadge(evidenceClassKey, onClickHandler) {
    const meta = EVIDENCE_CLASSES[evidenceClassKey] || {
      label: evidenceClassKey,
      badgeClass: "badge-hypo"
    };
    const badge = document.createElement("button");
    badge.type = "button";
    badge.className = `epistemic-badge ${meta.badgeClass}`;
    badge.textContent = `[${meta.label}]`;
    badge.setAttribute("title", "Click to inspect epistemic provenance and limitations");
    if (onClickHandler) {
      badge.addEventListener("click", onClickHandler);
    }
    return badge;
  }

  function showModal(modelId) {
    activeModalModelId = modelId;
    const meta = getModelMetadata(modelId);
    let modalEl = document.getElementById("provenanceModal");
    if (!modalEl) {
      modalEl = document.createElement("div");
      modalEl.id = "provenanceModal";
      modalEl.className = "modal-backdrop";
      modalEl.innerHTML = `
        <div class="modal-dialog" role="dialog" aria-modal="true" aria-labelledby="modalTitle">
          <div class="modal-header">
            <h3 class="modal-title" id="modalTitle">Epistemic Metadata &amp; Provenance</h3>
            <button type="button" class="modal-close" id="modalCloseBtn" aria-label="Close modal">&times;</button>
          </div>
          <div class="modal-body" id="modalBody"></div>
          <div class="modal-footer">
            <button type="button" class="btn-primary" id="modalDismissBtn">Close</button>
          </div>
        </div>
      `;
      document.body.appendChild(modalEl);

      const close = () => modalEl.classList.remove("open");
      modalEl.querySelector("#modalCloseBtn").onclick = close;
      modalEl.querySelector("#modalDismissBtn").onclick = close;
      modalEl.onclick = (e) => {
        if (e.target === modalEl) close();
      };
    }

    const body = modalEl.querySelector("#modalBody");
    if (!meta) {
      body.innerHTML = `<p>No metadata registered for model ID: <code>${modelId}</code>.</p>`;
    } else {
      const cls = EVIDENCE_CLASSES[meta.evidence_class] || { label: meta.evidence_class, badgeClass: "badge-hypo", description: "" };
      body.innerHTML = `
        <div style="display:flex; align-items:center; justify-content:space-between; gap:10px;">
          <strong style="font-size:15px; font-family:var(--font-mono);">${meta.model_id}</strong>
          <span class="epistemic-badge ${cls.badgeClass}">[${cls.label}]</span>
        </div>
        <p style="color:var(--fg-secondary); font-size:12px;">${cls.description}</p>
        
        <div class="callout-box callout-warn">
          <strong>Known Limitations &amp; Boundary Statements:</strong>
          <div style="margin-top:4px;">${meta.known_limitations || "None stated."}</div>
        </div>

        <table class="telemetry-table">
          <tbody>
            <tr><th>Model Version</th><td class="num">${meta.model_version || "1.0.0"}</td></tr>
            <tr><th>Specification</th><td>${meta.specification_version || "n/a"}</td></tr>
            <tr><th>Run Receipt</th><td class="num">${meta.run_receipt || "Precomputed / inline"}</td></tr>
            <tr><th>Units</th><td class="num">${meta.units || "dimensionless"}</td></tr>
            <tr><th>Coordinate System</th><td class="num">${meta.coordinate_system || "n/a"}</td></tr>
            <tr><th>Source Repository</th><td>${meta.source_repository || "oklo_pool_mobile"}</td></tr>
          </tbody>
        </table>
      `;
    }

    modalEl.classList.add("open");
  }

  return {
    EVIDENCE_CLASSES,
    loadManifest,
    getModelMetadata,
    createBadge,
    showModal
  };
});
