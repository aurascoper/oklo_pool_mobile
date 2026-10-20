import SwiftUI

public struct ScenarioControlsView: View {
    @Bindable var model: OkloAppModel

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Scenario Parameters")
                .font(.headline)

            // Pulse boundary selector
            VStack(alignment: .leading, spacing: 4) {
                Text("Pulse boundary b:")
                    .font(.caption)
                    .foregroundStyle(.secondary)

                Picker("Pulse boundary", selection: Binding(
                    get: { model.pulseBoundary },
                    set: { model.setPulseBoundary($0) }
                )) {
                    Text("0.17 (Stored files)").tag(0.17)
                    Text("0.1667 (Original GUI)").tag(0.1667)
                }
                .pickerStyle(.segmented)
                .accessibilityLabel("Pulse boundary selector")

                Text("Revision 0 has two values (finding R0-1). Neither is endorsed here.")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }

            Divider()

            // Null scenario toggle
            Toggle(isOn: Binding(
                get: { model.nullScenario },
                set: { model.setNullScenario($0) }
            )) {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Null Scenario")
                        .font(.subheadline)
                        .fontWeight(.medium)
                    Text("Zeroes pulse P(p) and desiccation D(p) drivers")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
            .accessibilityLabel("Null scenario toggle")

            Divider()

            // Crosshair overlay toggle
            Toggle(isOn: $model.showCrosshairs) {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Show Slice Crosshairs")
                        .font(.subheadline)
                        .fontWeight(.medium)
                    Text("High-contrast overlay indicating cut coordinates")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
            .accessibilityLabel("Show slice crosshairs toggle")
        }
        .padding(12)
        .background(Color(uiColor: .secondarySystemBackground))
        .cornerRadius(10)
    }
}
