# iOS Simulator Acceptance Receipt

Status: **PASS** for iOS simulator unit and UI tests.
Branch: `ios/v1-app`
Tested source commit: `2db67d4ce3526bb2968954fecdba6ade7a991b02` (`git diff 2db67d4ce3526bb2968954fecdba6ade7a991b02..HEAD -- ':!receipts'` is empty; `ios/` and `crates/` trees are byte-identical to `3b0390351fda773a5b3fb379cef44fc8f94ff7ba`)
Worktree clean at test: `true`
Main base commit: `041d1bab092553798dec6f7b84df1edfbe417ab4`

## Tested source and test file SHA-256 manifest

- `6a22fd0f2628acd974851b5345a0f73cfeb6c2ef2137cb901063b42701e4b191`  `ios/OkloCoreKit/Package.swift`
- `69146957b2f06729acdf15ef2b063b9c0e286a51c47be501cf71dfa6acb7ea90`  `ios/OkloCoreKit/Sources/OkloCore/OkloSim.swift`
- `904e970a3faf0bbe99decc5a7bffb7c8e404a5e3edfa576bb4e58899da67fc1d`  `ios/OkloCoreKit/Tests/OkloCoreTests/OkloSimTests.swift`
- `c6b5faca9ff0b62c81f19d8a29bd128ce96a2525ca2a4552836068e08a2a1519`  `ios/OkloPool.xcodeproj/project.pbxproj`
- `f8e395df8caf3391628e364c43368bd2ec3e9c5226700badb5b34b4a8fabc69f`  `ios/OkloPool/App/OkloPoolApp.swift`
- `ba983e233248b071b6757bfa69958b0086eff14e0e8e1b1034598f70a37cdbc7`  `ios/OkloPool/Models/ColorRamps.swift`
- `4b34e50d2fdb819026ce6c7f66dab247476d2b3dcc5041d3eed64c2ca369d3d7`  `ios/OkloPool/Models/FieldCatalog.swift`
- `1374e423fb6002999b30ef2f2d6f2a85d9e2ccf059c17551ea36537e25339dd2`  `ios/OkloPool/Models/OkloAppModel.swift`
- `f8465ff10e8d761fe6ecd0526b224416cbf443789eaaf63796d09c935258897a`  `ios/OkloPool/Models/PrescribedCurves.swift`
- `d4f453e06a344ee29c6f9826cdc62e42c33fb2bcc02c2f0875b881ed6bca4c14`  `ios/OkloPool/Shaders/VolumeRaymarcher.metal`
- `5ab9228fd6c0c869614f97ba70950e076c17cd93d37c9d417ad794471c62898e`  `ios/OkloPool/Views/Charts/StripChartView.swift`
- `0777b1a96332c5ce5f394308c560be94b961805b90cbfa6624fa5e7bf0cde5c0`  `ios/OkloPool/Views/ContentView.swift`
- `2a4ab6dd7d9f2c875408d4333910895ac24afb4ee882181e280e46b0e486a79e`  `ios/OkloPool/Views/Controls/FieldPickerView.swift`
- `c6ef760d982ba3850050d1d5a6c8ec3546531581c72f6c9a6b11d2ce721177d2`  `ios/OkloPool/Views/Controls/PhaseControlView.swift`
- `c2ee28c196e4ba03d64cd5e9c834d3e7fcd894e8a56e8f0fe4ebd670e0bf7fdf`  `ios/OkloPool/Views/Controls/ScenarioControlsView.swift`
- `b78de043c5262fc4ec8a1a1c0c9afaf34c75cdfe761148769e67010a33528e86`  `ios/OkloPool/Views/Provenance/ProvenanceSheetView.swift`
- `bb8624d5310c3a64ecae06d0f4fe4fe7a3ad8aae24dcc63b8dd6ae4e4404da8d`  `ios/OkloPool/Views/Stage/PlanSliceView.swift`
- `099568966924d56184f47e29bb06b5e8e605e4b41f1b6010e27bd4b3818f44f3`  `ios/OkloPool/Views/Stage/SectionSliceView.swift`
- `fc2f7df0456e87e4dfd1fb6a0dc6250d0236fc388a21e159ec9c2f8b52d3801d`  `ios/OkloPool/Views/Stage/SliceContainerView.swift`
- `fb347cb34e90d63f7454f4ef35927d771c7a967b8e034fddac4f4a3757e19508`  `ios/OkloPool/Views/Stage/VolumeRaymarcherView.swift`
- `72b294476597fb110a2e214e0ba9cea42e816950ea0256db9b2524e7f558447f`  `ios/OkloPoolTests/OkloAppModelTests.swift`
- `fec9a7700ee666f304545a1ed04113482e27c520019bba946c5076b8e1ac3c4c`  `ios/OkloPoolUITests/OkloPoolUITests.swift`
- `f65e9ccccb2cb572bfbf0b103c68399759ea6cde93abf98ad62b817f239e8cda`  `ios/project.yml`

