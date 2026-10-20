# iOS Simulator Acceptance Receipt

Status: **PASS** for iOS simulator unit and UI tests.

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
