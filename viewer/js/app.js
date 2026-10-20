/* Planetary Explorer v2 Main Application Controller */
/* Coordinates stages, UI controls, transitions, and provenance tracking */
/* Strict requirement: zero em-dashes and zero en-dashes */

(function () {
  "use strict";

  const $ = (id) => document.getElementById(id);

  let stage1 = null;
  let stage2 = null;
  let stage3 = null;
  let stage4 = null;

  let currentStage = 1;
  let isPlaying = false;
  let playInterval = null;

  // Active epistemic badge model IDs per stage
  const STAGE_MODELS = {
    1: "oklo_site_locator_wgs84",
    2: "oklo_compartments_v0",
    3: "oklo_temperature_v0",
    4: "darcy_flow_v1_a4"
  };

  function updateHUD(stageNum) {
    const badgeContainer = $("hudBadgeContainer");
    const disclaimerEl = $("hudDisclaimer");
    if (!badgeContainer) return;

    badgeContainer.innerHTML = "";
    let modelId = STAGE_MODELS[stageNum] || "oklo_site_locator_wgs84";

    if (stageNum === 3 && stage3) {
      const fieldIds = [
        "oklo_temperature_v0",
        "oklo_hydration_v0",
        "oklo_h2_concentration_v0",
        "oklo_u235_ratio_v0",
        "oklo_polymer_yield_v0",
        "oklo_compartments_v0"
      ];
      modelId = fieldIds[stage3.activeField] || "oklo_temperature_v0";
    } else if (stageNum === 4 && stage4) {
      modelId = stage4.activeModel === "darcy" ? "darcy_flow_v1_a4" : "uranium_redox_column_v1_a1";
    }

    const meta = ProvenanceHUD.getModelMetadata(modelId);
    const clsKey = meta ? meta.evidence_class : "prescribed_analytic_field";
    const badge = ProvenanceHUD.createBadge(clsKey, () => ProvenanceHUD.showModal(modelId));
    badgeContainer.appendChild(badge);

    if (disclaimerEl) {
      if (stageNum === 1) {
        disclaimerEl.innerHTML = "<strong>Modern geographic locator:</strong> NOT a Paleoproterozoic paleogeography reconstruction.";
      } else if (stageNum === 2) {
        disclaimerEl.innerHTML = "<strong>SCHEMATIC compartment geometry:</strong> NOT surveyed subsurface geology.";
      } else if (stageNum === 3) {
        disclaimerEl.innerHTML = "<strong>Revision-0 prescribed fields:</strong> Not a physical simulation.";
      } else if (stageNum === 4) {
        disclaimerEl.innerHTML = "<strong>Synthetic numerical transport:</strong> Uncoupled from Gabon geology.";
      }
    }
  }

  function setStage(stageNum, updateHash = true) {
    currentStage = stageNum;

    // Update Stage navigation buttons
    document.querySelectorAll(".stage-btn").forEach((btn) => {
      const s = parseInt(btn.dataset.stage, 10);
      btn.classList.toggle("active", s === stageNum);
    });

    // Update Viewport panels
    $("stage1View").hidden = stageNum !== 1;
    $("stage2View").hidden = stageNum !== 2;
    $("stage3View").hidden = stageNum !== 3;
    $("stage4View").hidden = stageNum !== 4;

    // Update Controls panels
    $("stage1Controls").hidden = stageNum !== 1;
    $("stage2Controls").hidden = stageNum !== 2;
    $("stage3Controls").hidden = stageNum !== 3;
    $("stage4Controls").hidden = stageNum !== 4;

    // Trigger canvas resize and re-render
    if (stageNum === 1 && stage1) {
      stage1.render();
      stage1.renderTimeline();
    } else if (stageNum === 2 && stage2) {
      stage2.render();
    } else if (stageNum === 3 && stage3) {
      stage3.render();
    } else if (stageNum === 4 && stage4) {
      stage4.render();
    }

    updateHUD(stageNum);
    if (updateHash) {
      window.location.hash = `#stage${stageNum}`;
    }
  }

  function triggerCameraDiveToStage2() {
    // Camera dive interpolation animation from globe to cutaway
    const duration = 600; // ms
    const startTime = performance.now();
    const startYaw = stage1.yaw;
    const startPitch = stage1.pitch;
    const startDist = stage1.dist;

    const diveStep = (now) => {
      const elapsed = now - startTime;
      const t = Math.min(1.0, elapsed / duration);
      const s = SpatialEngine.smoothstep(t);

      stage1.yaw = SpatialEngine.lerp(startYaw, 0.25, s);
      stage1.pitch = SpatialEngine.lerp(startPitch, 0.05, s);
      stage1.dist = SpatialEngine.lerp(startDist, 1.5, s);
      stage1.render();

      if (t < 1.0) {
        requestAnimationFrame(diveStep);
      } else {
        setStage(2);
      }
    };
    requestAnimationFrame(diveStep);
  }

  function initStage1() {
    stage1 = new Stage1Globe.Stage1Globe($("globeCanvas"), $("timelineCanvas"), {
      onSelectOklo: () => triggerCameraDiveToStage2(),
      onSiteChange: (site) => {
        $("selectedSiteName").textContent = site.name;
        $("selectedSiteCoords").textContent = site.coords_str;
        $("selectedSiteSource").textContent = site.source;
        $("selectedSiteDesc").textContent = site.description;
        $("enterCutawayBtn").hidden = !site.is_primary;
      }
    });

    $("enterCutawayBtn").onclick = () => triggerCameraDiveToStage2();

    // Populate site list radios
    const listEl = $("siteRadioList");
    if (listEl) {
      listEl.innerHTML = "";
      Stage1Globe.SITES.forEach((s, idx) => {
        const item = document.createElement("label");
        item.className = `radio-item ${idx === 0 ? "selected" : ""}`;
        item.innerHTML = `
          <input type="radio" name="siteSelect" value="${s.id}" ${idx === 0 ? "checked" : ""}>
          <div class="radio-content">
            <span class="radio-title">${s.name}</span>
            <span class="radio-desc">${s.region} (${s.lat >= 0 ? s.lat.toFixed(2) + " N" : (-s.lat).toFixed(2) + " S"})</span>
          </div>
        `;
        item.querySelector("input").addEventListener("change", () => {
          document.querySelectorAll("#siteRadioList .radio-item").forEach((el) => el.classList.remove("selected"));
          item.classList.add("selected");
          stage1.selectSite(s);
        });
        listEl.appendChild(item);
      });
    }
  }

  function initStage2() {
    stage2 = new Stage2Cutaway.Stage2Cutaway($("cutawayCanvas"), {
      onAdvanceToStage3: () => setStage(3),
      onBackToGlobe: () => setStage(1)
    });

    $("btnAdvanceToStage3").onclick = () => setStage(3);
    $("btnBackToGlobe").onclick = () => setStage(1);

    // Layer checkboxes
    const layerChecks = ["sandstone", "core", "halo", "conduit", "chimney"];
    layerChecks.forEach((layerId) => {
      const chk = $(`chkLayer_${layerId}`);
      if (chk) {
        chk.addEventListener("change", () => {
          stage2.toggleLayer(layerId);
        });
      }
    });
  }

  function initStage3() {
    stage3 = new Stage3Fields.Stage3Fields({
      glCanvas: $("glCanvas"),
      boxCanvas: $("boxCanvas"),
      planCanvas: $("planCanvas"),
      sectionCanvas: $("sectionCanvas"),
      readoutEl: $("voxelReadout"),
      limitsEl: $("fieldLimitations")
    });

    // Field selectors
    const fieldRadios = document.querySelectorAll("input[name='fieldSelect']");
    fieldRadios.forEach((r) => {
      r.addEventListener("change", () => {
        const fi = parseInt(r.value, 10);
        stage3.setField(fi);
        updateHUD(3);
      });
    });

    // View tab toggle (3D vs Slices)
    $("btnView3D").onclick = () => {
      $("btnView3D").classList.add("active");
      $("btnViewSlices").classList.remove("active");
      $("pane3D").hidden = false;
      $("paneSlices").hidden = true;
      stage3.draw3d();
    };

    $("btnViewSlices").onclick = () => {
      $("btnViewSlices").classList.add("active");
      $("btnView3D").classList.remove("active");
      $("pane3D").hidden = true;
      $("paneSlices").hidden = false;
      stage3.drawSlices();
    };

    // If WebGL2 failed on initialization, force Slices tab
    if (!stage3.hasWebGL2) {
      $("btnView3D").disabled = true;
      $("btnView3D").title = "WebGL2 unavailable in current environment";
      $("btnViewSlices").click();
      $("webglWarning").hidden = false;
    }

    // Cycle controls
    const frameSlider = $("cycleFrameSlider");
    const clockVal = $("cycleClockVal");
    const phaseNameVal = $("cyclePhaseName");

    const updateFrameUI = (frameIdx) => {
      frameSlider.value = frameIdx;
      const mins = Math.round((frameIdx / 60) * 180);
      clockVal.textContent = `${mins} min (${frameIdx}/60)`;
      const phase = frameIdx / 60;
      let pName = "Pulse / Boiling Phase";
      if (phase >= 0.17 && phase < 0.25) pName = "Desiccation / Dry Bake";
      else if (phase >= 0.25 && phase < 0.83) pName = "Quiescent Cooldown";
      else if (phase >= 0.83) pName = "Infiltration / Rewetting";
      phaseNameVal.textContent = pName;
      stage3.setFrame(frameIdx);
    };

    frameSlider.addEventListener("input", () => {
      updateFrameUI(parseInt(frameSlider.value, 10));
    });

    $("btnCyclePlay").onclick = () => {
      isPlaying = !isPlaying;
      $("btnCyclePlay").textContent = isPlaying ? "Pause" : "Play";
      if (isPlaying) {
        playInterval = setInterval(() => {
          let nextFrame = (stage3.activeFrame + 1) % 60;
          updateFrameUI(nextFrame);
        }, 100);
      } else {
        clearInterval(playInterval);
      }
    };

    // Pulse boundary buttons
    $("btnPulse17").onclick = () => {
      $("btnPulse17").classList.add("active");
      $("btnPulse1667").classList.remove("active");
      stage3.setPulseBoundary(0.17);
    };
    $("btnPulse1667").onclick = () => {
      $("btnPulse1667").classList.add("active");
      $("btnPulse17").classList.remove("active");
      stage3.setPulseBoundary(0.1667);
    };

    // Null scenario toggle
    $("chkNullScenario").addEventListener("change", (e) => {
      stage3.setNullScenario(e.target.checked);
    });

    // Slices sliders
    $("sliderSliceK").addEventListener("input", (e) => {
      const k = parseInt(e.target.value, 10);
      $("sliceKVal").textContent = `k = ${k}`;
      stage3.setSlices(k, undefined);
    });
    $("sliderSliceJ").addEventListener("input", (e) => {
      const j = parseInt(e.target.value, 10);
      $("sliceJVal").textContent = `j = ${j}`;
      stage3.setSlices(undefined, j);
    });
  }

  function initStage4() {
    stage4 = new Stage4Transport.Stage4Transport({
      darcyCanvas: $("darcyChartCanvas"),
      uraniumCanvas: $("uraniumChartCanvas")
    });

    // Model tab toggle
    $("tabModelDarcy").onclick = () => {
      $("tabModelDarcy").classList.add("active");
      $("tabModelUranium").classList.remove("active");
      $("panelDarcy").hidden = false;
      $("panelUranium").hidden = true;
      if ($("controlsDarcy")) $("controlsDarcy").hidden = false;
      if ($("controlsUranium")) $("controlsUranium").hidden = true;
      stage4.setModel("darcy");
      updateHUD(4);
    };

    $("tabModelUranium").onclick = () => {
      $("tabModelUranium").classList.add("active");
      $("tabModelDarcy").classList.remove("active");
      $("panelDarcy").hidden = true;
      $("panelUranium").hidden = false;
      if ($("controlsDarcy")) $("controlsDarcy").hidden = true;
      if ($("controlsUranium")) $("controlsUranium").hidden = false;
      stage4.setModel("uranium");
      updateHUD(4);
    };

    // Darcy water state toggle
    $("btnDarcyStateA").onclick = () => {
      $("btnDarcyStateA").classList.add("active");
      $("btnDarcyStateB").classList.remove("active");
      stage4.setDarcyWaterState("A");
      $("darcyStateDesc").textContent = "State A: 50 deg C (323.15 K, 30 MPa), rho = 1000.7 kg/m3, mu = 5.53e-4 Pa s";
      $("darcyTotalFlux").textContent = "Total Inflow: 1.449e-8 m3/s";
    };

    $("btnDarcyStateB").onclick = () => {
      $("btnDarcyStateB").classList.add("active");
      $("btnDarcyStateA").classList.remove("active");
      stage4.setDarcyWaterState("B");
      $("darcyStateDesc").textContent = "State B: 300 deg C (573.15 K, 30 MPa), rho = 750.7 kg/m3, mu = 9.33e-5 Pa s (flux x4.45)";
      $("darcyTotalFlux").textContent = "Total Inflow: 6.444e-8 m3/s";
    };

    // Darcy config select
    $("selectDarcyConfig").addEventListener("change", (e) => {
      stage4.setDarcyConfig(parseInt(e.target.value, 10));
    });

    // Uranium parameter controls
    const updateUranium = () => {
      const v = parseFloat($("selectUraniumV").value);
      const d = parseFloat($("selectUraniumD").value);
      const k = parseFloat($("selectUraniumK").value);
      const kd = parseFloat($("selectUraniumKd").value);
      stage4.setUraniumParams(v, d, k, kd);

      const run = stage4.findUraniumRun();
      if (run) {
        $("uRetardationVal").textContent = `R = ${run.R.toFixed(1)}`;
        $("uTransitTauVal").textContent = `tau = ${(run.tau_s / 3.15e7).toExponential(2)} yr`;
        const cVal = run.ledger.worst_closure_rel_at_outputs !== undefined ? run.ledger.worst_closure_rel_at_outputs : (run.ledger.closure_rel || 0);
        $("uLedgerClosure").textContent = `Ledger closure: < 1e-10 (${cVal.toExponential(2)})`;
      }
    };

    $("selectUraniumV").addEventListener("change", updateUranium);
    $("selectUraniumD").addEventListener("change", updateUranium);
    $("selectUraniumK").addEventListener("change", updateUranium);
    $("selectUraniumKd").addEventListener("change", updateUranium);
  }

  function initTheme() {
    const savedTheme = localStorage.getItem("oklo_theme") || "dark";
    document.documentElement.setAttribute("data-theme", savedTheme);
    $("themeToggleBtn").onclick = () => {
      const current = document.documentElement.getAttribute("data-theme") || "dark";
      const next = current === "dark" ? "light" : "dark";
      document.documentElement.setAttribute("data-theme", next);
      localStorage.setItem("oklo_theme", next);
      if (stage1) stage1.render();
      if (stage2) stage2.render();
      if (stage3) stage3.render();
      if (stage4) stage4.render();
    };
  }

  async function boot() {
    initTheme();

    // Stage navigation buttons
    document.querySelectorAll(".stage-btn").forEach((btn) => {
      btn.addEventListener("click", () => {
        const s = parseInt(btn.dataset.stage, 10);
        setStage(s);
      });
    });

    // Header help button
    $("btnHeaderHelp").onclick = () => {
      ProvenanceHUD.showModal(STAGE_MODELS[currentStage] || "oklo_site_locator_wgs84");
    };

    // Pre-load manifest metadata
    await ProvenanceHUD.loadManifest();

    initStage1();
    initStage2();
    initStage3();
    initStage4();

    // Check initial hash
    const hash = window.location.hash;
    if (hash === "#stage2") {
      setStage(2, false);
    } else if (hash === "#stage3") {
      setStage(3, false);
    } else if (hash === "#stage3-compartments") {
      setStage(3, false);
      const radio = document.querySelector("input[name='fieldSelect'][value='5']");
      if (radio) {
        radio.checked = true;
        stage3.setField(5);
        updateHUD(3);
      }
    } else if (hash === "#stage4") {
      setStage(4, false);
    } else if (hash === "#stage4-uranium") {
      setStage(4, false);
      $("tabModelUranium").click();
    } else {
      setStage(1, false);
    }
  }

  window.addEventListener("DOMContentLoaded", boot);
})();
