# Google Play Data safety draft

Status: **EVIDENCE-BACKED DRAFT (Pending Account-Owner Signed AAB Review and Console Entry).** Source audit, merged-manifest inspection, resolved `releaseRuntimeClasspath` dependency graph audit, and Android API 36 emulator runtime inspection support a "No data collected or shared" declaration in Google Play Console. See the [Data safety requirements](https://support.google.com/googleplay/android-developer/answer/10787469?hl=en-GB).

## Evidence reviewed

- Source and direct dependencies (`android/app/build.gradle.kts`) declare no account flow, analytics SDK, advertising SDK, crash reporter, or network client.
- The Android app manifest requests no `android.permission.INTERNET` permission and no user-data permission.
- The merged manifest contains only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`. Without `android.permission.INTERNET`, the Android Linux kernel denies `AF_INET` and `AF_INET6` socket creation for the app UID.
- The resolved `releaseRuntimeClasspath` dependency graph is captured in [`android-release-runtime-dependencies.txt`](android-release-runtime-dependencies.txt) (SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`). Only AndroidX UI/Lifecycle/Core, Kotlin stdlib/coroutines, and Guava `listenablefuture` are present.
- In-app disclosures:
  - Persistent initial-screen statement: "Revision-0 prescribed fields; not a physical simulation."
  - Six field limitation records documented in the Scope bottom sheet.
  - Bundled offline full-text privacy policy accessible directly inside the app.
- Runtime verification:
  - Prior booted emulator execution (`oklo-api36`, PID `4165`) confirmed zero open sockets (`ACTIVE_INET_SOCKETS: 0`).
  - Recent automated On-Render Gate emulator startup is documented as `BLOCKED / UNRESOLVED` due to host APFS disk space (<1 GiB available). Android JVM native tests against release JNI libraries verified all simulation and view-model pathways with 18/18 passes.

## Proposed Play Console Data safety responses

| Form question | Proposed response | Supporting evidence |
| --- | --- | --- |
| Does your app collect or share any of the required user data types? | **No** | No `INTERNET` permission, no network client or telemetry SDK in `releaseRuntimeClasspath`, and zero active sockets on runtime check. |
| Data types collected | **None** | All fixture calculations and rendering occur in local memory (`oklo_core` via JNI). |
| Data shared with third parties | **None** | No third-party analytics, advertising, or network SDK is included; network access is not permitted by the manifest. |
| Privacy policy URL | **https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html** | Explicit disclosure: The proposed URL remains unverified until confirmed live by the account owner. |

## Account-owner blocker catalog

The following blockers must be resolved before Google Play Store submission:

1. Sign the release Android App Bundle (`app-release.aab`) with the production upload key and confirm no build post-processing injects additional permissions or libraries.
2. Publish the finalized privacy policy at `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html` and confirm live hosting.
3. Free host APFS disk space (<1 GiB currently available) to unblock `oklo-api36` AVD startup and allow full instrumentation runs.
4. Execute physical-device validation on physical Android handset/tablet via Google Play internal testing track.
5. Enter the Data safety responses and store listing in Google Play Console.

The app targets API 36 (`compileSdk = 36`, `targetSdk = 36`), satisfying Google Play's requirement for new apps and updates effective 2026-08-31: [target API requirement](https://developer.android.com/google/play/requirements/target-sdk?authuser=383).
