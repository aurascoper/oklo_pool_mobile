# Privacy policy draft: Oklo Fixture Viewer

Status: **Draft only. Do not publish or submit.** The source review found no app network client, analytics SDK, advertising SDK, account flow, or user-data permission. Runtime network behavior has not been tested on either platform. Complete that test and replace the support contact and public policy URL before publication.

Effective date: pending release owner approval
App: Oklo Fixture Viewer
Publisher: pending account owner
Support contact: enter a monitored support email before publication
Public policy URL: assign after this policy is hosted over HTTPS

## What the app does

Oklo Fixture Viewer presents six fields from the revision 0 Oklo fixture. The user can select a field, change the displayed phase, choose a pulse boundary, select a null scenario, and inspect plan, section, volume, and chart views. The displayed values come from prescribed fixture functions. Phase navigation does not run a physical dynamics model.

## Data handled by the app

The reviewed application source does not define an account, analytics, advertising, or cloud sync feature. The application source and direct dependencies reviewed for this draft do not define an app network client or a user-data permission. These are source-level findings only. A runtime network capture and installed-app review have not been completed, so this draft does not make a verified claim that the app collects or transmits no data.

Before publication, record whether any data is collected, shared, stored, or transmitted by the built applications or their dependencies. Update this policy and the store disclosures to match that evidence.

## Device storage and permissions

The reviewed Android manifest does not request internet access or user data access. Its merged manifest includes one AndroidX-generated signature-only permission for non-exported dynamic receivers. The reviewed iOS project does not declare a data-access usage description. Runtime behavior remains to be checked.

## Service providers and distribution platforms

The iOS app uses Apple platform frameworks and a local Rust core. The Android app uses AndroidX UI and lifecycle libraries, Kotlin coroutines, and the local Rust core through JNI. No analytics or advertising SDK is declared directly by either app. Complete the transitive dependency and runtime audit before publication.

Apple and Google may process information under their own terms when users obtain or update the app through their stores. The app publisher must complete the applicable store privacy disclosures.

## Children and age suitability

The publisher must complete the current age-rating questionnaire in each store. Do not infer an age rating from this draft.

## Changes and contact

Update this policy when the app's data practices change. Contact: enter a monitored support email before publication.

## Release checks still required

- Capture runtime network behavior on supported iOS and Android builds.
- Review the final dependency graphs and packaged SDKs.
- Enter a real support contact and publisher name.
- Publish this policy at a public HTTPS URL and enter that URL in the store consoles.
- Make the store privacy declarations from the verified final builds.
