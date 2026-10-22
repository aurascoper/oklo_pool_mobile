import SwiftUI
@preconcurrency import OkloCore

public struct ProvenanceSheetView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var isPolicyExpanded: Bool = false

    public init() {}

    private func fieldLimitationRecord(for field: OkloSim.Field) -> String {
        switch field {
        case .temperature:
            return "Prescribed thermal profile; not coupled to hydrodynamic or convective cooling."
        case .hydration:
            return "Prescribed saturation index; not a multiphase porous flow solve."
        case .h2Concentration:
            return "Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve."
        case .u235Ratio:
            return "Prescribed isotopic depletion curve; not a 3D neutron transport solve."
        case .polymerYield:
            return "Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations."
        case .compartments:
            return "Prescribed geometric bounding cells; not a physical membrane simulation."
        }
    }

    public var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    // Header Banner
                    HStack(alignment: .top, spacing: 12) {
                        Image(systemName: "info.circle.fill")
                            .font(.title)
                            .foregroundStyle(Color.accentColor)

                        VStack(alignment: .leading, spacing: 4) {
                            Text("Educational Scenario Explorer")
                                .font(.headline)
                            Text("Not a physical simulation or empirical reconstruction")
                                .font(.subheadline)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .padding()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(Color.accentColor.opacity(0.1))
                    .cornerRadius(10)

                    // Core Epistemic Disclosures (verbatim docs/PROVENANCE.md)
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Assumptions and Provenance")
                            .font(.title3)
                            .fontWeight(.bold)

                        Text("This app is an educational scenario explorer. Every number it draws comes from a fixed formula chosen to give a plausible shape, not from measurements of Oklo and not from a physical simulation. The formulas are frozen so that the app, the original web viewer and the exported data files can be checked against each other.")
                            .font(.body)

                        Text("The natural fission reactors at Oklo in Gabon operated roughly two billion years ago, long after the earliest evidence of life on Earth. The pool shown here is therefore an analog scenario for ideas about early chemistry, not a reconstruction of a place where life began. The idea that wet and dry cycles at a natural reactor could drive the joining of small molecules is a published hypothesis, not an established finding.")
                            .font(.body)

                        Text("The roughly three-hour on and off cycle comes from the xenon record studied by Meshik and colleagues in 2004, and it has been reproduced in a computer model by Ibekwe and colleagues in 2020. The hydrogen map shows where radiolysis would happen, not how much hydrogen formed. Temperatures, water content and molecule amounts are illustrative indices. Units are shown as the original viewer labelled them, and several of those units have no derivation yet.")
                            .font(.body)
                    }

                    Divider()

                    // Per-field limitation cards
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Field-by-Field Limitations")
                            .font(.title3)
                            .fontWeight(.bold)

                        ForEach(FieldCatalog.allFields) { meta in
                            VStack(alignment: .leading, spacing: 6) {
                                HStack {
                                    Text(meta.displayName)
                                        .font(.subheadline)
                                        .fontWeight(.bold)
                                    Spacer()
                                    Text(meta.shortUnit)
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }

                                Text("Limitation Record:")
                                    .font(.caption2)
                                    .fontWeight(.semibold)
                                    .foregroundStyle(.orange)
                                Text(fieldLimitationRecord(for: meta.field))
                                    .font(.caption)
                                    .fontWeight(.medium)
                                    .foregroundStyle(.primary)

                                Text("Labelled Unit: \(meta.canonicalUnit)")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)

                                Text("Literature Anchor:")
                                    .font(.caption2)
                                    .fontWeight(.semibold)
                                    .foregroundStyle(.secondary)
                                Text(meta.literatureAnchor)
                                    .font(.caption)
                                    .foregroundStyle(.primary)

                                Text("Operational Limitation:")
                                    .font(.caption2)
                                    .fontWeight(.semibold)
                                    .foregroundStyle(.orange)
                                Text(meta.limitationStatement)
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            .padding(12)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .background(Color(uiColor: .secondarySystemBackground))
                            .cornerRadius(8)
                        }
                    }

                    // Engine provenance note
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Engine Contract")
                            .font(.caption)
                            .fontWeight(.bold)
                            .foregroundStyle(.secondary)
                        Text("Engine: oklo_core C ABI version 1, compiled to native static library via OkloCoreKit. Grid resolution: 32 x 32 x 32 voxels. All values evaluated in double precision and stored in 32-bit float buffers.")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    .padding(.top, 4)

                    Divider()

                    // Privacy Policy and Data Handling
                    VStack(alignment: .leading, spacing: 14) {
                        Text("Privacy Policy & Data Handling")
                            .font(.title3)
                            .fontWeight(.bold)

                        // Audit Summary Card
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Audit Summary")
                                .font(.subheadline)
                                .fontWeight(.semibold)
                            Text("Local computation only, zero tracking, zero networking. Oklo Fixture Viewer performs all computation locally using the embedded native oklo_core engine. The application contains zero tracking, zero analytics SDKs, zero advertising SDKs, zero user account flows, and requests zero system permissions. Runtime socket inspections confirm zero open network sockets (ZERO_INET_SOCKETS).")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                        .padding(12)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(Color(uiColor: .secondarySystemBackground))
                        .cornerRadius(8)

                        // External Link & Verification Status
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Public Hosted Policy")
                                .font(.subheadline)
                                .fontWeight(.semibold)

                            Link(destination: URL(string: "https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html")!) {
                                HStack(spacing: 6) {
                                    Image(systemName: "safari")
                                    Text("https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html")
                                        .font(.caption)
                                        .underline()
                                    Image(systemName: "arrow.up.forward.square")
                                        .font(.caption2)
                                }
                                .foregroundStyle(Color.accentColor)
                            }

                            Text("URL remains unverified until the account owner confirms live hosting.")
                                .font(.caption2)
                                .fontWeight(.medium)
                                .foregroundStyle(.orange)
                        }
                        .padding(12)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(Color(uiColor: .secondarySystemBackground))
                        .cornerRadius(8)

                        // Bundled full-text policy for offline access
                        DisclosureGroup("Bundled Full-Text Privacy Policy (Offline Access)", isExpanded: $isPolicyExpanded) {
                            Text(bundledPrivacyPolicyText)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                                .padding(.top, 6)
                                .textSelection(.enabled)
                        }
                        .padding(12)
                        .background(Color(uiColor: .secondarySystemBackground))
                        .cornerRadius(8)
                    }
                }
                .padding()
            }
            .navigationTitle("Provenance & Limitations")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

