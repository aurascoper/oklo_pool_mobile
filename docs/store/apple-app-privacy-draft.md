# Apple App Privacy draft

Status: **EVIDENCE-BACKED DRAFT (Pending Account-Owner Signed Archive Review and Console Entry).** Source audit, Swift package dependency graph inspection, and iOS 27.0 Simulator runtime socket inspection (`ZERO_INET_SOCKETS`) support a "Data Not Collected" disclosure. The account owner must confirm this against the final signed `.ipa` archive before submitting in App Store Connect.

## Source, dependency, and runtime review

- The iOS app (`ios/OkloPool`) uses Apple system frameworks (`SwiftUI`, `Charts`, `MetalKit`) and the local `OkloCoreKit` Swift wrapper around the Rust `oklo_core` static library.
- `swift package show-dependencies --format text` reports zero external Swift package dependencies.
- Static source inspection found no networking client (`URLSession`, `NWConnection`, `WKWebView`), no analytics or advertising SDK, no account flow, no `UserDefaults` or file-system telemetry, and no sensitive permission usage descriptions.
- Runtime verification on the booted `iPhone 17` simulator (iOS 27.0 `arm64`, PID `47404`) during and after full UI exercise confirmed zero open `IPv4` or `IPv6` sockets (`lsof -n -P -i -a -p 47404` -> `ZERO_INET_SOCKETS`). See `receipts/ios_simulator_test_2026-10-19.txt`.

## Proposed App Store Connect answers

| Topic | Proposed answer | Evidence and remaining account-owner check |
| --- | --- | --- |
| Do you or your third-party partners collect data from this app? | **No** (Data Not Collected) | Supported by source review, empty external dependency graph, and simulator runtime socket check (`ZERO_INET_SOCKETS`). Confirm on final signed `.ipa` archive before submitting. |
| Data linked to the user | **None** | No user accounts, identifiers, or network transmissions exist in the app. |
| Data used to track the user | **None** | No tracking SDK, advertising identifier (`IDFA`), or external network request is present. |
| Privacy policy URL | **Required before submission** | Host `docs/store/privacy-policy.html` at a public HTTPS URL after filling in the publisher name and support email. |
| Age rating | **Account-owner questionnaire** | Complete in App Store Connect (the app contains scientific visualization only, with no violence, user-generated content, commerce, or unrestricted web access). |

## Privacy manifest (`PrivacyInfo.xcprivacy`) and third-party code

Apple's `PrivacyInfo.xcprivacy` records data collection, tracking, and required-reason API usage. Neither `OkloPool` nor `OkloCoreKit` calls required-reason APIs (`UserDefaults`, file timestamp APIs, system boot time APIs, disk space APIs, or active keyboard APIs) or embeds third-party SDKs. When creating the signed distribution archive in Xcode, verify whether current App Store Connect upload validation requests an explicit `PrivacyInfo.xcprivacy` with `NSPrivacyTracking = false`, `NSPrivacyCollectedDataTypes = []`, and `NSPrivacyAccessedAPITypes = []`. See [Apple privacy manifest documentation](https://developer.apple.com/documentation/bundleresources/privacy-manifest-files?changes=_8).
