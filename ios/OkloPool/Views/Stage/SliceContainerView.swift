import SwiftUI

public struct SliceContainerView: View {
    @Bindable var model: OkloAppModel
    @State private var inspectedI: Int = 16
    @State private var inspectedJ: Int = 15
    @State private var inspectedK: Int = 12

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 14) {
            // Adaptive Grid: side-by-side on wide layouts, stacked on narrow layouts
            ViewThatFits(in: .horizontal) {
                HStack(alignment: .top, spacing: 16) {
                    planSliceBlock
                    sectionSliceBlock
                }
                VStack(spacing: 16) {
                    planSliceBlock
                    sectionSliceBlock
                }
            }

            // Voxel Inspector Readout Bar
            voxelInspectorBar

            // Coordinate note
            Text("k increases upward, matching standard structural section orientation. All values are prescribed functions of position and phase.")
                .font(.caption2)
                .foregroundStyle(.secondary)
        }
    }

    private var planSliceBlock: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text("Plan Slice (X by Y)")
                    .font(.subheadline)
                    .fontWeight(.semibold)
                Spacer()
                Text("k = \(model.kDepthSlice)")
                    .font(.caption)
                    .fontDesign(.monospaced)
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(Color(uiColor: .tertiarySystemFill))
                    .cornerRadius(4)
            }

            PlanSliceView(
                model: model,
                inspectedI: $inspectedI,
                inspectedJ: $inspectedJ
            )

            HStack {
                Text("Depth k:")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Slider(
                    value: Binding(
                        get: { Double(model.kDepthSlice) },
                        set: { model.setKDepth(Int(round($0))) }
                    ),
                    in: 0...31,
                    step: 1
                )
                .accessibilityLabel("Depth slice slider")
                .accessibilityValue("Level \(model.kDepthSlice) of 32")
            }
        }
    }

    private var sectionSliceBlock: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text("Section Slice (X by k)")
                    .font(.subheadline)
                    .fontWeight(.semibold)
                Spacer()
                Text("j = \(model.jSectionRow)")
                    .font(.caption)
                    .fontDesign(.monospaced)
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(Color(uiColor: .tertiarySystemFill))
                    .cornerRadius(4)
            }

            SectionSliceView(
                model: model,
                inspectedI: $inspectedI,
                inspectedK: $inspectedK
            )

            HStack {
                Text("Section j:")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Slider(
                    value: Binding(
                        get: { Double(model.jSectionRow) },
                        set: { model.setJSection(Int(round($0))) }
                    ),
                    in: 0...31,
                    step: 1
                )
                .accessibilityLabel("Section row slider")
                .accessibilityValue("Row \(model.jSectionRow) of 32")
            }
        }
    }

    private var voxelInspectorBar: some View {
        let val = model.voxelValueAt(i: inspectedI, j: inspectedJ, k: inspectedK)
        let meta = model.currentFieldMetadata

        return HStack(spacing: 8) {
            Image(systemName: "cross.circle")
                .font(.footnote)
                .foregroundStyle(.secondary)

            Text("Voxel (\(inspectedI), \(inspectedJ), \(inspectedK)):")
                .font(.caption)
                .foregroundStyle(.secondary)

            if let val = val {
                if model.selectedField == .compartments {
                    let zoneIdx = Int(round(val))
                    let zoneMeta = FieldCatalog.zone(for: zoneIdx)
                    HStack(spacing: 4) {
                        Text("Zone \(zoneIdx)")
                            .font(.caption)
                            .fontWeight(.semibold)
                        if let z = zoneMeta {
                            Text("(\(z.name))")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                    }
                } else {
                    Text(String(format: "%.\(meta.digits)f %@", val, meta.shortUnit))
                        .font(.caption)
                        .fontWeight(.semibold)
                        .fontDesign(.monospaced)
                }
            } else {
                Text("No data")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            Spacer()
        }
        .padding(.horizontal, 10)
        .padding(.vertical, 6)
        .background(Color(uiColor: .secondarySystemBackground))
        .cornerRadius(6)
        .accessibilityElement(children: .combine)
        .accessibilityLabel("Voxel coordinates \(inspectedI), \(inspectedJ), \(inspectedK)")
    }
}
