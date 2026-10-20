import SwiftUI

public struct ProvenanceSheetView: View {
    @Environment(\.dismiss) private var dismiss

    public init() {}

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
                    .padding(.top, 8)
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
