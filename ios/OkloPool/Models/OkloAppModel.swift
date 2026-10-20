import Foundation
import CoreGraphics
import SwiftUI
@preconcurrency import OkloCore

public enum ViewMode: String, CaseIterable, Identifiable, Sendable {
    case slices = "Slices"
    case volume = "3D Volume"

    public var id: String { rawValue }
}

@Observable
@MainActor
public final class OkloAppModel {
    public var selectedField: OkloSim.Field = .temperature
    public var phase: Double = 0.15
    public var pulseBoundary: Double = 0.17
    public var nullScenario: Bool = false
    public var kDepthSlice: Int = 12
    public var jSectionRow: Int = 15
    public var isPlaying: Bool = false
    public var viewMode: ViewMode = .slices
    public var volumeOpacity: Float = 1.0
    public var showCrosshairs: Bool = true
    public var showProvenanceSheet: Bool = false
    public var errorMessage: String? = nil

    public private(set) var currentFieldData: [Float] = []
    public private(set) var planSliceImage: CGImage? = nil
    public private(set) var sectionSliceImage: CGImage? = nil
    public private(set) var seriesStats: FieldSeriesStats? = nil

    private var sim: OkloSim? = nil
    private var playbackTask: Task<Void, Never>? = nil
    private var statsTask: Task<Void, Never>? = nil

    private var seriesCache: [String: FieldSeriesStats] = [:]

    public init() {
        refreshSeriesStats()
        rebuildSim()
    }

    isolated deinit {
        playbackTask?.cancel()
        statsTask?.cancel()
    }

    public var currentFrameIndex: Int {
        let idx = Int(round(phase * Double(PrescribedCurves.totalFrames)))
        return (idx % PrescribedCurves.totalFrames + PrescribedCurves.totalFrames) % PrescribedCurves.totalFrames
    }

    public var elapsedTimeSeconds: Double {
        phase * PrescribedCurves.cyclePeriodSeconds
    }

    public var elapsedTimeFormatted: String {
        let totalSec = Int(elapsedTimeSeconds)
        let hours = totalSec / 3600
        let minutes = (totalSec % 3600) / 60
        let seconds = totalSec % 60
        return String(format: "%02d:%02d:%02d / 03:00:00", hours, minutes, seconds)
    }

    public var elapsedTimeMinutesString: String {
        let mins = Int(round(elapsedTimeSeconds / 60.0))
        return "\(mins) min"
    }

    public var currentPhaseRegion: String {
        PrescribedCurves.phaseRegionName(phase: phase, pulseBoundary: pulseBoundary)
    }

    public var currentFieldMetadata: FieldMetadata {
        FieldCatalog.metadata(for: selectedField)
    }

    public var currentFieldRange: (min: Float, max: Float) {
        if !currentFieldData.isEmpty {
            var mn = Float.greatestFiniteMagnitude
            var mx = -Float.greatestFiniteMagnitude
            for v in currentFieldData {
                if v < mn { mn = v }
                if v > mx { mx = v }
            }
            if mn < mx {
                return (min: mn, max: mx)
            }
            if let r = seriesStats?.overallRange[selectedField] {
                return (min: Float(r.min), max: Float(r.max))
            }
            return (min: mn, max: mx)
        }
        if let r = seriesStats?.overallRange[selectedField] {
            return (min: Float(r.min), max: Float(r.max))
        }
        return (min: 0.0, max: 1.0)
    }

    public func rebuildSim() {
        let cfg = OkloSim.Config(
            nx: 32, ny: 32, nz: 32,
            phase: phase,
            nullScenario: nullScenario,
            pulseBoundary: pulseBoundary,
            cyclePeriodSeconds: PrescribedCurves.cyclePeriodSeconds
        )

        guard let newSim = OkloSim(cfg) else {
            errorMessage = "Core rejected configuration with pulse boundary \(pulseBoundary)."
            return
        }

        errorMessage = nil
        sim = newSim
        reloadFieldData()
    }

    public func reloadFieldData() {
        guard let sim = sim else { return }
        currentFieldData = sim.field(selectedField)
        updateSliceImages()
    }

    public func updateSliceImages() {
        let range = currentFieldRange
        planSliceImage = ColorRamps.createPlanSliceCGImage(
            data: currentFieldData,
            k: kDepthSlice,
            field: selectedField,
            range: range
        )
        sectionSliceImage = ColorRamps.createSectionSliceCGImage(
            data: currentFieldData,
            j: jSectionRow,
            field: selectedField,
            range: range
        )
    }

    public func selectField(_ field: OkloSim.Field) {
        selectedField = field
        reloadFieldData()
    }

