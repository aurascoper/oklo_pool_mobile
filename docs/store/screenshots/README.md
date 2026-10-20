# Store screenshots

Status: **CAPTURED FROM RUNNING SIMULATOR AND EMULATOR BUILDS** (2026-10-19).

Nine authentic high-resolution PNG screenshots captured directly from the running native iOS Simulator (`iPhone 17`, iOS 27.0 `arm64`, `1206x2622`) and Android Emulator (`oklo-api36`, Android 16 / API 36 `arm64-v8a`, `1080x2400`) applications are included in this directory.

## iOS (`iPhone 17`, iOS 27.0 `24A5297g`, `1206x2622` PNG)

Captured during `xcodebuild test -project ios/OkloPool.xcodeproj -scheme OkloPool -destination 'platform=iOS Simulator,id=646CF1C6-8720-4906-B73B-FD46ACAA8EF0'` (`OkloPoolUITests/testCaptureStoreScreenshots`):

| File | Resolution | SHA-256 | Depicted view |
| --- | --- | --- | --- |
| `ios-iphone17-slices.png` | `1206x2622` | `57a717582dcf89bff61805a02762fd52f436b093b91e113699f1bfe8069a612b` | 2D Plan (`XY` at `Z = 16`) and Section (`XZ` at `Y = 16`) slices, Temperature field (`26.85 to 432.2 degrees C`), transport controls, pulse boundary `0.1700` vs `0.1667` |
| `ios-iphone17-volume.png` | `1206x2622` | `497e5b8afea79efff9f3ac4bc7fd6d13cc4053638ae7083d26769b9df302865d` | 3D Volume raymarcher view (`32x32x32` voxel cube) with azimuth/elevation/opacity sliders and transport bar |
| `ios-iphone17-fields-chart.png` | `1206x2622` | `c52a8944a1158e58d4d2f43724a2914bd1b6e503dfd47d3e7821d596dcafa316` | Compartments field (`zone 0 to 4`, discrete 5-zone palette) and strip chart below slices |
| `ios-iphone17-provenance.png` | `1206x2622` | `9252b1cd4f008a4c9251d63fdc1d6ce8f7ea34a0eef6496d16675ba3b2467476` | In-app Provenance and Scientific Scope sheet disclosing prescribed revision 0 analytic functions and limitations |

## Android (`oklo-api36`, Android 16 / API 36 `arm64-v8a`, `1080x2400` PNG)

Captured during `adb shell am instrument -w -r org.oklo.pool.test/androidx.test.runner.AndroidJUnitRunner` (`ViewerSmokeTest`):

| File | Resolution | SHA-256 | Depicted view |
| --- | --- | --- | --- |
| `android-api36-plan-slice.png` | `1080x2400` | `605bdb7f334ae63c75802e7a75ef91845f23c53b927e75597b1a239df787d23c` | Plan slice (`X by Y at Z = 16`) of Temperature field, depth slider, phase transport, pulse boundary, and null-scenario controls |
| `android-api36-section-slice.png` | `1080x2400` | `cd59dc100693d79014638f6eaf1af9652033ccb85fc9273e9608be912ad1d9b5` | Section slice (`X by Z at Y = 16`) of Temperature field with row slider and phase controls |
| `android-api36-volume.png` | `1080x2400` | `17b96962174f4c070f13f9179618b084c2ddb54773305473c35a3e0cac2a8659` | Centered 3D isometric volume projection (16 depth layers) of Temperature field |
| `android-api36-chart.png` | `1080x2400` | `1b53a7121554142ceeac3e1ab5a1f39a703ed601efe4ca3b2335d2cc3460a1e7` | Dual-canvas chart: Temperature mean and min/max envelope over cycle phase plus prescribed pulse and desiccation driver curves |
| `android-api36-provenance.png` | `1080x2400` | `61930fa8c9016c22b859e153ca8ee8f32bc3ddeb50c468fc4b570856364bdf6f` | In-app Assumptions and Provenance modal dialog detailing prescribed fixture equations and unit caveats |
