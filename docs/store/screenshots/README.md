# Store screenshots

Status: BLOCKED. No screenshots are included because neither platform has a usable runtime available for capturing the actual app in this environment.

Capture screenshots from the final built apps after installing and launching them on supported iOS and Android devices or simulators. Include the six field choices, phase controls, plan and section slices, chart, provenance screen, and volume view. Verify legibility and accessibility at each selected store size. Do not use mockups or generated images as product screenshots.

Known runtime blockers:

- `xcrun simctl list runtimes` reports no registered iOS runtime. The Simulator app is reported by the user as installed, but runtime profiles are absent.
- Android Emulator 37.2.12 reports 2052.19 MB available and requires 7372.80 MB to create userdata for the API 36 arm64 AVD.
