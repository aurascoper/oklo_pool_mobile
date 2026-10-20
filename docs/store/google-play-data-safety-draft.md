# Google Play Data safety draft

Status: **UNVERIFIED. Do not submit this draft.** Google Play requires the Data safety form for apps on published testing and production tracks, including apps whose final answer is no user data collected or shared. The completed form also needs a public privacy-policy link. See the [Data safety requirements](https://support.google.com/googleplay/android-developer/answer/10787469?hl=en-GB).

## Evidence reviewed

- Source and direct dependencies do not declare an account flow, analytics SDK, ad SDK, or app network client.
- Android app manifest requests no `INTERNET` permission or user-data permission.
- Merged manifest contains one AndroidX-generated signature-only dynamic receiver permission.
- The resolved `releaseRuntimeClasspath` dependency graph is captured in [`android-release-runtime-dependencies.txt`](android-release-runtime-dependencies.txt). No app networking, analytics, advertising, attribution, or crash-reporting SDK is present in that graph. Runtime network behavior remains untested, and the final signed package has not been reviewed.

## Form responses to determine after runtime verification

| Form topic | Current status | Entry rule |
| --- | --- | --- |
| Does the app collect or share user data? | UNVERIFIED | Do not select a final answer until runtime network and final package review are complete. If evidence confirms no collection or sharing, answer accordingly. |
| Data types collected | UNVERIFIED | No type is established by current source inspection. Recheck all SDKs and runtime behavior. |
| Data shared with third parties | UNVERIFIED | No app sharing code was found; runtime and SDK behavior are not verified. |
| Security practices | UNVERIFIED | Review actual release storage, transport, and access behavior before answering. |
| Privacy policy URL | BLOCKED | Publish the approved policy at a real public HTTPS URL and enter that exact URL. |

## Account-owner actions

1. Review the final signed Android package and complete runtime network test.
2. Complete every Data safety question in Play Console from the resulting evidence.
3. Publish the approved privacy policy and enter its public HTTPS URL.
4. Verify the Data safety form against every testing and production artifact.

The app targets API 36. Google Play's current requirement for new apps and updates from 2026-08-31 is API 36 or higher: [target API requirement](https://developer.android.com/google/play/requirements/target-sdk?authuser=383).
