/* Spatial Engine for Planetary Explorer v2 */
/* Manages coordinate conversions, geodetic mappings, and camera orbit transitions */
/* Strict requirement: zero em-dashes and zero en-dashes */

(function (root, factory) {
  if (typeof module === "object" && module.exports) {
    module.exports = factory();
  } else {
    root.SpatialEngine = factory();
  }
})(typeof self !== "undefined" ? self : this, function () {
  "use strict";

  const DEG_TO_RAD = Math.PI / 180.0;
  const RAD_TO_DEG = 180.0 / Math.PI;

  /**
   * Convert WGS84 geographic coordinates (latitude, longitude) to 3D Cartesian coordinates on a sphere.
   * Frame: FRAME_SPHERICAL_UNIT.
   * +X points to (0 lat, 0 lon), +Y points to North Pole (+90 lat), -Z points to (0 lat, +90 lon).
   *
   * @param {number} lat Latitude in degrees [-90.0, 90.0]
   * @param {number} lon Longitude in degrees [-180.0, 180.0]
   * @param {number} radius Sphere radius (default 1.0)
   * @returns {[number, number, number]} [x, y, z] Cartesian vector
   */
  function wgs84ToCartesian(lat, lon, radius = 1.0) {
    if (typeof lat !== "number" || typeof lon !== "number" || !Number.isFinite(lat) || !Number.isFinite(lon)) {
      throw new Error("Invalid geographic coordinates: non-finite latitude or longitude");
    }
    if (lat < -90.0 || lat > 90.0) {
      throw new Error(`Latitude ${lat} out of valid bounds [-90.0, 90.0]`);
    }
    if (lon < -180.0 || lon > 180.0) {
      throw new Error(`Longitude ${lon} out of valid bounds [-180.0, 180.0]`);
    }

    const phi = lat * DEG_TO_RAD;
    const lambda = lon * DEG_TO_RAD;
    const cosPhi = Math.cos(phi);

    const x = radius * cosPhi * Math.cos(lambda);
    const y = radius * Math.sin(phi);
    const z = -radius * cosPhi * Math.sin(lambda);

    return [x, y, z];
  }

  /**
   * Convert Cartesian coordinates on a sphere back to WGS84 geographic coordinates.
   *
   * @param {number} x
   * @param {number} y
   * @param {number} z
   * @returns {{lat: number, lon: number}} Geographic coordinates in degrees
   */
  function cartesianToWgs84(x, y, z) {
    const len = Math.hypot(x, y, z);
    if (len < 1e-12) {
      return { lat: 0.0, lon: 0.0 };
    }
    const nx = x / len;
    const ny = Math.max(-1.0, Math.min(1.0, y / len));
    const nz = z / len;

    const lat = Math.asin(ny) * RAD_TO_DEG;
    const lon = Math.atan2(-nz, nx) * RAD_TO_DEG;

    return { lat, lon };
  }

  /**
   * Convert discrete fixture grid indices (i, j, k) to metric Cartesian coordinates in local block [0, 16m]^3.
   * Frame: FRAME_FIXTURE_GRID to FRAME_LOCAL_CARTESIAN.
   *
   * @param {number} i Grid column [0, 31]
   * @param {number} j Grid row [0, 31]
   * @param {number} k Grid vertical slice [0, 31]
   * @param {number} dx Voxel dimension in meters (default 0.5)
   * @returns {[number, number, number]} [x, y, z] in meters
   */
  function fixtureToCartesian(i, j, k, dx = 0.5) {
    return [(i + 0.5) * dx, (j + 0.5) * dx, (k + 0.5) * dx];
  }

  // Vector Math Utilities
  function vDot(a, b) {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  }

  function vCross(a, b) {
    return [
      a[1] * b[2] - a[2] * b[1],
      a[2] * b[0] - a[0] * b[2],
      a[0] * b[1] - a[1] * b[0]
    ];
  }

  function vNorm(v) {
    const l = Math.hypot(...v);
    return l > 1e-12 ? [v[0] / l, v[1] / l, v[2] / l] : [0, 0, 0];
  }

  function vSub(a, b) {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
  }

  function vAdd(a, b) {
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
  }

  function vScale(v, s) {
    return [v[0] * s, v[1] * s, v[2] * s];
  }

  function lerp(a, b, t) {
    return a + (b - a) * t;
  }

  function vLerp(a, b, t) {
    return [
      lerp(a[0], b[0], t),
      lerp(a[1], b[1], t),
      lerp(a[2], b[2], t)
    ];
  }

  /**
   * Spherical linear interpolation between two unit vectors.
   */
  function vSlerp(a, b, t) {
    let dot = Math.max(-1.0, Math.min(1.0, vDot(a, b)));
    if (dot > 0.9995) {
      return vNorm(vLerp(a, b, t));
    }
    const theta = Math.acos(dot);
    const sinTheta = Math.sin(theta);
    const wa = Math.sin((1.0 - t) * theta) / sinTheta;
    const wb = Math.sin(t * theta) / sinTheta;
    return [
      a[0] * wa + b[0] * wb,
      a[1] * wa + b[1] * wb,
      a[2] * wa + b[2] * wb
    ];
  }

  /**
   * Cubic Hermite smoothstep interpolation for smooth camera acceleration and deceleration.
   * s(t) = 3t^2 - 2t^3, for t in [0, 1].
   */
  function smoothstep(t) {
    const c = Math.max(0.0, Math.min(1.0, t));
    return c * c * (3.0 - 2.0 * c);
  }

  /**
   * Compute camera view basis vectors given yaw, pitch, distance, and target origin.
   * Pitch is clamped to [-1.45, 1.45] radians to prevent gimbal lock at the poles.
   *
   * @param {number} yaw Azimuth angle in radians
   * @param {number} pitch Elevation angle in radians
   * @param {number} dist Distance from target
   * @param {[number, number, number]} target Look-at center point
   * @param {string} upAxis World up axis: "Y" for globe, "Z" for local [i,j,k] block where +k is up
   * @returns {{pos: number[], fwd: number[], right: number[], up: number[], target: number[]}}
   */
  function computeCameraBasis(yaw, pitch, dist, target = [0, 0, 0], upAxis = "Y") {
    const clampedPitch = Math.max(-1.45, Math.min(1.45, pitch));
    const cp = Math.cos(clampedPitch);
    const sp = Math.sin(clampedPitch);

    if (upAxis === "Z") {
      const pos = [
        target[0] + dist * cp * Math.cos(yaw),
        target[1] + dist * cp * Math.sin(yaw),
        target[2] + dist * sp
      ];
      const fwd = vNorm(vSub(target, pos));
      const worldUp = [0, 0, 1];
      let right = vNorm(vCross(fwd, worldUp));
      if (Math.hypot(...right) < 1e-4) {
        right = [1, 0, 0];
      }
      const up = vNorm(vCross(right, fwd));
      return { pos, fwd, right, up, target };
    }

    const pos = [
      target[0] + dist * cp * Math.cos(yaw),
      target[1] + dist * sp,
      target[2] - dist * cp * Math.sin(yaw)
    ];

    const fwd = vNorm(vSub(target, pos));
    const worldUp = [0, 1, 0];
    let right = vNorm(vCross(fwd, worldUp));
    if (Math.hypot(...right) < 1e-4) {
      right = [1, 0, 0];
    }
    const up = vNorm(vCross(right, fwd));

    return { pos, fwd, right, up, target };
  }

  return {
    wgs84ToCartesian,
    cartesianToWgs84,
    fixtureToCartesian,
    computeCameraBasis,
    vDot,
    vCross,
    vNorm,
    vSub,
    vAdd,
    vScale,
    lerp,
    vLerp,
    vSlerp,
    smoothstep
  };
});
