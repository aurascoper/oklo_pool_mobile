import SwiftUI

public struct PhaseControlView: View {
    @Bindable var model: OkloAppModel

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                Text("Phase Navigation")
                    .font(.headline)
                Spacer()
                // Phase Region Badge
                Text(model.currentPhaseRegion)
                    .font(.caption2)
                    .fontWeight(.semibold)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 3)
                    .background(phaseBadgeBackground)
                    .cornerRadius(6)
            }

            // Transport controls (Step back, Play/Pause, Step forward) and Time readout
            HStack(spacing: 12) {
                Button(action: {
                    model.stepFrame(forward: false)
                }) {
                    Image(systemName: "backward.frame.fill")
                        .font(.title3)
                }
                .buttonStyle(.bordered)
                .accessibilityLabel("Step back one frame")

                Button(action: {
                    model.togglePlayback()
                }) {
                    HStack(spacing: 6) {
                        Image(systemName: model.isPlaying ? "pause.fill" : "play.fill")
                        Text(model.isPlaying ? "Pause" : "Play")
                            .fontWeight(.semibold)
                    }
                    .frame(minWidth: 80)
                }
                .buttonStyle(.borderedProminent)
                .accessibilityLabel(model.isPlaying ? "Pause cycle playback" : "Start cycle playback")

                Button(action: {
                    model.stepFrame(forward: true)
                }) {
                    Image(systemName: "forward.frame.fill")
                        .font(.title3)
                }
                .buttonStyle(.bordered)
                .accessibilityLabel("Step forward one frame")

                Spacer()

                VStack(alignment: .trailing, spacing: 2) {
                    Text(model.elapsedTimeMinutesString)
                        .font(.subheadline)
                        .fontWeight(.bold)
                        .fontDesign(.monospaced)
                    Text("Frame \(model.currentFrameIndex) of 60")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .fontDesign(.monospaced)
                }
            }

            // Scrubbing Slider
            VStack(spacing: 2) {
                Slider(
                    value: Binding(
                        get: { Double(model.currentFrameIndex) },
                        set: { model.setFrame(Int(round($0))) }
                    ),
                    in: 0...59,
                    step: 1
                )
                .accessibilityLabel("Cycle phase timeline slider")
                .accessibilityValue("Frame \(model.currentFrameIndex) of 60, \(model.elapsedTimeMinutesString)")

                HStack {
                    Text("0 min (0%)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                    Spacer()
                    Text(String(format: "Phase: %.1f%%", model.phase * 100.0))
                        .font(.caption2)
                        .fontDesign(.monospaced)
                        .foregroundStyle(.secondary)
                    Spacer()
                    Text("180 min (100%)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
        }
        .padding(12)
        .background(Color(uiColor: .secondarySystemBackground))
        .cornerRadius(10)
    }

    private var phaseBadgeBackground: some View {
        let idx = PrescribedCurves.phaseRegionIndex(
            phase: model.phase,
            pulseBoundary: model.pulseBoundary
        )
        switch idx {
        case 0:
            return Color.orange.opacity(0.2)
        case 1:
            return Color.yellow.opacity(0.2)
        case 2:
            return Color.blue.opacity(0.15)
        default:
            return Color.green.opacity(0.15)
        }
    }
}
