import SwiftUI

public struct ContentView: View {
    @Bindable var model: OkloAppModel
    @Environment(\.horizontalSizeClass) private var horizontalSizeClass

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        NavigationStack {
            Group {
                if horizontalSizeClass == .regular {
                    // iPad or wide landscape layout
                    HStack(alignment: .top, spacing: 20) {
                        ScrollView {
                            VStack(spacing: 16) {
                                viewModeSelector
                                stageView
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
                            PhaseControlView(model: model)
                            FieldPickerView(model: model)
                            StripChartView(model: model)
                            ScenarioControlsView(model: model)
                        }
                        .padding()
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
