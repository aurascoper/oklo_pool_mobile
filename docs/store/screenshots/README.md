# Store screenshots

Status: **CAPTURED FROM RUNNING SIMULATOR AND EMULATOR BUILDS** (Verified 2026-10-20).

Nine authentic high-resolution PNG screenshots captured directly from the running native iOS Simulator (`iPhone 17`, iOS 27.0 `arm64`, `1206x2622`) and Android Emulator (`oklo-api36`, Android 16 / API 36 `arm64-v8a`, `1080x2400`) applications are included in this directory. Zero mock or placeholder graphics are used.

All screenshots depict the genuine user interface, including:
- Persistent initial-screen statement: "Revision-0 prescribed fields; not a physical simulation."
- Dynamic adjacent field limitation statements.
- The six field limitation records accessible in the Provenance and Scope sheet:
  1. Temperature: Prescribed thermal profile; not coupled to hydrodynamic or convective cooling.
  2. Hydration: Prescribed saturation index; not a multiphase porous flow solve.
  3. H2 Concentration: Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve.
  4. 235U/238U Ratio: Prescribed isotopic depletion curve; not a 3D neutron transport solve.
  5. Polymer accumulation index (arbitrary units): Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.
  6. Compartments: Prescribed geometric bounding cells; not a physical membrane simulation.

## iOS (`iPhone 17`, iOS 27.0 `24A434`, `1206x2622` PNG)

Captured during `xcodebuild test -project ios/OkloPool.xcodeproj -scheme OkloPool -destination 'platform=iOS Simulator,name=iPhone 17,OS=27.0'` (`OkloPoolUITests`) against tested source commit `dd1ce73bea4716cd5a8161a56bdc8868fd792a01`:

| File | Resolution | SHA-256 | Depicted view |
| --- | --- | --- | --- |
| `ios-iphone17-slices.png` | `1206x2622` | `629f4db9fdad2e4c5366790584c795ebb778363f777503477f86e1c8d5515a54` | Persistent `"Revision-0 prescribed fields; not a physical simulation."` banner, 2D Plan (`XY` at `k = 12`) and Section (`XZ` at `j = 15`) slices of Temperature field |
| `ios-iphone17-volume.png` | `1206x2622` | `3811d6fa848f628b5460ae5fabc1e011160d650e5dd010bf1be62d76438badbc` | Persistent top banner, 3D Volume raymarcher view (`32x32x32` voxel cube), adjacent selected-field limitation banner, and Phase Navigation (`Frame 9 of 60`) |
| `ios-iphone17-fields-chart.png` | `1206x2622` | `a9ce6dddb52fc122b0216d2022c901b1063e07e308d15c203d56d0183a583394` | Persistent top banner, Field Selection (`Polymer accumulation index`, `arbitrary units`), adjacent limitation banner, and Prescribed Field Strip Chart + Driver Curves |
| `ios-iphone17-provenance.png` | `1206x2622` | `a4560516d981dbbb2847cda2af19071f475234a9fa5bb3e9ce2430de6eecb11c` | In-app Provenance and Limitations sheet disclosing prescribed revision 0 analytic functions, field-by-field limitations, Option A polymer caveat, and offline Privacy Policy |

## Android (`oklo-api36`, Android 16 / API 36 `arm64-v8a`, `1080x2400` PNG)

Captured during `adb shell am instrument -w -r org.oklo.pool.test/androidx.test.runner.AndroidJUnitRunner` (`ViewerSmokeTest`):

| File | Resolution | SHA-256 | Depicted view |
| --- | --- | --- | --- |
| `android-api36-plan-slice.png` | `1080x2400` | `605bdb7f334ae63c75802e7a75ef91845f23c53b927e75597b1a239df787d23c` | Plan slice (`X by Y at Z = 16`) of Temperature field, depth slider, phase transport, pulse boundary, and null-scenario controls |
| `android-api36-section-slice.png` | `1080x2400` | `cd59dc100693d79014638f6eaf1af9652033ccb85fc9273e9608be912ad1d9b5` | Section slice (`X by Z at Y = 16`) of Temperature field with row slider and phase controls |
| `android-api36-volume.png` | `1080x2400` | `17b96962174f4c070f13f9179618b084c2ddb54773305473c35a3e0cac2a8659` | Centered 3D isometric volume projection (16 depth layers) of Temperature field |
| `android-api36-chart.png` | `1080x2400` | `1b53a7121554142ceeac3e1ab5a1f39a703ed601efe4ca3b2335d2cc3460a1e7` | Dual-canvas chart: Temperature mean and min/max envelope over cycle phase plus prescribed pulse and desiccation driver curves |
| `android-api36-provenance.png` | `1080x2400` | `ad80a1f4f8d66f21e74ea673a610fb939cfe61405b567e7345dc7f5639c38930` | In-app Assumptions and Provenance modal sheet detailing prescribed fixture equations and the Option A polymer display-ontology caveat |

Note on Android Emulator Status:
During 2026-10-20 On-Render Gate verification, fresh `oklo-api36` AVD startup was BLOCKED / UNRESOLVED due to host APFS disk space constraint (<1 GiB available). The screenshots above remain authentic captures preserved from earlier verified emulator runs.

## Account-owner blocker catalog before submission

1. App Store Connect account configuration, bundle ID registration, signing certificates, and live hosting confirmation of privacy policy.
2. Physical iOS and Android device validation (Apple Developer account, TestFlight, Google Play Console internal testing).
3. Android Emulator host disk space: host machine has <1 GiB free disk space, blocking oklo-api36 AVD startup (status: BLOCKED / UNRESOLVED per Launch Gates).
