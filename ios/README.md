# Oklo Pool iOS Application

This directory contains the native iOS application and Swift package for the Oklo Natural Reactor revision-0 fixture.

## Architecture

The iOS workspace consists of two primary components:

1. `OkloCoreKit`: A local Swift package wrapping the frozen `oklo_core` C ABI v1 static library (`OkloCoreFFI.xcframework`). It exposes the `OkloSim` class and `OkloSim.Field` enumeration.
2. `OkloPool`: A native SwiftUI application for iOS 17.0+ structured under `ios/OkloPool/`, generated from `ios/project.yml` via XcodeGen. The generated Xcode project `OkloPool.xcodeproj` is committed to the repository for reproducible builds without requiring XcodeGen installation.

```
ios/
├── project.yml               (XcodeGen project specification)
├── OkloPool.xcodeproj/       (Generated and committed Xcode project)
├── OkloCoreKit/              (Swift package wrapping oklo_core C ABI)
│   ├── Package.swift
│   ├── OkloCoreFFI.xcframework
│   ├── Sources/OkloCore/     (OkloSim Swift wrapper)
│   └── Tests/OkloCoreTests/  (Bit-for-bit MRC verification tests)
├── OkloPool/                 (SwiftUI application target)
│   ├── App/                  (OkloPoolApp entry point and asset catalog)
│   ├── Models/               (OkloAppModel, FieldCatalog, ColorRamps, PrescribedCurves)
│   ├── Views/                (ContentView, Stage slices/3D, Controls, Charts, Provenance)
│   └── Shaders/              (VolumeRaymarcher.metal volume raymarching shader)
├── OkloPoolTests/            (Unit tests for app model, stepping, null scenario)
└── OkloPoolUITests/          (UI automation tests for launch and controls)
```

## System Requirements

- macOS 14.0 or later (tested on macOS 27.0 arm64)
- Xcode 16.0 or later (tested on Xcode 27.0 Build 27A266a)
- Swift 6.0
- Target Platform: iOS 17.0 or later (iPhone and iPad)

## Project Generation (Optional)

`OkloPool.xcodeproj` is committed directly to source control. If modifications to `ios/project.yml` are made, regenerate the Xcode project with:

```bash
cd ios
xcodegen generate --spec project.yml
```

## Reproducible Build Commands

### 1. Build Application for Generic iOS Device

Builds the application binary for physical iOS hardware (code signing disabled for local development):

```bash
xcodebuild -project ios/OkloPool.xcodeproj \
  -scheme OkloPool \
  -destination "generic/platform=iOS" \
  build
```

### 2. Build Application for Generic iOS Simulator

Builds the application binary for Apple Silicon iOS Simulators:

```bash
xcodebuild -project ios/OkloPool.xcodeproj \
  -scheme OkloPool \
  -destination "generic/platform=iOS Simulator" \
  build
```

### 3. Build Application and Test Bundles for Testing

Builds the application, unit tests, and UI test bundles:

```bash
xcodebuild -project ios/OkloPool.xcodeproj \
  -scheme OkloPool \
  -destination "generic/platform=iOS Simulator" \
  ARCHS=arm64 \
  build-for-testing
```

## Testing Protocol and Evidence

### 1. Swift Package Conformance Tests (PASS)

The core wrapper tests run on the host macOS toolchain and verify all six fixture fields against reference Python MRC files bit for bit:

```bash
(cd ios/OkloCoreKit && swift test)
```

Result: 5 tests passed (including 6 parameterized bit-for-bit comparisons) in ~0.12 seconds.

### 2. Simulator Runtime Tests (BLOCKED)

Status: BLOCKED per Rule 5.
Reason: 0 iOS Simulator runtimes are installed in the local CoreSimulator environment (`xcrun simctl list runtimes` returns empty).
Executing `xcodebuild ... test` exits with code 70:
`Cannot test target "OkloPoolTests" on "Any iOS Simulator Device": Tests must be run on a concrete device`

Prerequisite to unblock simulator testing:
Install an iOS simulator runtime using Xcode:
```bash
xcodebuild -downloadPlatform iOS
```
Once installed, execute:
```bash
xcodebuild -project ios/OkloPool.xcodeproj \
  -scheme OkloPool \
  -destination "platform=iOS Simulator,name=iPhone 16" \
  test
```

### 3. Physical Device Testing (BLOCKED)

Status: BLOCKED.
Reason: No connected physical iOS device detected (`xcrun devicectl list devices` returns empty), and no Apple Developer Program team signing identity is provisioned on the host.

## Application Features

### 1. Six Revision-0 Fixture Fields

Each field is displayed with its canonical unit, colormap, and literature limitation per `docs/PROVENANCE.md`:

- Temperature: degrees C as labelled; coefficients underived. Inferno colormap. Not a thermal-hydraulic solution.
- Hydration: pore saturation, dimensionless (0.02 to 1.0). Blues colormap. Prescribed drying shape, not a transport solution.
- H2 Concentration: index; labelled mmol/L with no dose basis. Viridis colormap. G-value alone cannot give a concentration.
- 235U/238U Ratio: atom ratio; natural baseline 0.00725. Cividis colormap. Ratio, not interchangeable with 0.7204 atom percent.
- Polymer Yield: index; labelled umol/L with no kinetic basis. Magma colormap. Mass balance closes by construction.
- Compartments: zone labels 0 to 4 (categorical: Sandstone, Reactor Core, Halo, Fracture Conduit, Steam Chimney). Prescribed geometry, not mapped geology.

### 2. Interactive Phase Navigation

- Scrubbing slider across 60 frames / 10,800s (3.0 hours).
- Play/Pause transport control with timer-driven playback.
- Forward and backward step buttons (180s / 1 frame per step).
- Pulse boundary selector: toggle between 0.17 (stored reference files) and 0.1667 (original web GUI).
- Null scenario toggle: zeroing pulse factor P(p) and desiccation D(p).
- Phase region badges: Pulse/Critical Boiling, Desiccation/Dry Bake, Quiescent Cooldown, Infiltration/Re-wetting.

### 3. Spatial Visualization

- Plan Slice: Horizontal XY slice at depth level k (0 to 31), displaying cut line for section row j.
- Section Slice: Vertical XZ slice at row j (0 to 31), displaying cut line for depth level k.
- 3D Volume Raymarcher: MetalKit MTKView raymarching shader sampling 3D volume texture and 2D transfer function LUT with camera orbiting gestures and opacity control.
- Interactive crosshairs and direct-touch voxel inspection showing coordinate values.

### 4. Prescribed Field Strip Chart

- Native SwiftUI Charts (`import Charts`).
- Top panel: Mean curve and [min, max] envelope for continuous fields; zone percentage shares for compartments.
- Bottom panel: Prescribed driver curves for pulse P(p) (amber) and desiccation D(p) (cyan).
- Vertical phase cursor tracking active scrubbing.

### 5. In-App Epistemic Disclosures

Accessible via the info icon in the navigation toolbar, presenting a modal with verbatim text from `docs/PROVENANCE.md` explaining that phase stepping navigates prescribed analytic functions rather than physical or empirical dynamics.

### 6. Accessibility & Invariants

- Full VoiceOver labels and values across all controls and slice views.
- Dynamic Type support.
- Dual-stroke high-contrast crosshair rendering.
- Zero en-dashes, zero em-dashes, and zero quarantined terms across all source and metadata files.