## Environment

- Host: arm64 macOS 27.0.1.
- Xcode: 27.0, build 27A266a.
- Runtime: iOS 27.0, build 24A434.
- Simulator: iPhone 17, arm64, UDID `646CF1C6-8720-4906-B73B-FD46ACAA8EF0`.
- App version: 1.0.0, bundle ID `org.oklo.pool`.

## Commands

```sh
bash scripts/build_xcframework.sh
xcodebuild test -project ios/OkloPool.xcodeproj \
  -scheme OkloPool \
  -destination 'platform=iOS Simulator,id=646CF1C6-8720-4906-B73B-FD46ACAA8EF0' \
  -derivedDataPath /tmp/oklo-ios-derived -jobs 1 CODE_SIGNING_ALLOWED=NO
```

The framework build used Cargo `--offline` and the already installed Rust targets `aarch64-apple-ios`, `aarch64-apple-ios-sim`, and `aarch64-apple-darwin`. XcodeGen 2.46.0 was used to regenerate the checked-in project from `ios/project.yml`; the generated project file matched the committed version.

## Results

- Result: **PASS**, 14 tests, 0 failures, 0 skipped.
- Unit tests: 10 passed (including `testCf2TwentyCaseMatrixMatchesReference`, verifying all 20 CF-2 cases / 120 3D field volumes bit-for-bit against the reference SHA-256 digests on the iOS 27.0 arm64 simulator).
- UI tests: 4 passed: app launch and main controls, provenance presentation, playback, and view-mode switching with screenshot capture.
- Runtime network check: `lsof -n -P -i -a -p <pid>` against the launched `org.oklo.pool` simulator process (PID 47404) reported zero IPv4 or IPv6 network sockets.
- The structured Xcode result is `ios_simulator_test_2026-10-19.json`; SHA-256: `506d2740252b11edafa7f2d1a2557a83b71c2d59253a7c2f8ce825c4b14f0903`.
- The console test log is `ios_simulator_test_2026-10-19.txt`; SHA-256: `9e75c60ca4259cdb2eb5672a6f02857494f4dea89101bc987831d149f7779a5d`.

The first simulator run exposed two stale assertions: a 15.0 polymer-yield maximum despite an observed fixture sweep maximum of 15.12197, and a UI test querying a visible button title instead of its explicit accessibility label. Both assertions were corrected. Runtime review also found initial chart-summary calculation blocking the first render, slice drag gestures intercepting vertical scroll on compact iPhone screens, and Metal volume blend factors darkening premultiplied alpha output. Those issues were resolved so the initial frame renders immediately with current-frame dynamic range, tap gestures inspect slice voxels without blocking vertical scroll, and the 3D raymarcher renders with premultiplied alpha blending. The final result above is from the corrected code.

## Still outstanding

- Physical-device smoke test and distribution signing: **BLOCKED**, no device or signing identity available.
- Store privacy declarations, runtime network observation, policy hosting, account-owner console entries, and publication: outstanding; no store submission or release is claimed.
- Scientific scope remains the prescribed revision-0 fixture, not validated physical dynamics.
