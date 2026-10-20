import SwiftUI

public struct ContentView: View {
    @Bindable var model: OkloAppModel
    @Environment(\.horizontalSizeClass) private var horizontalSizeClass

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                persistentDisclaimerBanner
                    .padding(.horizontal)
                    .padding(.top, 4)

                Group {
                    if horizontalSizeClass == .regular {
                        // iPad or wide landscape layout
                        HStack(alignment: .top, spacing: 20) {
                            ScrollView {
                                VStack(spacing: 16) {
                                    viewModeSelector
                                    stageView
                                    selectedFieldLimitationView
                                    PhaseControlView(model: model)
                                }
                                .padding()
                            }
                            .frame(maxWidth: .infinity)

                            ScrollView {
                                VStack(spacing: 16) {
                                    FieldPickerView(model: model)
                                    StripChartView(model: model)
                                    ScenarioControlsView(model: model)
                                }
                                .padding()
                            }
                            .frame(maxWidth: .infinity)
                        }
                    } else {
                        // iPhone compact layout
                        ScrollView {
                            VStack(spacing: 16) {
                                if let error = model.errorMessage {
                                    errorBanner(error)
                                }

                                viewModeSelector
                                stageView
                                selectedFieldLimitationView
                                PhaseControlView(model: model)
                                FieldPickerView(model: model)
                                StripChartView(model: model)
                                ScenarioControlsView(model: model)
                            }
                            .padding()
                        }
                    }
                }
            }
            .navigationTitle("Oklo Pool")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Button(action: {
                        model.resetToDefaults()
                    }) {
                        Text("Reset")
                    }
                    .accessibilityLabel("Reset controls to default values")
                }

                ToolbarItem(placement: .topBarTrailing) {
                    Button(action: {
                        model.showProvenanceSheet = true
                    }) {
                        Image(systemName: "info.circle")
                    }
                    .accessibilityLabel("Open Provenance and Limitations")
                }
            }
            .sheet(isPresented: $model.showProvenanceSheet) {
                ProvenanceSheetView()
            }
        }
    }

    private var persistentDisclaimerBanner: some View {
        HStack(spacing: 8) {
            Image(systemName: "info.circle.fill")
                .foregroundStyle(.orange)
            Text("Revision-0 prescribed fields; not a physical simulation.")
                .font(.footnote)
                .fontWeight(.semibold)
                .foregroundStyle(.primary)
        }
        .padding(8)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.orange.opacity(0.12))
        .cornerRadius(8)
        .accessibilityIdentifier("persistentDisclaimerBanner")
    }

    private var selectedFieldLimitationView: some View {
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
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.orange.opacity(0.1))
        .cornerRadius(6)
        .accessibilityIdentifier("selectedFieldLimitation")
    }

    private var viewModeSelector: some View {
        Picker("View Mode", selection: $model.viewMode) {
            ForEach(ViewMode.allCases) { mode in
                Text(mode.rawValue).tag(mode)
            }
        }
        .pickerStyle(.segmented)
        .accessibilityLabel("View mode selector: Slices or 3D Volume")
    }

    @ViewBuilder
    private var stageView: some View {
        switch model.viewMode {
        case .slices:
            SliceContainerView(model: model)
        case .volume:
            VolumeRaymarcherView(model: model)
        }
    }

    private func errorBanner(_ message: String) -> some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundStyle(.white)
            Text(message)
                .font(.footnote)
                .foregroundStyle(.white)
            Spacer()
            Button("Dismiss") {
                model.errorMessage = nil
            }
            .buttonStyle(.bordered)
            .tint(.white)
        }
        .padding(10)
        .background(Color.red.opacity(0.85))
        .cornerRadius(8)
    }
}
