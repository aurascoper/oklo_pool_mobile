import SwiftUI
import CoreGraphics

public struct SectionSliceView: View {
    @Bindable var model: OkloAppModel
    @Binding var inspectedI: Int
    @Binding var inspectedK: Int

    public init(model: OkloAppModel, inspectedI: Binding<Int>, inspectedK: Binding<Int>) {
        self.model = model
        self._inspectedI = inspectedI
        self._inspectedK = inspectedK
    }

    public var body: some View {
        GeometryReader { proxy in
            let size = min(proxy.size.width, proxy.size.height)
            let side = max(1.0, size)

            ZStack(alignment: .topLeading) {
                // Background image of slice
                if let cgImage = model.sectionSliceImage {
                    Image(decorative: cgImage, scale: 1.0)
                        .interpolation(.none)
                        .resizable()
                        .frame(width: side, height: side)
                        .border(Color(uiColor: .separator), width: 1)
                } else {
                    Rectangle()
                        .fill(Color(uiColor: .secondarySystemBackground))
                        .frame(width: side, height: side)
                        .overlay(
                            Text("Loading slice...")
                                .font(.footnote)
                                .foregroundStyle(.secondary)
                        )
                }

                // High-contrast dual-stroke crosshair for plan cut level Z = k
                if model.showCrosshairs {
                    let normY = (31.0 - Double(model.kDepthSlice) + 0.5) / 32.0
                    let yPos = normY * side

                    // Outer white stroke
                    Path { path in
                        path.move(to: CGPoint(x: 0, y: yPos))
                        path.addLine(to: CGPoint(x: side, y: yPos))
                    }
                    .stroke(Color.white.opacity(0.85), lineWidth: 3)

                    // Inner dark stroke
                    Path { path in
                        path.move(to: CGPoint(x: 0, y: yPos))
                        path.addLine(to: CGPoint(x: side, y: yPos))
                    }
                    .stroke(Color.black.opacity(0.75), lineWidth: 1)
                }
            }
            .frame(width: side, height: side)
            .contentShape(Rectangle())
            .gesture(
                DragGesture(minimumDistance: 0)
                    .onChanged { value in
                        let clampedX = max(0.0, min(side, value.location.x))
                        let clampedY = max(0.0, min(side, value.location.y))
                        let normX = clampedX / side
                        let normY = 1.0 - (clampedY / side)
                        let newI = Int(round(normX * 31.0))
                        let newK = Int(round(normY * 31.0))
                        inspectedI = max(0, min(31, newI))
                        inspectedK = max(0, min(31, newK))
                        model.setKDepth(newK)
                    }
            )
            .accessibilityElement(children: .ignore)
            .accessibilityLabel("Section slice at row j = \(model.jSectionRow) of 32")
            .accessibilityValue("Crosshair depth level k = \(model.kDepthSlice) of 32")
            .accessibilityHint("Drag vertically to change depth level, horizontally to inspect voxels")
        }
        .aspectRatio(1, contentMode: .fit)
    }
}
