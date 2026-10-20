import SwiftUI
@preconcurrency import OkloCore

public struct FieldPickerView: View {
    @Bindable var model: OkloAppModel

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Field Selection")
                .font(.headline)

            // 6-field grid / list
            LazyVGrid(columns: [GridItem(.adaptive(minimum: 150), spacing: 8)], spacing: 8) {
                ForEach(FieldCatalog.allFields) { meta in
                    let isSelected = model.selectedField == meta.field
                    Button(action: {
                        model.selectField(meta.field)
                    }) {
                        VStack(alignment: .leading, spacing: 3) {
                            HStack {
                                Text(meta.displayName)
                                    .font(.subheadline)
                                    .fontWeight(isSelected ? .bold : .medium)
                                Spacer()
                                if isSelected {
                                    Image(systemName: "checkmark.circle.fill")
                                        .font(.caption)
                                        .foregroundStyle(Color.accentColor)
                                }
                            }
                            Text(meta.shortUnit)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                        .padding(.horizontal, 10)
                        .padding(.vertical, 8)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(isSelected ? Color.accentColor.opacity(0.15) : Color(uiColor: .secondarySystemBackground))
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(isSelected ? Color.accentColor : Color.clear, lineWidth: 1.5)
                        )
                        .cornerRadius(8)
                    }
                    .buttonStyle(.plain)
                    .accessibilityLabel("\(meta.displayName), \(meta.shortUnit)")
                    .accessibilityValue(isSelected ? "Selected" : "Not selected")
                    .accessibilityHint("Double tap to select this field")
                }
            }

            // Legend / Colorbar
            if model.selectedField == .compartments {
                zonesLegend
            } else {
                continuousColorbar
            }

            // Limitation callout per docs/PROVENANCE.md
            HStack(alignment: .top, spacing: 8) {
                Image(systemName: "exclamationmark.triangle.fill")
                    .font(.caption)
                    .foregroundStyle(.orange)
                    .padding(.top, 2)
                Text(model.currentFieldMetadata.limitationStatement)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            .padding(8)
            .background(Color.orange.opacity(0.1))
            .cornerRadius(6)
        }
    }

    private var zonesLegend: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("Categorical Zones:")
                .font(.caption)
                .foregroundStyle(.secondary)

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 8) {
                    ForEach(FieldCatalog.zones) { z in
                        HStack(spacing: 5) {
                            Circle()
                                .fill(Color(hex: z.hexColor))
                                .frame(width: 10, height: 10)
                                .overlay(Circle().stroke(Color.primary.opacity(0.2), lineWidth: 0.5))
                            Text("\(z.id): \(z.name)")
                                .font(.caption2)
                        }
                        .padding(.horizontal, 6)
                        .padding(.vertical, 3)
                        .background(Color(uiColor: .tertiarySystemFill))
                        .cornerRadius(4)
                    }
                }
            }
        }
    }

    private var continuousColorbar: some View {
        let meta = model.currentFieldMetadata
        let range = model.currentFieldRange

        return VStack(spacing: 2) {
            GeometryReader { proxy in
                let w = proxy.size.width
                LinearGradient(
                    gradient: gradientFor(colormap: meta.colormapName),
                    startPoint: .leading,
                    endPoint: .trailing
                )
                .frame(width: w, height: 12)
                .cornerRadius(3)
                .overlay(RoundedRectangle(cornerRadius: 3).stroke(Color(uiColor: .separator), lineWidth: 1))
            }
            .frame(height: 12)

            HStack {
                Text(String(format: "%.\(meta.digits)f", range.min))
                    .font(.caption2)
                    .fontDesign(.monospaced)
                    .foregroundStyle(.secondary)
                Spacer()
                Text(meta.canonicalUnit)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)
                Spacer()
                Text(String(format: "%.\(meta.digits)f", range.max))
                    .font(.caption2)
                    .fontDesign(.monospaced)
                    .foregroundStyle(.secondary)
            }
        }
    }

    private func gradientFor(colormap: String) -> Gradient {
        let stops = ColorRamps.stops(for: colormap)
        let colors = stops.map { s in
            Color(red: Double(s.r) / 255.0, green: Double(s.g) / 255.0, blue: Double(s.b) / 255.0)
        }
        return Gradient(colors: colors)
    }
}

extension Color {
    init(hex: String) {
        var clean = hex.trimmingCharacters(in: .whitespacesAndNewlines)
        if clean.hasPrefix("#") {
            clean.removeFirst()
        }
        let val = UInt32(clean, radix: 16) ?? 0
        let r = Double((val >> 16) & 0xFF) / 255.0
        let g = Double((val >> 8) & 0xFF) / 255.0
        let b = Double(val & 0xFF) / 255.0
        self.init(red: r, green: g, blue: b)
    }
}
