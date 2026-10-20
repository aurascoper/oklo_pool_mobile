# Apple App Privacy draft

Status: **UNVERIFIED. Do not submit.** The App Privacy answers must describe the final built app and all included third-party code. Runtime network behavior and the final transitive dependency inventory have not been verified.

## Source-level review

The iOS app declares Apple platform frameworks and the local `OkloCoreKit` Rust wrapper. No third-party analytics, advertising, or attribution SDK is declared. Static source inspection found no app networking client, tracking code, account flow, or data-access usage description. These findings do not prove the absence of runtime transmission.

## App Store Connect answers to determine

| Topic | Current status | Required evidence before entry |
| --- | --- | --- |
| Data collected | UNVERIFIED | Signed release package review and runtime network capture. |
| Data linked to the user | UNVERIFIED | Final data inventory and service behavior. |
| Data used for tracking | UNVERIFIED | Final SDK/domain review and runtime test. |
| Privacy policy URL | BLOCKED | Publish the approved policy at a real public HTTPS URL. |
| Age rating | NOT ENTERED | Account owner completes the current questionnaire. |

If final evidence supports no data collection, choose the corresponding App Store Connect answers then. No category is selected in this draft.

## Privacy manifest and third-party code

Apple's `PrivacyInfo.xcprivacy` format records collected data, tracking, and required-reason API access. Apple also requires valid manifests from specified commonly used third-party SDKs. The reviewed project declares no third-party SDK; the local Rust core is built into the app. Recheck the final archive and current Apple SDK requirements before submission. Do not add a manifest that declares no collected data until the runtime and API audits are complete. See [Apple privacy manifest documentation](https://developer.apple.com/documentation/bundleresources/privacy-manifest-files?changes=_8) and [third-party SDK manifest guidance](https://developer.apple.com/documentation/bundleresources/adding-a-privacy-manifest-to-your-app-or-third-party-sdk?changes=_8).
