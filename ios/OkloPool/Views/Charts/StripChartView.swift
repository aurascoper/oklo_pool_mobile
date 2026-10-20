import SwiftUI
import Charts
@preconcurrency import OkloCore

public struct StripChartView: View {
    @Bindable var model: OkloAppModel

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Prescribed Field Strip Chart")
                    .font(.headline)
                Spacer()
                Text("Native SwiftUI Charts")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }

            // Top Chart: Field Statistics over the Cycle
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(model.selectedField == .compartments
                         ? "Zone Shares (Fracture conduit & Steam chimney)"
                         : "\(model.currentFieldMetadata.displayName): Mean and Envelope [Min, Max]")
                        .font(.caption)
                        .fontWeight(.semibold)
                    Spacer()
                    Text("Unit: \(model.currentFieldMetadata.shortUnit)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                fieldChart
                    .frame(height: 160)
            }

            // Bottom Chart: Prescribed Pulse P and Desiccation D Drivers
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    HStack(spacing: 4) {
                        Circle().fill(Color.orange).frame(width: 8, height: 8)
                        Text("Pulse P(p)").font(.caption2).fontWeight(.semibold)
                    }
                    HStack(spacing: 4) {
                        Circle().fill(Color.cyan).frame(width: 8, height: 8)
                        Text("Desiccation D(p)").font(.caption2).fontWeight(.semibold)
                    }
                    Spacer()
                    Text("Prescribed Drivers (0.0 to 1.0)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                driversChart
                    .frame(height: 100)
            }

            // Explanatory note
            Text("Prescribed curves of phase, not simulated dynamics. In revision 0 the geyser exists only as two curves, the pulse P and the desiccation D. Hydration is a prescribed shape and the steam chimney is a zone label; no velocity, pressure or flow field is computed. Drag or tap the chart to scrub phase.")
                .font(.caption2)
                .foregroundStyle(.secondary)
        }
        .padding(12)
        .background(Color(uiColor: .secondarySystemBackground))
        .cornerRadius(10)
    }

    @ViewBuilder
    private var fieldChart: some View {
        let stats = model.seriesStats?.stats[model.selectedField] ?? []
        let currentMinutes = model.phase * (PrescribedCurves.cyclePeriodSeconds / 60.0)

        Chart {
            if model.selectedField == .compartments {
                ForEach(stats) { item in
                    LineMark(
                        x: .value("Time", item.timeMinutes),
                        y: .value("Conduit %", item.zone3Fraction * 100.0),
                        series: .value("Zone", "Fracture Conduit")
                    )
                    .foregroundStyle(Color(hex: "#3C7FC0"))
                    .lineStyle(StrokeStyle(lineWidth: 2))

                    LineMark(
                        x: .value("Time", item.timeMinutes),
                        y: .value("Chimney %", item.zone4Fraction * 100.0),
                        series: .value("Zone", "Steam Chimney")
                    )
                    .foregroundStyle(Color.orange)
                    .lineStyle(StrokeStyle(lineWidth: 2))
                }
            } else {
                ForEach(stats) { item in
                    AreaMark(
                        x: .value("Time", item.timeMinutes),
                        yStart: .value("Min", item.minVal),
                        yEnd: .value("Max", item.maxVal)
                    )
                    .foregroundStyle(Color.accentColor.opacity(0.2))

                    LineMark(
                        x: .value("Time", item.timeMinutes),
                        y: .value("Mean", item.meanVal)
                    )
                    .foregroundStyle(Color.accentColor)
                    .lineStyle(StrokeStyle(lineWidth: 2))
                }
            }

            // Vertical cursor indicating current phase
            RuleMark(x: .value("Current Time", currentMinutes))
                .foregroundStyle(Color.primary.opacity(0.8))
                .lineStyle(StrokeStyle(lineWidth: 1.5, dash: [4, 4]))
        }
        .chartXScale(domain: 0...180)
        .chartXAxis {
            AxisMarks(values: [0, 30, 60, 90, 120, 150, 180]) { val in
                AxisGridLine()
                AxisTick()
                AxisValueLabel {
                    if let m = val.as(Int.self) {
                        Text("\(m)m")
                            .font(.caption2)
                    }
                }
            }
        }
        .chartOverlay { proxy in
            GeometryReader { geo in
                Rectangle()
                    .fill(Color.clear)
                    .contentShape(Rectangle())
                    .onTapGesture { location in
                        updatePhaseFromChartLocation(location.x, proxy: proxy, geo: geo)
                    }
            }
        }
    }

    @ViewBuilder
    private var driversChart: some View {
        let drivers = model.seriesStats?.driverCurve ?? []
        let currentMinutes = model.phase * (PrescribedCurves.cyclePeriodSeconds / 60.0)

        Chart {
            ForEach(drivers) { pt in
                LineMark(
                    x: .value("Time", pt.timeMinutes),
                    y: .value("Driver", pt.pulse),
                    series: .value("Series", "Pulse P")
                )
                .foregroundStyle(Color.orange)
                .lineStyle(StrokeStyle(lineWidth: 2))

                LineMark(
                    x: .value("Time", pt.timeMinutes),
                    y: .value("Driver", pt.desiccation),
                    series: .value("Series", "Desiccation D")
                )
                .foregroundStyle(Color.cyan)
                .lineStyle(StrokeStyle(lineWidth: 2))
            }

            RuleMark(x: .value("Current Time", currentMinutes))
                .foregroundStyle(Color.primary.opacity(0.8))
                .lineStyle(StrokeStyle(lineWidth: 1.5, dash: [4, 4]))
        }
        .chartXScale(domain: 0...180)
        .chartYScale(domain: 0...1.05)
        .chartXAxis {
            AxisMarks(values: [0, 30, 60, 90, 120, 150, 180]) { val in
                AxisGridLine()
                AxisTick()
                AxisValueLabel {
                    if let m = val.as(Int.self) {
                        Text("\(m)m")
                            .font(.caption2)
                    }
                }
            }
        }
        .chartOverlay { proxy in
            GeometryReader { geo in
                Rectangle()
                    .fill(Color.clear)
                    .contentShape(Rectangle())
                    .onTapGesture { location in
                        updatePhaseFromChartLocation(location.x, proxy: proxy, geo: geo)
                    }
            }
        }
    }

    private func updatePhaseFromChartLocation(_ xLoc: CGFloat, proxy: ChartProxy, geo: GeometryProxy) {
        guard let plotFrame = proxy.plotFrame else { return }
        let relativeX = xLoc - geo[plotFrame].origin.x
        let plotWidth = geo[plotFrame].width
        guard plotWidth > 0 else { return }

        let frac = max(0.0, min(1.0, Double(relativeX / plotWidth)))
        model.setPhase(frac)
    }
}
