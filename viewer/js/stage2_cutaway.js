/* Stage 2: Geological Cutaway Schematic */
/* Interactive 3D schematic block and camera dive transition */
/* Strict requirement: zero em-dashes and zero en-dashes */

(function (root, factory) {
  if (typeof module === "object" && module.exports) {
    module.exports = factory(require("./spatial_engine.js"), require("./provenance_hud.js"));
  } else {
    root.Stage2Cutaway = factory(root.SpatialEngine, root.ProvenanceHUD);
  }
})(typeof self !== "undefined" ? self : this, function (SpatialEngine, ProvenanceHUD) {
  "use strict";

  const LAYERS = [
    {
      id: "sandstone",
      name: "Sandstone Matrix",
      zone: 0,
      color: "#d8c79f",
      description: "Host Francevillian FA formation sandstone matrix surrounding the mineralized core."
    },
    {
      id: "core",
      name: "Uranium Reactor Core",
      zone: 1,
      color: "#6b2d82",
      description: "Primary uranium-rich reaction zone (cz_core = 0.35 * nz) where criticality occurred."
    },
    {
      id: "halo",
      name: "Alteration Halo",
      zone: 2,
      color: "#8ea86c",
      description: "Hydrothermal clay and phosphate alteration envelope formed by heated fluid circulation."
    },
    {
      id: "conduit",
      name: "Fracture Conduit",
      zone: 3,
      color: "#3c7fc0",
      description: "Permeable vertical corridor extending upward from the core center plane (k >= 11)."
    },
    {
      id: "chimney",
      name: "Steam Chimney Zone",
      zone: 4,
      color: "#f0eee8",
      description: "Degassed vadose conduit at upper boundary (k >= 24) appearing during active heating."
    }
  ];

  class Stage2Cutaway {
    constructor(canvasEl, options = {}) {
      this.canvas = canvasEl;
      this.ctx = canvasEl.getContext("2d");
      this.onAdvanceToStage3 = options.onAdvanceToStage3 || null;
      this.onBackToGlobe = options.onBackToGlobe || null;

      // Local block camera orbit (centered at [0.5, 0.5, 0.5])
      this.yaw = -0.75;
      this.pitch = 0.45;
      this.dist = 2.6; // distance from block center
      this.target = [0.5, 0.5, 0.5];

      this.isDragging = false;
      this.lastMouse = [0, 0];
      this.visibleLayers = {
        sandstone: true,
        core: true,
        halo: true,
        conduit: true,
        chimney: true
      };

      this.initEvents();
    }

    initEvents() {
      const c = this.canvas;
      c.addEventListener("pointerdown", (e) => {
        c.setPointerCapture(e.pointerId);
        this.isDragging = true;
        this.lastMouse = [e.clientX, e.clientY];
      });

      c.addEventListener("pointermove", (e) => {
        if (!this.isDragging) return;
        const dx = e.clientX - this.lastMouse[0];
        const dy = e.clientY - this.lastMouse[1];
        this.yaw -= dx * 0.008;
        this.pitch = Math.max(-1.45, Math.min(1.45, this.pitch + dy * 0.008));
        this.lastMouse = [e.clientX, e.clientY];
        this.render();
      });

      const stopDrag = (e) => {
        if (this.isDragging) {
          this.isDragging = false;
          try { c.releasePointerCapture(e.pointerId); } catch (_) {}
        }
      };
      c.addEventListener("pointerup", stopDrag);
      c.addEventListener("pointercancel", stopDrag);

      c.addEventListener("wheel", (e) => {
        e.preventDefault();
        this.dist = Math.max(1.6, Math.min(4.5, this.dist * Math.exp(e.deltaY * 0.001)));
        this.render();
      }, { passive: false });

      window.addEventListener("resize", () => this.render());
    }

    toggleLayer(layerId) {
      if (this.visibleLayers.hasOwnProperty(layerId)) {
        this.visibleLayers[layerId] = !this.visibleLayers[layerId];
        this.render();
      }
    }

    projectPoint(p, w, h) {
      const cam = SpatialEngine.computeCameraBasis(this.yaw, this.pitch, this.dist, this.target, "Z");
      const r = SpatialEngine.vSub(p, cam.pos);
      const z = SpatialEngine.vDot(cam.fwd, r);
      if (z <= 0.01) return null;

      const aspect = w / h;
      const fov = 1.8;
      const px = (SpatialEngine.vDot(cam.right, r) / z) * fov / aspect;
      const py = (SpatialEngine.vDot(cam.up, r) / z) * fov;

      return [
        (px * 0.5 + 0.5) * w,
        (1.0 - (py * 0.5 + 0.5)) * h,
        z
      ];
    }

    render() {
      const c = this.canvas;
      const ctx = this.ctx;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const rect = c.getBoundingClientRect();
      const w = Math.max(1, Math.round(rect.width * dpr));
      const h = Math.max(1, Math.round(rect.height * dpr));

      if (c.width !== w || c.height !== h) {
        c.width = w;
        c.height = h;
      }

      ctx.clearRect(0, 0, w, h);

      // Gradient background
      const bg = ctx.createLinearGradient(0, 0, 0, h);
      bg.addColorStop(0, "#0d131a");
      bg.addColorStop(1, "#06090d");
      ctx.fillStyle = bg;
      ctx.fillRect(0, 0, w, h);

      // Draw 3D bounding box corners and wireframe
      // Unit cube [0, 1]^3 with k (Z) pointing upward
      const corners = [
        [0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], // Bottom face (k=0)
        [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]  // Top face (k=1)
      ];

      const projCorners = corners.map((p) => this.projectPoint(p, w, h));

      // Draw Sandstone host cutaway faces (Zone 0) if enabled
      if (this.visibleLayers.sandstone && projCorners.every(Boolean)) {
        // Base plane (k=0)
        ctx.fillStyle = "rgba(216, 199, 159, 0.14)";
        ctx.beginPath();
        ctx.moveTo(projCorners[0][0], projCorners[0][1]);
        ctx.lineTo(projCorners[1][0], projCorners[1][1]);
        ctx.lineTo(projCorners[2][0], projCorners[2][1]);
        ctx.lineTo(projCorners[3][0], projCorners[3][1]);
        ctx.closePath();
        ctx.fill();

        // Back cutaway walls (x=0 and y=1) showing sandstone matrix
        ctx.fillStyle = "rgba(216, 199, 159, 0.08)";
        ctx.beginPath();
        ctx.moveTo(projCorners[0][0], projCorners[0][1]);
        ctx.lineTo(projCorners[3][0], projCorners[3][1]);
        ctx.lineTo(projCorners[7][0], projCorners[7][1]);
        ctx.lineTo(projCorners[4][0], projCorners[4][1]);
        ctx.closePath();
        ctx.fill();

        ctx.fillStyle = "rgba(216, 199, 159, 0.06)";
        ctx.beginPath();
        ctx.moveTo(projCorners[3][0], projCorners[3][1]);
        ctx.lineTo(projCorners[2][0], projCorners[2][1]);
        ctx.lineTo(projCorners[6][0], projCorners[6][1]);
        ctx.lineTo(projCorners[7][0], projCorners[7][1]);
        ctx.closePath();
        ctx.fill();
      }

      // Draw Alteration Halo (Zone 2) around [0.5, 0.5, 0.35]
      if (this.visibleLayers.halo) {
        const haloCenter = [0.5, 0.5, 0.35];
        const projHalo = this.projectPoint(haloCenter, w, h);
        if (projHalo) {
          ctx.strokeStyle = "rgba(142, 168, 108, 0.85)";
          ctx.fillStyle = "rgba(142, 168, 108, 0.24)";
          ctx.lineWidth = 2 * dpr;
          const rx = 95 * dpr * (2.6 / this.dist);
          const ry = 56 * dpr * (2.6 / this.dist);
          ctx.beginPath();
          ctx.ellipse(projHalo[0], projHalo[1], rx, ry, 0, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          ctx.font = `${Math.round(10 * dpr)}px -apple-system, sans-serif`;
          ctx.fillStyle = "#a8c686";
          ctx.textAlign = "right";
          ctx.fillText("Alteration Halo (Zone 2)", projHalo[0] - rx - 8 * dpr, projHalo[1] + 4 * dpr);
        }
      }

      // Draw Fracture Conduit (Zone 3: vertical column from k=11/32 to k=24/32)
      if (this.visibleLayers.conduit) {
        const pBottom = this.projectPoint([0.5, 0.5, 0.35], w, h);
        const pTop = this.projectPoint([0.5, 0.5, 0.75], w, h);
        if (pBottom && pTop) {
          ctx.strokeStyle = "rgba(60, 127, 192, 0.85)";
          ctx.lineWidth = 26 * dpr * (2.6 / this.dist);
          ctx.lineCap = "round";
          ctx.beginPath();
          ctx.moveTo(pBottom[0], pBottom[1]);
          ctx.lineTo(pTop[0], pTop[1]);
          ctx.stroke();
          ctx.lineCap = "butt";

          ctx.font = `${Math.round(11 * dpr)}px -apple-system, sans-serif`;
          ctx.fillStyle = "#58a6ff";
          ctx.textAlign = "left";
          ctx.fillText("Fracture Conduit (Zone 3)", pTop[0] + 22 * dpr, (pBottom[1] + pTop[1]) * 0.5);
        }
      }

      // Draw Steam Chimney (Zone 4: upper conduit k >= 24/32 = 0.75 to 1.0)
      if (this.visibleLayers.chimney) {
        const pChimneyBot = this.projectPoint([0.5, 0.5, 0.75], w, h);
        const pChimneyTop = this.projectPoint([0.5, 0.5, 0.96], w, h);
        if (pChimneyBot && pChimneyTop) {
          ctx.strokeStyle = "rgba(240, 238, 232, 0.75)";
          ctx.lineWidth = 30 * dpr * (2.6 / this.dist);
          ctx.lineCap = "round";
          ctx.beginPath();
          ctx.moveTo(pChimneyBot[0], pChimneyBot[1]);
          ctx.lineTo(pChimneyTop[0], pChimneyTop[1]);
          ctx.stroke();
          ctx.lineCap = "butt";

          ctx.strokeStyle = "rgba(240, 238, 232, 0.9)";
          ctx.fillStyle = "rgba(240, 238, 232, 0.35)";
          ctx.lineWidth = 1.5 * dpr;
          const rx = 28 * dpr * (2.6 / this.dist);
          const ry = 14 * dpr * (2.6 / this.dist);
          ctx.beginPath();
          ctx.ellipse(pChimneyTop[0], pChimneyTop[1], rx, ry, 0, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          ctx.font = `${Math.round(11 * dpr)}px -apple-system, sans-serif`;
          ctx.fillStyle = "rgba(240, 238, 232, 0.95)";
          ctx.textAlign = "left";
          ctx.fillText("Steam Chimney (Zone 4, k >= 24)", pChimneyTop[0] + rx + 10 * dpr, pChimneyTop[1] + 4 * dpr);
        }
      }

      // Draw Reactor Core (Zone 1)
      if (this.visibleLayers.core) {
        const coreCenter = [0.5, 0.5, 0.35];
        const projCore = this.projectPoint(coreCenter, w, h);
        if (projCore) {
          ctx.strokeStyle = "#b565d9";
          ctx.fillStyle = "rgba(107, 45, 130, 0.88)";
          ctx.lineWidth = 2 * dpr;
          const rx = 54 * dpr * (2.6 / this.dist);
          const ry = 32 * dpr * (2.6 / this.dist);
          ctx.beginPath();
          ctx.ellipse(projCore[0], projCore[1], rx, ry, 0, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          // Core label
          ctx.font = `600 ${Math.round(11 * dpr)}px -apple-system, sans-serif`;
          ctx.fillStyle = "#ffffff";
          ctx.textAlign = "center";
          ctx.fillText("Reactor Core (Zone 1)", projCore[0], projCore[1] + 4 * dpr);
        }
      }

      // Draw Bounding Cube Wireframe Edges
      const edges = [
        [0, 1], [1, 2], [2, 3], [3, 0], // Bottom square
        [4, 5], [5, 6], [6, 7], [7, 4], // Top square
        [0, 4], [1, 5], [2, 6], [3, 7]  // Vertical pillars
      ];

      ctx.strokeStyle = "rgba(110, 118, 129, 0.5)";
      ctx.lineWidth = 1.5 * dpr;
      ctx.beginPath();
      for (const [a, b] of edges) {
        if (projCorners[a] && projCorners[b]) {
          ctx.moveTo(projCorners[a][0], projCorners[a][1]);
          ctx.lineTo(projCorners[b][0], projCorners[b][1]);
        }
      }
      ctx.stroke();

      // Axis labels (k points upward)
      const topLabel = this.projectPoint([0.5, 0.5, 1.08], w, h);
      if (topLabel) {
        ctx.font = `${Math.round(11 * dpr)}px ui-monospace, "SF Mono", monospace`;
        ctx.fillStyle = "#e6edf3";
        ctx.textAlign = "center";
        ctx.fillText("+k (elevation / depth axis)", topLabel[0], topLabel[1]);
      }
    }
  }

  return {
    LAYERS,
    Stage2Cutaway
  };
});