    public func setPulseBoundary(_ boundary: Double) {
        guard boundary != pulseBoundary else { return }
        pulseBoundary = boundary
        refreshSeriesStats()
        rebuildSim()
    }

    public func setNullScenario(_ isNull: Bool) {
        guard isNull != nullScenario else { return }
        nullScenario = isNull
        refreshSeriesStats()
        rebuildSim()
    }

    public func setKDepth(_ k: Int) {
        let clamped = max(0, min(31, k))
        guard clamped != kDepthSlice else { return }
        kDepthSlice = clamped
        planSliceImage = ColorRamps.createPlanSliceCGImage(
            data: currentFieldData,
            k: kDepthSlice,
            field: selectedField,
            range: currentFieldRange
        )
    }

    public func setJSection(_ j: Int) {
        let clamped = max(0, min(31, j))
        guard clamped != jSectionRow else { return }
        jSectionRow = clamped
        sectionSliceImage = ColorRamps.createSectionSliceCGImage(
            data: currentFieldData,
            j: jSectionRow,
            field: selectedField,
            range: currentFieldRange
        )
    }

    public func setFrame(_ frameIndex: Int) {
        let clamped = max(0, min(PrescribedCurves.totalFrames - 1, frameIndex))
        let targetPhase = Double(clamped) / Double(PrescribedCurves.totalFrames)
        setPhase(targetPhase)
    }

    public func setPhase(_ newPhase: Double) {
        var p = newPhase.truncatingRemainder(dividingBy: 1.0)
        if p < 0.0 { p += 1.0 }
        phase = p
        rebuildSim()
    }

    public func stepFrame(forward: Bool) {
        let stepDelta = forward ? PrescribedCurves.frameStepSeconds : -PrescribedCurves.frameStepSeconds
        let currentP = phase
        let deltaP = stepDelta / PrescribedCurves.cyclePeriodSeconds
        var nextP = (currentP + deltaP).truncatingRemainder(dividingBy: 1.0)
        if nextP < 0.0 { nextP += 1.0 }
        setPhase(nextP)
    }

    public func togglePlayback() {
        if isPlaying {
            pause()
        } else {
            startPlayback()
        }
    }

    public func startPlayback() {
        isPlaying = true
        playbackTask?.cancel()
        playbackTask = Task { [weak self] in
            while !Task.isCancelled {
                try? await Task.sleep(nanoseconds: 250_000_000)
                guard let self = self else { break }
                if !self.isPlaying { break }
                self.stepFrame(forward: true)
            }
        }
    }

    public func pause() {
        isPlaying = false
        playbackTask?.cancel()
        playbackTask = nil
    }

    public func resetToDefaults() {
        pause()
        selectedField = .temperature
        phase = 0.15
        pulseBoundary = 0.17
        nullScenario = false
        kDepthSlice = 12
        jSectionRow = 15
        viewMode = .slices
        volumeOpacity = 1.0
        refreshSeriesStats()
        rebuildSim()
    }

    public func voxelValueAt(i: Int, j: Int, k: Int) -> Float? {
        guard i >= 0, i < 32, j >= 0, j < 32, k >= 0, k < 32 else { return nil }
        let idx = k * 32 * 32 + j * 32 + i
        guard idx < currentFieldData.count else { return nil }
        return currentFieldData[idx]
    }

    private func refreshSeriesStats() {
        let cacheKey = "\(pulseBoundary)|\(nullScenario)"
        if let cached = seriesCache[cacheKey] {
            statsTask?.cancel()
            seriesStats = cached
            updateSliceImages()
            return
        }
        statsTask?.cancel()
        seriesStats = nil
        let requestedBoundary = pulseBoundary
        let requestedNullScenario = nullScenario
        statsTask = Task.detached(priority: .userInitiated) { [weak self] in
            let computed = PrescribedCurves.computeSeries(
                pulseBoundary: requestedBoundary,
                nullScenario: requestedNullScenario
            )
            guard !Task.isCancelled else { return }
            await self?.installSeriesStats(
                computed,
                cacheKey: cacheKey,
                pulseBoundary: requestedBoundary,
                nullScenario: requestedNullScenario
            )
        }
    }

    private func installSeriesStats(
        _ stats: FieldSeriesStats,
        cacheKey: String,
        pulseBoundary: Double,
        nullScenario: Bool
    ) {
        guard self.pulseBoundary == pulseBoundary, self.nullScenario == nullScenario else { return }
        seriesCache[cacheKey] = stats
        seriesStats = stats
        updateSliceImages()
    }
}
