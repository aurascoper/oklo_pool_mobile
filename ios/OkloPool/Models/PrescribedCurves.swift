import Foundation
import SwiftUI
@preconcurrency import OkloCore

public struct FrameStat: Sendable, Identifiable {
    public let id: Int
    public let phase: Double
    public let timeMinutes: Double
    public let minVal: Double
    public let meanVal: Double
    public let maxVal: Double
    public let zone3Fraction: Double
    public let zone4Fraction: Double

    public init(
        id: Int,
        phase: Double,
        timeMinutes: Double,
        minVal: Double,
        meanVal: Double,
        maxVal: Double,
        zone3Fraction: Double,
        zone4Fraction: Double
    ) {
        self.id = id
        self.phase = phase
        self.timeMinutes = timeMinutes
        self.minVal = minVal
        self.meanVal = meanVal
        self.maxVal = maxVal
        self.zone3Fraction = zone3Fraction
        self.zone4Fraction = zone4Fraction
    }
}

public struct DriverCurvePoint: Sendable, Identifiable {
    public let id: Int
    public let phase: Double
    public let timeMinutes: Double
    public let pulse: Double
    public let desiccation: Double

    public init(id: Int, phase: Double, timeMinutes: Double, pulse: Double, desiccation: Double) {
        self.id = id
        self.phase = phase
        self.timeMinutes = timeMinutes
        self.pulse = pulse
        self.desiccation = desiccation
    }
}

public struct FieldSeriesStats: Sendable {
    public let stats: [OkloSim.Field: [FrameStat]]
    public let overallRange: [OkloSim.Field: (min: Double, max: Double)]
    public let driverCurve: [DriverCurvePoint]

    public init(
        stats: [OkloSim.Field: [FrameStat]],
        overallRange: [OkloSim.Field: (min: Double, max: Double)],
        driverCurve: [DriverCurvePoint]
    ) {
        self.stats = stats
        self.overallRange = overallRange
        self.driverCurve = driverCurve
    }
}

public enum PrescribedCurves {
    public static let totalFrames: Int = 60
    public static let cyclePeriodSeconds: Double = 10800.0 // 3 hours
    public static let frameStepSeconds: Double = 180.0     // 3 minutes

    public static func pulse(phase: Double, pulseBoundary: Double) -> Double {
        if phase < pulseBoundary {
            return sin((phase / pulseBoundary) * Double.pi)
        } else {
            return max(0.0, exp(-8.0 * (phase - pulseBoundary)))
        }
    }

    public static func desiccation(phase: Double) -> Double {
        if phase >= 0.10 && phase <= 0.35 {
            return sin(((phase - 0.10) / 0.25) * Double.pi)
        } else {
            return 0.0
        }
    }

    public static func phaseRegionName(phase: Double, pulseBoundary: Double) -> String {
        let t1 = pulseBoundary
        let t2 = 0.25
        let t3 = (abs(pulseBoundary - 0.17) < 0.001) ? 0.83 : 0.8333

        if phase < t1 {
            return "Pulse/Critical Boiling"
        } else if phase < t2 {
            return "Desiccation/Dry Bake"
        } else if phase < t3 {
            return "Quiescent Cooldown"
        } else {
            return "Infiltration/Re-wetting"
        }
    }

    public static func phaseRegionIndex(phase: Double, pulseBoundary: Double) -> Int {
        let t1 = pulseBoundary
        let t2 = 0.25
        let t3 = (abs(pulseBoundary - 0.17) < 0.001) ? 0.83 : 0.8333

        if phase < t1 {
            return 0
        } else if phase < t2 {
            return 1
        } else if phase < t3 {
            return 2
        } else {
            return 3
        }
    }

    public static func computeSeries(pulseBoundary: Double, nullScenario: Bool) -> FieldSeriesStats {
        let totalVoxels: Double = 32.0 * 32.0 * 32.0
        var statsMap: [OkloSim.Field: [FrameStat]] = [:]
        var rangeMap: [OkloSim.Field: (min: Double, max: Double)] = [:]

        for field in OkloSim.Field.allCases {
            statsMap[field] = []
            rangeMap[field] = (min: Double.greatestFiniteMagnitude, max: -Double.greatestFiniteMagnitude)
        }

        // Generate genuine 60-frame series using OkloSim
        for frame in 0..<totalFrames {
            let p = Double(frame) / Double(totalFrames)
            let timeMin = p * (cyclePeriodSeconds / 60.0)

            let cfg = OkloSim.Config(
                nx: 32, ny: 32, nz: 32,
                phase: p,
                nullScenario: nullScenario,
                pulseBoundary: pulseBoundary,
                cyclePeriodSeconds: cyclePeriodSeconds
            )

            guard let sim = OkloSim(cfg) else { continue }

            for field in OkloSim.Field.allCases {
                let voxels = sim.field(field)
                var mn = Double.greatestFiniteMagnitude
                var mx = -Double.greatestFiniteMagnitude
                var sum: Double = 0.0
                var zone3Count = 0
                var zone4Count = 0

                for v in voxels {
                    let d = Double(v)
                    if d < mn { mn = d }
                    if d > mx { mx = d }
                    sum += d

                    if field == .compartments {
                        let z = Int(round(v))
                        if z == 3 { zone3Count += 1 }
                        if z == 4 { zone4Count += 1 }
                    }
                }

                let meanVal = voxels.isEmpty ? 0.0 : sum / Double(voxels.count)
                let z3Frac = Double(zone3Count) / totalVoxels
                let z4Frac = Double(zone4Count) / totalVoxels

                let stat = FrameStat(
                    id: frame,
                    phase: p,
                    timeMinutes: timeMin,
                    minVal: mn,
                    meanVal: meanVal,
                    maxVal: mx,
                    zone3Fraction: z3Frac,
                    zone4Fraction: z4Frac
                )
                statsMap[field]?.append(stat)

                if let currentRange = rangeMap[field] {
                    rangeMap[field] = (
                        min: min(currentRange.min, mn),
                        max: max(currentRange.max, mx)
                    )
                }
            }
        }

        // Prescribed driver curve across 120 points for smooth charts
        var drivers: [DriverCurvePoint] = []
        for i in 0...120 {
            let p = Double(i) / 120.0
            let timeMin = p * (cyclePeriodSeconds / 60.0)
            let pVal = nullScenario ? 0.0 : pulse(phase: p, pulseBoundary: pulseBoundary)
            let dVal = nullScenario ? 0.0 : desiccation(phase: p)
            drivers.append(DriverCurvePoint(
                id: i,
                phase: p,
                timeMinutes: timeMin,
                pulse: pVal,
                desiccation: dVal
            ))
        }

        return FieldSeriesStats(stats: statsMap, overallRange: rangeMap, driverCurve: drivers)
    }
}