private let bundledPrivacyPolicyText: String = """
Oklo Fixture Viewer - Offline Privacy Policy

What the app does:
Oklo Fixture Viewer is a free scientific visualization app that presents six fields from the revision 0 Oklo natural reactor conformance fixture. Users can select a field, change the displayed cycle phase, choose a pulse boundary (0.17 or 0.1667), toggle the null scenario, and inspect 2D plan and section slices, a 3D volume view, and prescribed field charts. All displayed values are evaluated locally from fixed analytic fixture equations. Phase navigation does not run a live physical dynamics simulation.

Data collection, sharing, and transmission:
Oklo Fixture Viewer does not collect, store, transmit, or share personal data, usage analytics, diagnostics, or device identifiers:
- The app has no user accounts, sign-in, cloud sync, analytics SDK, advertising SDK, crash-reporting service, or network client.
- All computations and rendering run locally on the user's device using the embedded oklo_core engine.
- On Android, the application manifest does not request the android.permission.INTERNET permission or any user-data permission.
- On iOS, the application declares no external package dependencies, no network client, and no sensitive system permission usage descriptions.
- Runtime verification on both iOS Simulator (iPhone 17, iOS 27.0) and Android Emulator (Android 16 / API 36) confirmed zero open network sockets during operation.

Device storage and permissions:
- The app does not persist personal information or user preferences to disk across launches.
- On Android, allowBackup is set to false, and the merged manifest contains only the AndroidX-generated signature-only permission used internally by AndroidX for non-exported dynamic receivers.
- On iOS, the app does not request access to location, camera, photos, contacts, microphone, tracking (IDFA), or local network resources.

Distribution platforms:
When you download or update Oklo Fixture Viewer through the Apple App Store or Google Play Store, Apple Inc. or Google LLC may collect or process installation and account information governed by their own privacy policies and terms of service. Oklo Fixture Viewer itself receives no personal data from those platforms.

Children's privacy:
Oklo Fixture Viewer does not collect personal information from anyone, including children.

Changes to this policy and contact:
If future versions of Oklo Fixture Viewer change how data is handled, this privacy policy will be updated prior to release.
"""
