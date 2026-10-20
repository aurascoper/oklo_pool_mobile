/* Stage 1: Interactive Planetary Globe & Deep-Time Timeline */
/* Strict requirement: zero em-dashes and zero en-dashes */

(function (root, factory) {
  if (typeof module === "object" && module.exports) {
    module.exports = factory(require("./spatial_engine.js"), require("./provenance_hud.js"));
  } else {
    root.Stage1Globe = factory(root.SpatialEngine, root.ProvenanceHUD);
  }
})(typeof self !== "undefined" ? self : this, function (SpatialEngine, ProvenanceHUD) {
  "use strict";

  const SITES = [
    {
      id: "oklo",
      name: "Oklo Reactor Zone 2",
      region: "Haut-Ogooue, Gabon",
      lat: -1.4000,
      lon: 13.1833,
      coords_str: "1 deg 24' 00\" S, 13 deg 11' 00\" E",
      source: "Gauthier-Lafaye, Holliger and Blanc 1996, GCA 60(23), page 4831",
      evidence_class: "geographic_observation_or_locator",
      is_primary: true,
      description: "Primary site of the natural nuclear fission reactors that operated approximately 1.95 Ga ago in the Franceville Basin."
    },
    {
      id: "dallol",
      name: "Dallol Hydrothermal System",
      region: "Danakil Depression, Ethiopia",
      lat: 14.2417,
      lon: 40.3000,
      coords_str: "14 deg 14' 30\" N, 40 deg 18' 00\" E",
      source: "Cavalazzi et al. 2019, Astrobiology 19(9), page 1065",
      evidence_class: "geographic_observation_or_locator",
      is_primary: false,
      description: "Extreme poly-extreme hypersaline hydrothermal brine analog."
    },
    {
      id: "yellowstone",
      name: "Yellowstone (Norris Geyser)",
      region: "Wyoming, USA",
      lat: 44.7283,
      lon: -110.7042,
      coords_str: "44 deg 43' 42\" N, 110 deg 42' 15\" W",
      source: "USGS Yellowstone Volcano Observatory / Damer and Deamer 2015",
      evidence_class: "geographic_observation_or_locator",
      is_primary: false,
      description: "Terrestrial hot spring wet-dry cycling scenario analog."
    },
    {
      id: "lost_city",
      name: "Lost City Hydrothermal Field",
      region: "Mid-Atlantic Ridge",
      lat: 30.1167,
      lon: -42.1167,
      coords_str: "30 deg 07' 00\" N, 42 deg 07' 00\" W",
      source: "Kelley et al. 2001, Nature 412, page 145",
      evidence_class: "geographic_observation_or_locator",
      is_primary: false,
      description: "Low-temperature serpentinization-driven alkaline hydrothermal vent analog."
    }
  ];

  // Simplified continent coastlines representation (latitude, longitude waypoints)
  const CONTINENTS = [
    // Africa
    [[37, 10], [32, 25], [31, 32], [22, 37], [12, 44], [12, 51], [0, 42], [-10, 40], [-25, 33], [-34, 18], [-20, 12], [-5, 12], [4, 9], [5, -3], [14, -17], [21, -17], [28, -10], [35, -6], [37, 10]],
    // Eurasia
    [[36, -6], [43, -9], [48, -5], [54, 5], [58, 5], [62, 5], [71, 28], [70, 60], [77, 105], [70, 179], [60, 165], [45, 135], [30, 122], [22, 114], [10, 105], [1, 103], [8, 98], [15, 80], [25, 62], [30, 48], [40, 26], [36, -6]],
    // North America
    [[71, -156], [70, -130], [60, -90], [55, -60], [45, -65], [30, -80], [25, -80], [20, -87], [15, -92], [20, -105], [32, -117], [48, -125], [60, -145], [65, -168], [71, -156]],
    // South America
    [[12, -72], [10, -60], [-5, -35], [-23, -42], [-40, -62], [-54, -68], [-45, -75], [-20, -70], [-5, -80], [8, -77], [12, -72]],
    // Australia
    [[-12, 131], [-15, 145], [-28, 153], [-38, 145], [-35, 115], [-22, 114], [-15, 124], [-12, 131]]
  ];

  class Stage1Globe {
    constructor(canvasEl, timelineCanvasEl, options = {}) {
      this.canvas = canvasEl;
      this.timelineCanvas = timelineCanvasEl;
      this.ctx = canvasEl.getContext("2d");
      this.timelineCtx = timelineCanvasEl ? timelineCanvasEl.getContext("2d") : null;
      this.onSelectOklo = options.onSelectOklo || null;
      this.onSiteChange = options.onSiteChange || null;

      this.selectedSite = SITES[0];
      this.hoveredSite = null;

      // Camera orientation (yaw, pitch, distance)
      // Initial focus on Africa (approx yaw 0.2 rad, pitch 0.0 rad)
      this.yaw = 0.25;
      this.pitch = 0.05;
      this.dist = 2.4; // 1.5 to 5.0
      this.targetYaw = this.yaw;
      this.targetPitch = this.pitch;
      this.targetDist = this.dist;

      this.isDragging = false;
      this.lastMouse = [0, 0];
      this.timelineData = null;

      this.initEvents();
      this.initTimeline();
    }

    async initTimeline() {
      try {
        const res = await fetch("data/paleo_intervals_v1.json");
        if (res.ok) {
          this.timelineData = await res.json();
        }
      } catch (_) {
        try {
          const res2 = await fetch("../data/timeline/paleo_intervals_v1.json");
          if (res2.ok) this.timelineData = await res2.json();
        } catch (e) {
          console.warn("Stage1Globe: could not load timeline data file.");
        }
      }
      this.renderTimeline();
    }

    initEvents() {
      const c = this.canvas;
      c.addEventListener("pointerdown", (e) => {
        c.setPointerCapture(e.pointerId);
        this.isDragging = true;
        this.lastMouse = [e.clientX, e.clientY];
      });

      c.addEventListener("pointermove", (e) => {
        const rect = c.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;

        if (this.isDragging) {
          const dx = e.clientX - this.lastMouse[0];
          const dy = e.clientY - this.lastMouse[1];
          this.yaw += dx * 0.008;
          this.pitch = Math.max(-1.45, Math.min(1.45, this.pitch + dy * 0.008));
          this.targetYaw = this.yaw;
          this.targetPitch = this.pitch;
          this.lastMouse = [e.clientX, e.clientY];
          this.render();
          return;
        }

        // Hit-test site markers
        const hit = this.hitTestSite(mx, my);
        if (hit !== this.hoveredSite) {
          this.hoveredSite = hit;
          c.style.cursor = hit ? "pointer" : "grab";
          this.render();
        }
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
        this.dist = Math.max(1.5, Math.min(5.0, this.dist * Math.exp(e.deltaY * 0.001)));
        this.targetDist = this.dist;
        this.render();
      }, { passive: false });

      c.addEventListener("click", (e) => {
        const rect = c.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;
        const hit = this.hitTestSite(mx, my);
        if (hit) {
          this.selectSite(hit);
        }
      });

      window.addEventListener("resize", () => {
        this.render();
        this.renderTimeline();
      });
    }

    selectSite(site) {
      this.selectedSite = site;
      // Animate camera to center on selected site
      const phi = site.lat * (Math.PI / 180.0);
      const lambda = site.lon * (Math.PI / 180.0);
      this.targetPitch = phi;
      this.targetYaw = -lambda + Math.PI * 0.5;
      this.animateToTarget();

      if (this.onSiteChange) {
        this.onSiteChange(site);
      }
      this.render();
    }

    animateToTarget() {
      const step = () => {
        const dy = this.targetYaw - this.yaw;
        const dp = this.targetPitch - this.pitch;
        if (Math.abs(dy) > 0.001 || Math.abs(dp) > 0.001) {
          this.yaw += dy * 0.15;
          this.pitch += dp * 0.15;
          this.render();
          requestAnimationFrame(step);
        } else {
          this.yaw = this.targetYaw;
          this.pitch = this.targetPitch;
          this.render();
        }
      };
      requestAnimationFrame(step);
    }

    projectPoint(p, w, h) {
      // Camera basis from yaw, pitch, distance
      const cam = SpatialEngine.computeCameraBasis(this.yaw, this.pitch, this.dist, [0, 0, 0]);
      const r = SpatialEngine.vSub(p, cam.pos);
      const z = SpatialEngine.vDot(cam.fwd, r);
      if (z <= 0.01) return null; // Behind camera

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

    hitTestSite(mx, my) {
      const w = this.canvas.width;
      const h = this.canvas.height;
      const dpr = window.devicePixelRatio || 1;
      const clickX = mx * dpr;
      const clickY = my * dpr;

      let closest = null;
      let minD = 24 * dpr;

      for (const site of SITES) {
        const [x, y, z] = SpatialEngine.wgs84ToCartesian(site.lat, site.lon, 1.0);
        // Check if marker is on visible hemisphere facing camera
        const cam = SpatialEngine.computeCameraBasis(this.yaw, this.pitch, this.dist, [0, 0, 0]);
        const dot = SpatialEngine.vDot([x, y, z], cam.pos);
        if (dot < 0.2) continue; // Occluded by back of earth

        const proj = this.projectPoint([x, y, z], w, h);
        if (proj) {
          const d = Math.hypot(proj[0] - clickX, proj[1] - clickY);
          if (d < minD) {
            minD = d;
            closest = site;
          }
        }
      }
      return closest;
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

      // Deep space background gradient
      const bgGrad = ctx.createRadialGradient(w * 0.5, h * 0.5, 10, w * 0.5, h * 0.5, Math.max(w, h));
      bgGrad.addColorStop(0, "#0e1520");
      bgGrad.addColorStop(1, "#05070a");
      ctx.fillStyle = bgGrad;
      ctx.fillRect(0, 0, w, h);

      // Atmosphere outer glow
      const cam = SpatialEngine.computeCameraBasis(this.yaw, this.pitch, this.dist, [0, 0, 0]);
      const centerProj = this.projectPoint([0, 0, 0], w, h);
      const rimProj = this.projectPoint([1.0, 0, 0], w, h);
      const globeRadiusPx = centerProj && rimProj ? Math.hypot(rimProj[0] - centerProj[0], rimProj[1] - centerProj[1]) : (w * 0.35);

      if (centerProj) {
        const glow = ctx.createRadialGradient(centerProj[0], centerProj[1], globeRadiusPx * 0.95, centerProj[0], centerProj[1], globeRadiusPx * 1.15);
        glow.addColorStop(0, "rgba(56, 139, 253, 0.25)");
        glow.addColorStop(0.5, "rgba(56, 139, 253, 0.08)");
        glow.addColorStop(1, "rgba(56, 139, 253, 0.0)");
        ctx.fillStyle = glow;
        ctx.beginPath();
        ctx.arc(centerProj[0], centerProj[1], globeRadiusPx * 1.15, 0, Math.PI * 2);
        ctx.fill();

        // Earth globe base sphere shading
        const sphereGrad = ctx.createRadialGradient(
          centerProj[0] - globeRadiusPx * 0.3,
          centerProj[1] - globeRadiusPx * 0.3,
          globeRadiusPx * 0.1,
          centerProj[0],
          centerProj[1],
          globeRadiusPx
        );
        sphereGrad.addColorStop(0, "#1c2b3d");
        sphereGrad.addColorStop(0.7, "#0c1522");
        sphereGrad.addColorStop(1, "#04070d");
        ctx.fillStyle = sphereGrad;
        ctx.beginPath();
        ctx.arc(centerProj[0], centerProj[1], globeRadiusPx, 0, Math.PI * 2);
        ctx.fill();

        ctx.strokeStyle = "rgba(56, 139, 253, 0.3)";
        ctx.lineWidth = 1 * dpr;
        ctx.stroke();
      }

      // Parallels (latitude lines)
      ctx.strokeStyle = "rgba(100, 140, 180, 0.18)";
      ctx.lineWidth = 1 * dpr;
      for (const lat of [-60, -30, 0, 30, 60]) {
        ctx.beginPath();
        let first = true;
        for (let lon = -180; lon <= 180; lon += 5) {
          const p = SpatialEngine.wgs84ToCartesian(lat, lon, 1.0);
          const dot = SpatialEngine.vDot(p, cam.pos);
          if (dot <= 0.05) {
            first = true;
            continue;
          }
          const proj = this.projectPoint(p, w, h);
          if (proj) {
            if (first) { ctx.moveTo(proj[0], proj[1]); first = false; }
            else { ctx.lineTo(proj[0], proj[1]); }
          }
        }
        ctx.stroke();
      }

      // Meridians (longitude lines)
      for (let lon = -180; lon < 180; lon += 30) {
        ctx.beginPath();
        let first = true;
        for (let lat = -85; lat <= 85; lat += 5) {
          const p = SpatialEngine.wgs84ToCartesian(lat, lon, 1.0);
          const dot = SpatialEngine.vDot(p, cam.pos);
          if (dot <= 0.05) {
            first = true;
            continue;
          }
          const proj = this.projectPoint(p, w, h);
          if (proj) {
            if (first) { ctx.moveTo(proj[0], proj[1]); first = false; }
            else { ctx.lineTo(proj[0], proj[1]); }
          }
        }
        ctx.stroke();
      }

      // Simplified continent outlines
      ctx.strokeStyle = "rgba(80, 180, 120, 0.45)";
      ctx.lineWidth = 1.5 * dpr;
      for (const poly of CONTINENTS) {
        ctx.beginPath();
        let first = true;
        for (const [lat, lon] of poly) {
          const p = SpatialEngine.wgs84ToCartesian(lat, lon, 1.002);
          const dot = SpatialEngine.vDot(p, cam.pos);
          if (dot <= 0.05) {
            first = true;
            continue;
          }
          const proj = this.projectPoint(p, w, h);
          if (proj) {
            if (first) { ctx.moveTo(proj[0], proj[1]); first = false; }
            else { ctx.lineTo(proj[0], proj[1]); }
          }
        }
        ctx.stroke();
      }

      // Render site markers
      for (const site of SITES) {
        const p = SpatialEngine.wgs84ToCartesian(site.lat, site.lon, 1.01);
        const dot = SpatialEngine.vDot(p, cam.pos);
        if (dot <= 0.1) continue; // Behind earth

        const proj = this.projectPoint(p, w, h);
        if (!proj) continue;

        const isSelected = this.selectedSite && this.selectedSite.id === site.id;
        const isHovered = this.hoveredSite && this.hoveredSite.id === site.id;

        // Pin outer halo
        ctx.fillStyle = site.is_primary ? "rgba(35, 134, 54, 0.35)" : "rgba(88, 166, 255, 0.3)";
        ctx.beginPath();
        const pinR = (site.is_primary ? 8 : 6) * dpr * (isSelected || isHovered ? 1.4 : 1.0);
        ctx.arc(proj[0], proj[1], pinR, 0, Math.PI * 2);
        ctx.fill();

        // Pin center dot
        ctx.fillStyle = site.is_primary ? "#3fb950" : "#58a6ff";
        ctx.beginPath();
        ctx.arc(proj[0], proj[1], (site.is_primary ? 4.5 : 3.5) * dpr, 0, Math.PI * 2);
        ctx.fill();
        ctx.strokeStyle = "#ffffff";
        ctx.lineWidth = 1 * dpr;
        ctx.stroke();

        // Marker label
        ctx.font = `${Math.round(11 * dpr)}px -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`;
        ctx.fillStyle = isSelected ? "#ffffff" : "rgba(230, 237, 243, 0.85)";
        ctx.textAlign = "left";
        ctx.fillText(site.name, proj[0] + 12 * dpr, proj[1] + 4 * dpr);
      }
    }

    renderTimeline() {
      if (!this.timelineCanvas || !this.timelineCtx) return;
      const c = this.timelineCanvas;
      const ctx = this.timelineCtx;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const rect = c.getBoundingClientRect();
      const w = Math.max(1, Math.round(rect.width * dpr));
      const h = Math.max(1, Math.round(rect.height * dpr));

      if (c.width !== w || c.height !== h) {
        c.width = w;
        c.height = h;
      }

      ctx.clearRect(0, 0, w, h);

      // Deep-time Proterozoic scale: 2.6 Ga (left) to 1.4 Ga (right)
      const tMin = 2.6; // Ga
      const tMax = 1.4; // Ga
      const xForGa = (ga) => {
        const frac = (tMin - ga) / (tMin - tMax);
        return Math.max(20 * dpr, Math.min(w - 20 * dpr, 20 * dpr + frac * (w - 40 * dpr)));
      };

      // Baseline timeline axis
      const yAxis = h * 0.72;
      ctx.strokeStyle = "rgba(110, 118, 129, 0.4)";
      ctx.lineWidth = 1.5 * dpr;
      ctx.beginPath();
      ctx.moveTo(20 * dpr, yAxis);
      ctx.lineTo(w - 20 * dpr, yAxis);
      ctx.stroke();

      // Axis tick marks (every 0.2 Ga)
      ctx.font = `${Math.round(10 * dpr)}px ui-monospace, "SF Mono", monospace`;
      ctx.fillStyle = "rgba(157, 167, 179, 0.9)";
      ctx.textAlign = "center";
      for (let ga = 2.6; ga >= 1.4; ga -= 0.2) {
        const tx = xForGa(ga);
        ctx.beginPath();
        ctx.moveTo(tx, yAxis - 4 * dpr);
        ctx.lineTo(tx, yAxis + 4 * dpr);
        ctx.stroke();
        ctx.fillText(`${ga.toFixed(1)} Ga`, tx, yAxis + 16 * dpr);
      }

      // Interval 1: Gumsley et al. 2017 GOE (2.460 to 2.060 Ga)
      const xGumsleyStart = xForGa(2.460);
      const xGumsleyEnd = xForGa(2.060);
      ctx.fillStyle = "rgba(35, 134, 54, 0.22)";
      ctx.strokeStyle = "rgba(63, 185, 80, 0.7)";
      ctx.lineWidth = 1 * dpr;
      ctx.fillRect(xGumsleyStart, yAxis - 46 * dpr, xGumsleyEnd - xGumsleyStart, 16 * dpr);
      ctx.strokeRect(xGumsleyStart, yAxis - 46 * dpr, xGumsleyEnd - xGumsleyStart, 16 * dpr);
      ctx.fillStyle = "#3fb950";
      ctx.textAlign = "left";
      ctx.fillText("GOE: Gumsley 2017 (2.46 to 2.06 Ga)", xGumsleyStart + 4 * dpr, yAxis - 34 * dpr);

      // Interval 2: Craig et al. 2023 GOE (2.43 to 2.22 Ga)
      const xCraigStart = xForGa(2.43);
      const xCraigEnd = xForGa(2.22);
      ctx.fillStyle = "rgba(210, 153, 34, 0.2)";
      ctx.strokeStyle = "rgba(227, 179, 65, 0.7)";
      ctx.fillRect(xCraigStart, yAxis - 26 * dpr, xCraigEnd - xCraigStart, 14 * dpr);
      ctx.strokeRect(xCraigStart, yAxis - 26 * dpr, xCraigEnd - xCraigStart, 14 * dpr);
      ctx.fillStyle = "#e3b341";
      ctx.fillText("GOE: Craig 2023 (2.43 to 2.22 Ga)", xCraigStart + 4 * dpr, yAxis - 16 * dpr);

      // Dated Marker: Oklo Natural Reactors (1.95 +/- 0.04 Ga)
      const xOklo = xForGa(1.95);
      const xOkloLo = xForGa(1.95 + 0.04);
      const xOkloHi = xForGa(1.95 - 0.04);

      // Error whiskers
      ctx.strokeStyle = "#f85149";
      ctx.lineWidth = 2 * dpr;
      ctx.beginPath();
      ctx.moveTo(xOkloLo, yAxis - 22 * dpr);
      ctx.lineTo(xOkloHi, yAxis - 22 * dpr);
      ctx.moveTo(xOkloLo, yAxis - 26 * dpr);
      ctx.lineTo(xOkloLo, yAxis - 18 * dpr);
      ctx.moveTo(xOkloHi, yAxis - 26 * dpr);
      ctx.lineTo(xOkloHi, yAxis - 18 * dpr);
      ctx.stroke();

      // Oklo central diamond marker
      ctx.fillStyle = "#f85149";
      ctx.beginPath();
      ctx.moveTo(xOklo, yAxis - 27 * dpr);
      ctx.lineTo(xOklo + 5 * dpr, yAxis - 22 * dpr);
      ctx.lineTo(xOklo, yAxis - 17 * dpr);
      ctx.lineTo(xOklo - 5 * dpr, yAxis - 22 * dpr);
      ctx.closePath();
      ctx.fill();

      ctx.fillStyle = "#f85149";
      ctx.textAlign = "center";
      ctx.fillText("Oklo 1.95 +/- 0.04 Ga", xOklo, yAxis - 32 * dpr);

      // Interval 3: Craig 2023 Eukaryogenesis (2.04 to 1.79 Ga core)
      const xEukStart = xForGa(2.04);
      const xEukEnd = xForGa(1.79);
      ctx.fillStyle = "rgba(147, 51, 234, 0.22)";
      ctx.strokeStyle = "rgba(168, 85, 247, 0.7)";
      ctx.fillRect(xEukStart, yAxis - 58 * dpr, xEukEnd - xEukStart, 10 * dpr);
      ctx.strokeRect(xEukStart, yAxis - 58 * dpr, xEukEnd - xEukStart, 10 * dpr);
      ctx.fillStyle = "#c084fc";
      ctx.textAlign = "left";
      ctx.fillText("Eukaryogenesis core: 2.04 to 1.79 Ga", xEukStart + 4 * dpr, yAxis - 62 * dpr);
    }
  }

  return {
    SITES,
    Stage1Globe
  };
});
