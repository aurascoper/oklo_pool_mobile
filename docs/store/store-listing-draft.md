# Store listing and content-rating inputs

Status: draft. The account owner must check each store's current field limits and enter the final listing.

## Proposed listing

**Name:** Oklo Fixture Viewer

**Short description:** Explore six fields from a documented Oklo natural reactor fixture.

**Description:**

Oklo Fixture Viewer is a scientific visualization app for a revision 0, six-field Oklo natural reactor fixture. Select a field, navigate prescribed phases, choose a pulse boundary or null scenario, and inspect plan and section slices, a volume view, and field charts.

The app displays a persistent initial-screen disclaimer across all views:
"Revision-0 prescribed fields; not a physical simulation."

The viewer renders prescribed analytic fixture functions. Phase navigation changes the displayed fixture values; it does not run an independently simulated physical model. Each active field renders an adjacent limitation statement, and all six field limitation records are fully documented in the in-app Scope and Provenance view:

1. Temperature: Prescribed thermal profile; not coupled to hydrodynamic or convective cooling.
2. Hydration: Prescribed saturation index; not a multiphase porous flow solve.
3. H2 Concentration: Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve.
4. 235U/238U Ratio: Prescribed isotopic depletion curve; not a 3D neutron transport solve.
5. Polymer accumulation index (arbitrary units): Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.
6. Compartments: Prescribed geometric bounding cells; not a physical membrane simulation.

**Privacy and Data Handling:**
All calculations run entirely offline on device. The app collects zero user data, performs zero tracking, and includes zero networking or analytics libraries. A complete plain-text privacy policy is bundled in-app for offline review. The proposed public policy URL is https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html (URL remains unverified until the account owner confirms live hosting).

**Category suggestion:** Education or Reference. The store account owner selects the final category.

## Rating questionnaire facts

- Scientific visualization of a natural reactor fixture.
- No user accounts, user-generated content, social features, commerce, or in-app messaging.
- No violence, gambling, mature themes, or user-submitted media.
- No network access requested or used.
- Final age rating is not assigned here. The account owner must complete each store's current questionnaire.

## Support and publisher metadata

- Publisher name: enter the account owner's legal or public publisher name.
- Support email: enter a monitored support email.
- Support URL: provide a working public HTTPS support page.
- Privacy policy URL: publish the approved policy and enter its confirmed public HTTPS URL (proposed: https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html).

## Account-owner blocker checklist before publishing

1. App Store Connect account configuration, bundle ID registration, signing certificates, and live hosting confirmation of privacy policy.
2. Physical iOS and Android device validation (Apple Developer account, TestFlight, Google Play Console internal testing).
3. Android Emulator host disk space: host machine has <1 GiB free disk space, blocking oklo-api36 AVD startup (status: BLOCKED / UNRESOLVED per Launch Gates).
