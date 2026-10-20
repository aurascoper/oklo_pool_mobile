/* Stage 3: Revision-0 Field Inspection Module */
/* WebGL2 volume raymarching with clean 2D orthogonal slice fallback */
/* Strict requirement: zero em-dashes and zero en-dashes */

(function (root, factory) {
  if (typeof module === "object" && module.exports) {
    module.exports = factory(require("./spatial_engine.js"), require("./provenance_hud.js"));
  } else {
    root.Stage3Fields = factory(root.SpatialEngine, root.ProvenanceHUD);
  }
})(typeof self !== "undefined" ? self : this, function (SpatialEngine, ProvenanceHUD) {
  "use strict";

  const N = 32;
  const VOX = N * N * N;
  const FRAMES = 60;
  const PERIOD_MIN = 180;

  const FIELDS = [
    {
      id: "temperature",
      name: "Temperature",
      unit: "deg C as labelled; coefficients underived",
      cmap: "inferno",
      digits: 2,
      invert: false,
      limit: "Not a thermal-hydraulic solution. Algebraic proxy function; must not be used to state historical temperatures at Oklo."
    },
    {
      id: "hydration",
      name: "Hydration",
      unit: "pore saturation, dimensionless [0.02, 1.0]",
      cmap: "blues",
      digits: 3,
      invert: true,
      limit: "A prescribed drying shape, not a transport solution. The 3D view highlights the driest desiccated voxels."
    },
    {
      id: "h2_concentration",
      name: "H2 Index",
      unit: "index; labelled mmol/L with no dose basis",
      cmap: "viridis",
      digits: 3,
      invert: false,
      limit: "A G-value alone cannot give a concentration. Never convert this field into a dose or a chemical concentration."
    },
    {
      id: "u235_ratio",
      name: "235U/238U Ratio",
      unit: "atom ratio; natural baseline 0.00725",
      cmap: "cividis",
      digits: 5,
      invert: true,
      limit: "An atom ratio, not interchangeable with 0.7204 atom percent. Depleted core highlights fission consumption. Does not vary over cycle."
    },
    {
      id: "polymer_yield",
      name: "Polymer Index",
      unit: "index; labelled umol/L with no kinetic basis",
      cmap: "magma",
      digits: 3,
      invert: false,
      limit: "Not evidence of polymerization at Oklo or anywhere. Mass balance closes by construction (Finding R0-8)."
    },
    {
      id: "compartments",
      name: "Compartments",
      unit: "zone label 0 to 4",
      cmap: null,
      digits: 0,
      invert: false,
      limit: "A prescribed geometry, not surveyed subsurface geology. Zone 4 is a label applied where desiccation exceeds 0.4; no steam dynamics modelled."
    }
  ];

  const ZONES = [
    ["Sandstone", "#d8c79f", 0],
    ["Reactor Core", "#6b2d82", 230],
    ["Clay and Phosphate Halo", "#8ea86c", 40],
    ["Fracture Conduit", "#3c7fc0", 170],
    ["Steam Chimney", "#f0eee8", 200]
  ];

  const COLOR_STOPS = {
    inferno: ["#000004", "#1b0c41", "#4a0c6b", "#781c6d", "#a52c60", "#cf4446", "#ed6925", "#fb9b06", "#f7d13d", "#fcffa4"],
    blues: ["#f7fbff", "#deebf7", "#c6dbef", "#9ecae1", "#6baed6", "#4292c6", "#2171b5", "#08519c", "#08306b"],
    viridis: ["#440154", "#472d7b", "#3b528b", "#2c728e", "#21918c", "#28ae80", "#5ec962", "#addc30", "#fde725"],
    cividis: ["#00224e", "#123570", "#3b496c", "#575d6d", "#707173", "#8a8678", "#a59c74", "#c3b369", "#e1cc55", "#fee838"],
    magma: ["#000004", "#180f3d", "#440f76", "#721f81", "#9e2f7f", "#cd4071", "#f1605d", "#fd9668", "#feca8d", "#fcfdbf"]
  };

  function hexToRgb(h) {
    const v = parseInt(h.slice(1), 16);
    return [v >> 16, (v >> 8) & 255, v & 255];
  }

  const RGB_STOPS = Object.fromEntries(
    Object.entries(COLOR_STOPS).map(([k, v]) => [k, v.map(hexToRgb)])
  );

  function ramp(name, t) {
    const s = RGB_STOPS[name];
    if (!s) return [128, 128, 128];
    const x = Math.min(1.0, Math.max(0.0, t)) * (s.length - 1);
    const i = Math.min(s.length - 2, Math.floor(x));
    const f = x - i;
    return s[i].map((c, n) => Math.round(c + (s[i + 1][n] - c) * f));
  }

  function colorForValue(fieldIdx, val, range) {
    const f = FIELDS[fieldIdx];
    if (!f.cmap) {
      const z = ZONES[Math.round(val)] || ZONES[0];
      return hexToRgb(z[1]);
    }
    const lo = range[0];
    const hi = range[1];
    const span = hi - lo || 1.0;
    return ramp(f.cmap, (val - lo) / span);
  }

  // Pure JavaScript Bit-Identical Revision-0 Evaluation
  function pulseTerm(p, b) {
    if (p < b) return Math.sin((p / b) * Math.PI);
    return Math.max(0.0, Math.exp(-8.0 * (p - b)));
  }

  function desiccationTerm(p) {
    if (p >= 0.10 && p <= 0.35) return Math.sin(((p - 0.10) / 0.25) * Math.PI);
    return 0.0;
  }

  function computeFieldNative(fieldId, phase, nullScenario, b, outBuffer) {
    const pulseFactor = nullScenario ? 0.0 : pulseTerm(phase, b);
    const desic = nullScenario ? 0.0 : desiccationTerm(phase);
    const nz = N;
    let idx = 0;

    for (let k = 0; k < N; k++) {
      const depth = k / nz;
      for (let j = 0; j < N; j++) {
        for (let i = 0; i < N; i++) {
          const cx = (N - 1) / 2;
          const cy = (N - 1) / 2;
          const cz_core = nz * 0.35;
          const z_norm = (k - cz_core) / (nz * 0.35);
          const y_norm = (j - cy) / (N * 0.40);
          const x_norm = (i - cx) / (N * 0.40);
          const r_core = Math.hypot(x_norm, y_norm, z_norm);

          const dx_frac = (i - cx) / (N * 0.12);
          const dy_frac = (j - cy) / (N * 0.12);
          const r_fracture = Math.hypot(dx_frac, dy_frac);
          const f = k >= cz_core ? Math.exp(-0.5 * r_fracture * r_fracture) : 0.0;
          const c = Math.exp(-0.5 * Math.pow(r_core / 0.6, 2));
          const h = Math.max(0.0, Math.exp(-0.5 * Math.pow((r_core - 1.1) / 0.35, 2)));

          let val = 0.0;
          switch (fieldId) {
            case 0: { // Temperature
              const base_temp = 25.0 + 35.0 * depth;
              if (nullScenario) {
                val = base_temp + 30.0 * c;
              } else {
                val = base_temp + 320.0 * pulseFactor * c + 180.0 * pulseFactor * f + 40.0 * h * (1.0 - pulseFactor * 0.5);
              }
              break;
            }
            case 1: { // Hydration
              const base_hyd = 0.85 - 0.10 * depth;
              if (nullScenario) {
                val = base_hyd;
              } else {
                val = Math.min(1.0, Math.max(0.02, base_hyd - desic * (0.80 * f + 0.65 * c) + 0.08 * h));
              }
              break;
            }
            case 2: { // H2 index
              if (nullScenario) {
                val = 0.001;
              } else {
                const water = Math.max(0.05, 1.0 - desic * 0.7);
                const rad = c + 0.3 * h;
                val = 0.40 * 12.5 * rad * water + 1.8 * f * (1.0 - desic);
              }
              break;
            }
            case 3: // U235
              val = 0.00725 - 0.00305 * c;
              break;
            case 4: // Polymer
              if (nullScenario) {
                val = 0.0;
              } else {
                const cat = 0.7 * h + 0.5 * f;
                const eff = 1.0 + 0.5 * Math.sin(phase * 2.0 * Math.PI);
                val = 8.5 * cat * eff;
              }
              break;
            case 5: // Compartments
              if (c > 0.45) val = 1.0;
              else if (f > 0.35) val = desic > 0.4 ? 4.0 : 3.0;
              else if (h > 0.35) val = 2.0;
              else val = 0.0;
              break;
          }
          outBuffer[idx++] = Math.fround(val);
        }
      }
    }
  }

  class Stage3Fields {
    constructor(elements = {}, options = {}) {
      this.glCanvas = elements.glCanvas;
      this.planCanvas = elements.planCanvas;
      this.sectionCanvas = elements.sectionCanvas;
      this.boxCanvas = elements.boxCanvas;
      this.readoutEl = elements.readoutEl;
      this.limitsEl = elements.limitsEl;
      this.options = options;

      this.activeField = 0;
      this.activeFrame = 9; // Default frame 9 (phase 0.15)
      this.b = 0.17;
      this.nullScenario = false;
      this.viewMode = "3d"; // "3d" or "slices"
      this.k = 12; // Plan slice
      this.j = 15; // Section slice
      this.opacity = 1.0;

      // 3D camera
      this.yaw = -0.85;
      this.pitch = 0.42;
      this.dist = 2.7;
      this.isDragging = false;
      this.lastMouse = [0, 0];

      // WebGL2 context
      this.gl = null;
      this.glProg = null;
      this.volTex = null;
      this.cmapTex = null;
      this.uni = {};

      this.currentBuffer = new Float32Array(VOX);
      this.currentRange = [0, 1];
      this.hasWebGL2 = false;

      this.initWebGL();
      this.initEvents();
      this.updateField();
    }

    initWebGL() {
      if (!this.glCanvas) return;
      try {
        const gl = this.glCanvas.getContext("webgl2", { premultipliedAlpha: true, antialias: false });
        if (!gl) {
          this.hasWebGL2 = false;
          return;
        }

        const VS = `#version 300 es
        in vec2 pos; out vec2 uv;
        void main(){ uv = pos * 0.5 + 0.5; gl_Position = vec4(pos, 0.0, 1.0); }`;

        const FS = `#version 300 es
        precision highp float; precision highp sampler3D;
        uniform sampler3D vol; uniform sampler2D cmap;
        uniform vec3 camPos, camRight, camUp, camFwd;
        uniform float aspect, opacity; uniform int invertA, categorical;
        in vec2 uv; out vec4 frag;
        void main(){
          vec2 p = uv * 2.0 - 1.0;
          vec3 d = normalize(camFwd * 1.8 + camRight * p.x * aspect + camUp * p.y);
          vec3 inv = 1.0 / d, t0 = -camPos * inv, t1 = (vec3(1.0) - camPos) * inv;
          vec3 tn = min(t0, t1), tx = max(t0, t1);
          float a = max(max(tn.x, tn.y), tn.z), b = min(min(tx.x, tx.y), tx.z);
          if (a > b || b < 0.0) { frag = vec4(0.0); return; }
          a = max(a, 0.0);
          const int STEPS = 192; float dt = 1.7320508 / float(STEPS);
          vec4 acc = vec4(0.0);
          for (int s = 0; s < STEPS; s++) {
            float t = a + (float(s) + 0.5) * dt; if (t > b) break;
            vec3 q = camPos + d * t;
            float v = texture(vol, q).r;
            vec4 c;
            if (categorical == 1) { c = texture(cmap, vec2((v * 255.0 + 0.5) / 256.0, 0.5)); }
            else { c = vec4(texture(cmap, vec2(v, 0.5)).rgb, smoothstep(0.25, 1.0, invertA == 1 ? 1.0 - v : v)); }
            float al = clamp(c.a * opacity * dt * 22.0, 0.0, 1.0);
            acc.rgb += (1.0 - acc.a) * al * c.rgb; acc.a += (1.0 - acc.a) * al;
            if (acc.a > 0.97) break;
          }
          frag = acc;
        }`;

        const sh = (type, src) => {
          const s = gl.createShader(type);
          gl.shaderSource(s, src);
          gl.compileShader(s);
          if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(s));
          return s;
        };

        const prog = gl.createProgram();
        gl.attachShader(prog, sh(gl.VERTEX_SHADER, VS));
        gl.attachShader(prog, sh(gl.FRAGMENT_SHADER, FS));
        gl.linkProgram(prog);
        if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(prog));

        gl.useProgram(prog);
        const buf = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buf);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
        const loc = gl.getAttribLocation(prog, "pos");
        gl.enableVertexAttribArray(loc);
        gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);

        for (const n of ["vol", "cmap", "camPos", "camRight", "camUp", "camFwd", "aspect", "opacity", "invertA", "categorical"]) {
          this.uni[n] = gl.getUniformLocation(prog, n);
        }

        this.volTex = gl.createTexture();
        this.cmapTex = gl.createTexture();
        gl.uniform1i(this.uni.vol, 0);
        gl.uniform1i(this.uni.cmap, 1);
        gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);

        this.gl = gl;
        this.glProg = prog;
        this.hasWebGL2 = true;
      } catch (e) {
        console.warn("Stage3Fields: WebGL2 initialization failed; falling back to 2D orthogonal slices.", e);
        this.hasWebGL2 = false;
      }
    }

    initEvents() {
      if (this.glCanvas) {
        const c = this.glCanvas;
        c.addEventListener("pointerdown", (e) => {
          c.setPointerCapture(e.pointerId);
          this.isDragging = true;
          this.lastMouse = [e.clientX, e.clientY];
        });
        c.addEventListener("pointermove", (e) => {
          if (!this.isDragging) return;
          const dx = e.clientX - this.lastMouse[0];
          const dy = e.clientY - this.lastMouse[1];
          this.yaw -= dx * 0.01;
          this.pitch = Math.max(-1.45, Math.min(1.45, this.pitch + dy * 0.01));
          this.lastMouse = [e.clientX, e.clientY];
          this.draw3d();
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
          this.dist = Math.max(1.6, Math.min(5.0, this.dist * Math.exp(e.deltaY * 0.001)));
          this.draw3d();
        }, { passive: false });
      }

      // Slice crosshair hover
      const setupSliceHover = (canvas, isPlan) => {
        if (!canvas) return;
        canvas.addEventListener("pointermove", (e) => {
          const r = canvas.getBoundingClientRect();
          const ci = Math.min(N - 1, Math.max(0, Math.floor((e.clientX - r.left) / r.width * N)));
          const cj = N - 1 - Math.min(N - 1, Math.max(0, Math.floor((e.clientY - r.top) / r.height * N)));
          const [i, j, k] = isPlan ? [ci, cj, this.k] : [ci, this.j, cj];
          const v = this.currentBuffer[k * N * N + j * N + i];
          const f = FIELDS[this.activeField];
          const valStr = f.cmap ? v.toFixed(f.digits) : `${v} (${ZONES[Math.round(v)] ? ZONES[Math.round(v)][0] : "Zone"})`;
          if (this.readoutEl) {
            this.readoutEl.textContent = `Voxel: i=${i}, j=${j}, k=${k} | Metric: x=${((i + 0.5) * 0.5).toFixed(1)}m, y=${((j + 0.5) * 0.5).toFixed(1)}m, z=${((k + 0.5) * 0.5).toFixed(1)}m | Value: ${valStr}`;
          }
        });
      };

      setupSliceHover(this.planCanvas, true);
      setupSliceHover(this.sectionCanvas, false);

      window.addEventListener("resize", () => {
        this.render();
      });
    }

    setField(fieldIdx) {
      if (fieldIdx >= 0 && fieldIdx < FIELDS.length) {
        this.activeField = fieldIdx;
        this.updateField();
      }
    }

    setFrame(frameIdx) {
      this.activeFrame = Math.max(0, Math.min(FRAMES - 1, frameIdx));
      this.updateField();
    }

    setPulseBoundary(bVal) {
      this.b = bVal;
      this.updateField();
    }

    setNullScenario(isNull) {
      this.nullScenario = isNull;
      this.updateField();
    }

    setSlices(kVal, jVal) {
      if (kVal !== undefined) this.k = Math.max(0, Math.min(N - 1, kVal));
      if (jVal !== undefined) this.j = Math.max(0, Math.min(N - 1, jVal));
      this.drawSlices();
    }

    updateField() {
      const phase = this.activeFrame / FRAMES;
      computeFieldNative(this.activeField, phase, this.nullScenario, this.b, this.currentBuffer);

      let mn = Infinity;
      let mx = -Infinity;
      for (let n = 0; n < VOX; n++) {
        const v = this.currentBuffer[n];
        if (v < mn) mn = v;
        if (v > mx) mx = v;
      }
      this.currentRange = [mn, mx];

      if (this.limitsEl) {
        this.limitsEl.textContent = FIELDS[this.activeField].limit;
      }

      this.uploadVolume();
      this.render();
    }

    uploadVolume() {
      if (!this.hasWebGL2 || !this.gl) return;
      const gl = this.gl;
      const f = FIELDS[this.activeField];
      const u8 = new Uint8Array(VOX);

      if (!f.cmap) {
        for (let n = 0; n < VOX; n++) u8[n] = Math.round(this.currentBuffer[n]);
      } else {
        const s = 255.0 / ((this.currentRange[1] - this.currentRange[0]) || 1.0);
        for (let n = 0; n < VOX; n++) {
          u8[n] = Math.round((this.currentBuffer[n] - this.currentRange[0]) * s);
        }
      }

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_3D, this.volTex);
      gl.texImage3D(gl.TEXTURE_3D, 0, gl.R8, N, N, N, 0, gl.RED, gl.UNSIGNED_BYTE, u8);
      const filt = f.cmap ? gl.LINEAR : gl.NEAREST;
      gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_MIN_FILTER, filt);
      gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_MAG_FILTER, filt);
      gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_WRAP_R, gl.CLAMP_TO_EDGE);

      const cm = new Uint8Array(256 * 4);
      for (let x = 0; x < 256; x++) {
        let rgb, a = 255;
        if (!f.cmap) {
          const z = ZONES[x];
          rgb = z ? hexToRgb(z[1]) : [0, 0, 0];
          a = z ? z[2] : 0;
        } else {
          rgb = ramp(f.cmap, x / 255.0);
        }
        cm.set([...rgb, a], x * 4);
      }

      gl.activeTexture(gl.TEXTURE1);
      gl.bindTexture(gl.TEXTURE_2D, this.cmapTex);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, 256, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, cm);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

      gl.uniform1i(this.uni.invertA, f.invert ? 1 : 0);
      gl.uniform1i(this.uni.categorical, f.cmap ? 0 : 1);
    }

    draw3d() {
      if (!this.hasWebGL2 || !this.gl || !this.glCanvas) return;
      const gl = this.gl;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const rect = this.glCanvas.getBoundingClientRect();
      const w = Math.max(1, Math.round(rect.width * dpr));
      const h = Math.max(1, Math.round(rect.height * dpr));

      if (this.glCanvas.width !== w || this.glCanvas.height !== h) {
        this.glCanvas.width = w;
        this.glCanvas.height = h;
      }

      gl.viewport(0, 0, w, h);
      gl.clearColor(0, 0, 0, 0);
      gl.clear(gl.COLOR_BUFFER_BIT);

      const cam = SpatialEngine.computeCameraBasis(this.yaw, this.pitch, this.dist, [0.5, 0.5, 0.5]);
      gl.uniform3fv(this.uni.camPos, cam.pos);
      gl.uniform3fv(this.uni.camRight, cam.right);
      gl.uniform3fv(this.uni.camUp, cam.up);
      gl.uniform3fv(this.uni.camFwd, cam.fwd);
      gl.uniform1f(this.uni.aspect, w / h);
      gl.uniform1f(this.uni.opacity, this.opacity);

      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

      this.drawBoxOverlay(cam, w, h);
    }

    drawBoxOverlay(cam, w, h) {
      if (!this.boxCanvas) return;
      const c = this.boxCanvas;
      if (c.width !== w || c.height !== h) {
        c.width = w;
        c.height = h;
      }
      const ctx = c.getContext("2d");
      ctx.clearRect(0, 0, w, h);

      const proj = (X) => {
        const r = SpatialEngine.vSub(X, cam.pos);
        const z = SpatialEngine.vDot(cam.fwd, r);
        const px = (SpatialEngine.vDot(cam.right, r) / z) * 1.8 / (w / h);
        const py = (SpatialEngine.vDot(cam.up, r) / z) * 1.8;
        return [(px * 0.5 + 0.5) * w, (1.0 - (py * 0.5 + 0.5)) * h];
      };

      const P = [];
      for (let n = 0; n < 8; n++) P.push(proj([n & 1, (n >> 1) & 1, (n >> 2) & 1]));
      const E = [[0, 1], [2, 3], [4, 5], [6, 7], [0, 2], [1, 3], [4, 6], [5, 7], [0, 4], [1, 5], [2, 6], [3, 7]];
      ctx.strokeStyle = "rgba(157, 167, 179, 0.5)";
      ctx.lineWidth = 1;
      ctx.beginPath();
      for (const [a, b] of E) {
        ctx.moveTo(...P[a]);
        ctx.lineTo(...P[b]);
      }
      ctx.stroke();

      const top = proj([0, 0, 1.08]);
      ctx.fillStyle = "rgba(157, 167, 179, 0.9)";
      ctx.font = `${Math.round(11 * (w / c.getBoundingClientRect().width))}px ui-monospace, monospace`;
      ctx.fillText("+k", top[0] - 4, top[1]);
    }

    drawSlices() {
      // Draw Plan (X-Y at k)
      if (this.planCanvas) {
        const c = this.planCanvas;
        const ctx = c.getContext("2d");
        const img = ctx.createImageData(N, N);
        const k = this.k;

        for (let r = 0; r < N; r++) {
          const j = N - 1 - r;
          for (let col = 0; col < N; col++) {
            const i = col;
            const v = this.currentBuffer[k * N * N + j * N + i];
            const [R, G, B] = colorForValue(this.activeField, v, this.currentRange);
            const o = (r * N + col) * 4;
            img.data[o] = R;
            img.data[o + 1] = G;
            img.data[o + 2] = B;
            img.data[o + 3] = 255;
          }
        }

        // Render to offscreen canvas and draw scaled to canvas
        const off = document.createElement("canvas");
        off.width = N;
        off.height = N;
        off.getContext("2d").putImageData(img, 0, 0);

        ctx.imageSmoothingEnabled = false;
        ctx.drawImage(off, 0, 0, c.width, c.height);

        // Section line indicator (horizontal line at row j)
        const lineY = (N - 1 - this.j + 0.5) * c.height / N;
        ctx.strokeStyle = "rgba(255, 255, 255, 0.85)";
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(0, lineY);
        ctx.lineTo(c.width, lineY);
        ctx.stroke();
      }

      // Draw Section (X-k at j)
      if (this.sectionCanvas) {
        const c = this.sectionCanvas;
        const ctx = c.getContext("2d");
        const img = ctx.createImageData(N, N);
        const j = this.j;

        for (let r = 0; r < N; r++) {
          const k = N - 1 - r;
          for (let col = 0; col < N; col++) {
            const i = col;
            const v = this.currentBuffer[k * N * N + j * N + i];
            const [R, G, B] = colorForValue(this.activeField, v, this.currentRange);
            const o = (r * N + col) * 4;
            img.data[o] = R;
            img.data[o + 1] = G;
            img.data[o + 2] = B;
            img.data[o + 3] = 255;
          }
        }

        const off = document.createElement("canvas");
        off.width = N;
        off.height = N;
        off.getContext("2d").putImageData(img, 0, 0);

        ctx.imageSmoothingEnabled = false;
        ctx.drawImage(off, 0, 0, c.width, c.height);

        // Plan line indicator (horizontal line at slice k)
        const lineY = (N - 1 - this.k + 0.5) * c.height / N;
        ctx.strokeStyle = "rgba(255, 255, 255, 0.85)";
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(0, lineY);
        ctx.lineTo(c.width, lineY);
        ctx.stroke();
      }
    }

    render() {
      this.draw3d();
      this.drawSlices();
    }
  }

  return {
    FIELDS,
    ZONES,
    COLOR_STOPS,
    Stage3Fields,
    computeFieldNative
  };
});
