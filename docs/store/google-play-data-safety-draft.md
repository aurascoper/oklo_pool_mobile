# Google Play Data safety draft

Status: **EVIDENCE-BACKED DRAFT (Pending Account-Owner Signed AAB Review and Console Entry).** Source audit, merged-manifest inspection, resolved `releaseRuntimeClasspath` dependency graph audit, and Android API 36 emulator runtime inspection support a "No data collected or shared" declaration in Google Play Console. See the [Data safety requirements](https://support.google.com/googleplay/android-developer/answer/10787469?hl=en-GB).

## Evidence reviewed

- Source and direct dependencies (`android/app/build.gradle.kts`) declare no account flow, analytics SDK, advertising SDK, crash reporter, or network client.
- The Android app manifest requests no `android.permission.INTERNET` permission and no user-data permission.
- The merged manifest contains only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`. Without `android.permission.INTERNET`, the Android Linux kernel denies `AF_INET` and `AF_INET6` socket creation for the app UID.
- The resolved `releaseRuntimeClasspath` dependency graph is captured in [`android-release-runtime-dependencies.txt`](android-release-runtime-dependencies.txt) (SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`). Only AndroidX UI/Lifecycle/Core, Kotlin stdlib/coroutines, and Guava `listenablefuture` are present.
- Runtime verification on the Android 16 / API 36 `arm64-v8a` emulator (`oklo-api36`, PID `4165`) confirmed that only `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` is granted and zero TCP/UDP sockets are open (`ACTIVE_INET_SOCKETS: 0`). See `receipts/android_emulator_test_2026-10-19.txt` on branch `android/v1-app`.

## Proposed Play Console Data safety responses

| Form question | Proposed response | Supporting evidence |
| --- | --- | --- |
| Does your app collect or share any of the required user data types? | **No** | No `INTERNET` permission, no network client or telemetry SDK in `releaseRuntimeClasspath`, and zero active sockets on API 36 emulator runtime. |
| Data types collected | **None** | All fixture calculations and rendering occur in local memory (`oklo_core` via JNI). |
| Data shared with third parties | **None** | No third-party analytics, advertising, or network SDK is included; network access is not permitted by the manifest. |
| Privacy policy URL | **Required before submission** | Publish `docs/store/privacy-policy.html` at a public HTTPS URL after inserting the publisher name and monitored support email. |

## Account-owner actions

1. Sign the release Android App Bundle (`app-release.aab`) with the production upload key and confirm no build post-processing injects additional permissions or libraries.
2. Enter the Data safety responses above in Google Play Console.
3. Publish the finalized privacy policy at a public HTTPS URL and enter that URL in Play Console.

The app targets API 36 (`compileSdk = 36`, `targetSdk = 36`), satisfying Google Play's requirement for new apps and updates effective 2026-08-31: [target API requirement](https://developer.android.com/google/play/requirements/target-sdk?authuser=383).
