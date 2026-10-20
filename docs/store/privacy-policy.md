# Privacy policy draft: Oklo Fixture Viewer

Status: **Draft ready for publisher contact completion and HTTPS hosting.** Source review, resolved dependency graph audits, and runtime inspections on the iOS 27.0 Simulator and Android API 36 Emulator found no network client, analytics SDK, advertising SDK, account flow, user-data permission, or open network socket. Before publishing this policy at a public HTTPS URL, the account owner must fill in the effective date, publisher name, and monitored support email, and confirm that the final signed store archives are unchanged.

Effective date: [Insert effective date prior to publication]
App: Oklo Fixture Viewer
Publisher: [Insert publisher or account-owner name]
Support contact: [Insert monitored support email]
Public policy URL: [Insert public HTTPS URL once hosted]

## What the app does

Oklo Fixture Viewer is a free scientific visualization app that presents six fields from the revision 0 Oklo natural reactor conformance fixture. Users can select a field, change the displayed cycle phase, choose a pulse boundary (`0.17` or `0.1667`), toggle the null scenario, and inspect 2D plan and section slices, a 3D volume view, and prescribed field charts. All displayed values are evaluated locally from fixed analytic fixture equations. Phase navigation does not run a live physical dynamics simulation.

## Data collection, sharing, and transmission

Oklo Fixture Viewer does not collect, store, transmit, or share personal data, usage analytics, diagnostics, or device identifiers:

- The app has no user accounts, sign-in, cloud sync, analytics SDK, advertising SDK, crash-reporting service, or network client.
- All computations and rendering run locally on the user's device using the embedded `oklo_core` engine.
- On Android, the application manifest does not request the `android.permission.INTERNET` permission or any user-data permission.
- On iOS, the application declares no external package dependencies, no network client, and no sensitive system permission usage descriptions.
- Runtime verification on both the iOS Simulator (`iPhone 17`, iOS 27.0) and the Android Emulator (Android 16 / API 36) confirmed zero open network sockets during operation.

## Device storage and permissions

- The app does not persist personal information or user preferences to disk across launches.
- On Android, `allowBackup` is set to `false`, and the merged manifest contains only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` used internally by AndroidX for non-exported dynamic receivers.
- On iOS, the app does not request access to location, camera, photos, contacts, microphone, tracking (`IDFA`), or local network resources.

## Distribution platforms

When you download or update Oklo Fixture Viewer through the Apple App Store or Google Play Store, Apple Inc. or Google LLC may collect or process installation and account information governed by their own privacy policies and terms of service. Oklo Fixture Viewer itself receives no personal data from those platforms.

## Children's privacy

Oklo Fixture Viewer does not collect personal information from anyone, including children.

## Changes to this policy and contact

If future versions of Oklo Fixture Viewer change how data is handled, this privacy policy will be updated prior to release. For questions regarding this policy or the application, contact: [Insert monitored support email].
