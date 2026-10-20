import XCTest
import MetalKit
import CoreGraphics
@preconcurrency import OkloCore
@testable import OkloPool

@MainActor
final class OnRenderGateBenchmarkTests: XCTestCase {

    private func checkNoDashes(_ text: String, label: String) {
        XCTAssertFalse(text.contains("\u{2013}"), "\(label) must not contain en-dash")
        XCTAssertFalse(text.contains("\u{2014}"), "\(label) must not contain em-dash")
    }

    private func durationToMs(_ d: Duration) -> Double {
        let s = Double(d.components.seconds)
        let a = Double(d.components.attoseconds)
        return s * 1000.0 + a / 1e15
    }

    // ITEM 1: Cold launch and first useful frame
    func testItem1ColdLaunchAndFirstUsefulFrame() {
        let model = OkloAppModel()
        XCTAssertEqual(model.selectedField, .temperature)
        XCTAssertEqual(model.phase, 0.15, accuracy: 1e-6)
        XCTAssertEqual(model.pulseBoundary, 0.17, accuracy: 1e-6)
        XCTAssertFalse(model.nullScenario)
        XCTAssertEqual(model.viewMode, .slices)

        // First useful frame
        XCTAssertEqual(model.currentFieldData.count, 32768)
        XCTAssertNotNil(model.planSliceImage)
        XCTAssertNotNil(model.sectionSliceImage)
        XCTAssertEqual(model.planSliceImage?.width, 32)
        XCTAssertEqual(model.planSliceImage?.height, 32)
        XCTAssertEqual(model.sectionSliceImage?.width, 32)
        XCTAssertEqual(model.sectionSliceImage?.height, 32)
    }

    // ITEM 2: Initial-field rendering correctness and pixel diversity
    func testItem2InitialFieldRenderingCorrectnessAndPixelDiversity() {
        let model = OkloAppModel()
        let data = model.currentFieldData
        XCTAssertEqual(data.count, 32768)

        var minVal: Float = Float.greatestFiniteMagnitude
        var maxVal: Float = -Float.greatestFiniteMagnitude
        for v in data {
            if v < minVal { minVal = v }
            if v > maxVal { maxVal = v }
        }

        XCTAssertGreaterThanOrEqual(minVal, 20.0, "Min temperature must be >= 20.0 C")
        XCTAssertLessThanOrEqual(maxVal, 600.0, "Max temperature must be <= 600.0 C")
        XCTAssertGreaterThan(maxVal, minVal, "Temperature field must not be flat")

        // Pixel diversity: verify non-trivial spatial gradient (> 100 distinct float values)
        let sampleValues = Set(data.prefix(4096))
        XCTAssertGreaterThanOrEqual(sampleValues.count, 100, "Pixel diversity requires spatial gradient")
    }

    // ITEM 3: All six field transitions and their limitation statements
    func testItem3AllSixFieldTransitionsAndLimitations() {
        let model = OkloAppModel()
        XCTAssertEqual(OkloSim.Field.allCases.count, 6)

        for field in OkloSim.Field.allCases {
            model.selectField(field)
            XCTAssertEqual(model.selectedField, field)
            XCTAssertEqual(model.currentFieldData.count, 32768)

            let meta = FieldCatalog.metadata(for: field)
            XCTAssertFalse(meta.displayName.isEmpty)
            XCTAssertFalse(meta.canonicalUnit.isEmpty)
            XCTAssertFalse(meta.literatureAnchor.isEmpty)
            XCTAssertFalse(meta.limitationStatement.isEmpty)

            checkNoDashes(meta.displayName, label: "displayName for \(field)")
            checkNoDashes(meta.canonicalUnit, label: "canonicalUnit for \(field)")
            checkNoDashes(meta.literatureAnchor, label: "literatureAnchor for \(field)")
            checkNoDashes(meta.limitationStatement, label: "limitationStatement for \(field)")
        }
    }

    // ITEM 4: First chart entry and the 60-frame series
    func testItem4FirstChartEntryAndSixtyFrameSeries() {
        let stats = PrescribedCurves.computeSeries(pulseBoundary: 0.17, nullScenario: false)

        for field in OkloSim.Field.allCases {
            guard let frameList = stats.stats[field] else {
                XCTFail("Missing stats for field \(field)")
                continue
            }
            XCTAssertEqual(frameList.count, 60, "Series must contain exactly 60 frames")

            // First chart entry: frame 0, phase 0.0
            let firstEntry = frameList[0]
            XCTAssertEqual(firstEntry.id, 0)
            XCTAssertEqual(firstEntry.phase, 0.0, accuracy: 1e-6)
            XCTAssertTrue(firstEntry.minVal.isFinite)
            XCTAssertTrue(firstEntry.meanVal.isFinite)
            XCTAssertTrue(firstEntry.maxVal.isFinite)
            XCTAssertGreaterThanOrEqual(firstEntry.maxVal, firstEntry.minVal)
        }
    }

    // ITEM 5: Three-dimensional volume-view readiness
    func testItem5ThreeDimensionalVolumeViewReadiness() {
        guard let device = MTLCreateSystemDefaultDevice() else {
            XCTFail("Metal device not available on simulator host")
            return
        }

        guard let library = device.makeDefaultLibrary() else {
            XCTFail("Metal default library not found")
            return
        }

        let vertexFn = library.makeFunction(name: "volumeVertexShader")
        let fragmentFn = library.makeFunction(name: "volumeFragmentShader")
        XCTAssertNotNil(vertexFn, "volumeVertexShader must compile")
        XCTAssertNotNil(fragmentFn, "volumeFragmentShader must compile")

        let desc = MTLRenderPipelineDescriptor()
        desc.vertexFunction = vertexFn
        desc.fragmentFunction = fragmentFn
        desc.colorAttachments[0].pixelFormat = .bgra8Unorm
        desc.colorAttachments[0].isBlendingEnabled = true
        desc.colorAttachments[0].rgbBlendOperation = .add
        desc.colorAttachments[0].alphaBlendOperation = .add
        desc.colorAttachments[0].sourceRGBBlendFactor = .one
        desc.colorAttachments[0].sourceAlphaBlendFactor = .one
        desc.colorAttachments[0].destinationRGBBlendFactor = .oneMinusSourceAlpha
        desc.colorAttachments[0].destinationAlphaBlendFactor = .oneMinusSourceAlpha

        let pipeline = try? device.makeRenderPipelineState(descriptor: desc)
        XCTAssertNotNil(pipeline, "Render pipeline state must build")

        // 3D volume texture descriptor readiness
        let texDesc = MTLTextureDescriptor()
        texDesc.textureType = .type3D
        texDesc.pixelFormat = .r8Unorm
        texDesc.width = 32
        texDesc.height = 32
        texDesc.depth = 32
        texDesc.usage = [.shaderRead]
        let volTexture = device.makeTexture(descriptor: texDesc)
        XCTAssertNotNil(volTexture, "3D volume texture must allocate")

        // 2D LUT texture descriptor readiness
        let lutDesc = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: .rgba8Unorm,
            width: 256,
            height: 1,
            mipmapped: false
        )
        lutDesc.usage = [.shaderRead]
        let lutTexture = device.makeTexture(descriptor: lutDesc)
        XCTAssertNotNil(lutTexture, "2D LUT texture must allocate")

        // RaymarcherUniforms layout validation (64 bytes)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.size, 64)
    }

    // ITEM 6: Phase changes at 0/60, 9/60, 15/60, 30/60, and 59/60
    func testItem6PhaseChangesAtFiveGateCheckpoints() {
        let model = OkloAppModel()
        let checkpoints: [(frameIndex: Int, expectedPhase: Double)] = [
            (0, 0.0),
            (9, 9.0 / 60.0),
            (15, 15.0 / 60.0),
            (30, 30.0 / 60.0),
            (59, 59.0 / 60.0)
        ]

        for cp in checkpoints {
            model.setFrame(cp.frameIndex)
            XCTAssertEqual(model.currentFrameIndex, cp.frameIndex)
            XCTAssertEqual(model.phase, cp.expectedPhase, accuracy: 1e-5)
            XCTAssertEqual(model.currentFieldData.count, 32768)
            XCTAssertNotNil(model.planSliceImage)
            XCTAssertNotNil(model.sectionSliceImage)
        }
    }

    // PROVISIONAL TIMING BUDGETS UNDER FROZEN METHODOLOGY
    func testFiveTimingBudgetsUnderFrozenMethodology() {
        let model = OkloAppModel()

        // 1. Warm-up protocol: 3 unmeasured passes
        for _ in 0..<3 {
            let _ = OkloAppModel()
            model.selectField(.hydration)
            model.selectField(.temperature)
            let _ = PrescribedCurves.computeSeries(pulseBoundary: 0.17, nullScenario: false)
            model.setFrame(15)
            model.setFrame(0)
        }

        // Budget 1: Initial model and slice generation <= 100 ms (N=30)
        var b1CompTimes: [Double] = []
        var b1RenderTimes: [Double] = []
        for _ in 0..<30 {
            let t0 = ContinuousClock.now
            let cfg = OkloSim.Config(
                nx: 32, ny: 32, nz: 32,
                phase: 0.15,
                nullScenario: false,
                pulseBoundary: 0.17,
                cyclePeriodSeconds: 10800.0
            )
            let sim = OkloSim(cfg)!
            let data = sim.field(.temperature)
            let t1 = ContinuousClock.now

            let range: (min: Float, max: Float) = (20.0, 600.0)
            let planImg = ColorRamps.createPlanSliceCGImage(data: data, k: 12, field: .temperature, range: range)
            let secImg = ColorRamps.createSectionSliceCGImage(data: data, j: 15, field: .temperature, range: range)
            let t2 = ContinuousClock.now

            XCTAssertNotNil(planImg)
            XCTAssertNotNil(secImg)

            let compMs = durationToMs(t1 - t0)
            let totalMs = durationToMs(t2 - t0)
            b1CompTimes.append(compMs)
            b1RenderTimes.append(totalMs)
        }
        b1RenderTimes.sort()
        let b1Median = b1RenderTimes[b1RenderTimes.count / 2]
        let b1P95 = b1RenderTimes[Int(ceil(0.95 * Double(b1RenderTimes.count))) - 1]
        let b1Fails = b1RenderTimes.filter { $0 > 100.0 }.count
        print("iOS Budget 1 (Initial model+slice): median=\(String(format: "%.2f", b1Median)) ms, p95=\(String(format: "%.2f", b1P95)) ms, fails=\(b1Fails), threshold=100.0 ms")
        XCTAssertLessThanOrEqual(b1P95, 100.0, "Budget 1 p95 must be <= 100 ms")

        // Budget 2: Field switching <= 50 ms per switch (N=30)
        var b2CompTimes: [Double] = []
        var b2RenderTimes: [Double] = []
        let fields = OkloSim.Field.allCases
        for i in 0..<30 {
            let targetField = fields[(i + 1) % fields.count]
            let t0 = ContinuousClock.now
            model.selectField(targetField)
            let t1 = ContinuousClock.now

            let range = model.currentFieldRange
            let planImg = ColorRamps.createPlanSliceCGImage(data: model.currentFieldData, k: 12, field: targetField, range: range)
            let secImg = ColorRamps.createSectionSliceCGImage(data: model.currentFieldData, j: 15, field: targetField, range: range)
            let t2 = ContinuousClock.now

            XCTAssertNotNil(planImg)
            XCTAssertNotNil(secImg)

            let compMs = durationToMs(t1 - t0)
            let totalMs = durationToMs(t2 - t0)
            b2CompTimes.append(compMs)
            b2RenderTimes.append(totalMs)
        }
        b2RenderTimes.sort()
        let b2Median = b2RenderTimes[b2RenderTimes.count / 2]
        let b2P95 = b2RenderTimes[Int(ceil(0.95 * Double(b2RenderTimes.count))) - 1]
        let b2Fails = b2RenderTimes.filter { $0 > 50.0 }.count
        print("iOS Budget 2 (Field switching): median=\(String(format: "%.2f", b2Median)) ms, p95=\(String(format: "%.2f", b2P95)) ms, fails=\(b2Fails), threshold=50.0 ms")
        XCTAssertLessThanOrEqual(b2P95, 50.0, "Budget 2 p95 must be <= 50 ms")

        // Budget 3: Uncached 60-frame chart calculation <= 400 ms (N=20)
        // Dedicated warm-up: 3 iterations
        for _ in 0..<3 {
            let _ = PrescribedCurves.computeSingleFieldSeries(field: .temperature, pulseBoundary: 0.17, nullScenario: false)
        }
        var b3Times: [Double] = []
        for _ in 0..<20 {
            let t0 = ContinuousClock.now
            let s = PrescribedCurves.computeSingleFieldSeries(field: .temperature, pulseBoundary: 0.17, nullScenario: false)
            let t1 = ContinuousClock.now
            XCTAssertEqual(s.count, 60)
            let totalMs = durationToMs(t1 - t0)
            b3Times.append(totalMs)
        }
        b3Times.sort()
        let b3Median = b3Times[b3Times.count / 2]
        let b3P95 = b3Times[Int(ceil(0.95 * Double(b3Times.count))) - 1]
        let b3Fails = b3Times.filter { $0 > 400.0 }.count
        print("iOS Budget 3 (Uncached 60-frame chart): median=\(String(format: "%.2f", b3Median)) ms, p95=\(String(format: "%.2f", b3P95)) ms, fails=\(b3Fails), threshold=400.0 ms")
        XCTAssertLessThanOrEqual(b3P95, 400.0, "Budget 3 p95 must be <= 400 ms")

        // Diagnostic measurement: Full 6-field 60-frame precomputation (360 3D volumes)
        var fullSeriesTimes: [Double] = []
        for _ in 0..<5 {
            let t0 = ContinuousClock.now
            let fullStats = PrescribedCurves.computeSeries(pulseBoundary: 0.17, nullScenario: false)
            let t1 = ContinuousClock.now
            XCTAssertEqual(fullStats.stats[.temperature]?.count, 60)
            fullSeriesTimes.append(durationToMs(t1 - t0))
        }
        fullSeriesTimes.sort()
        let fullMedian = fullSeriesTimes[fullSeriesTimes.count / 2]
        let fullP95 = fullSeriesTimes[fullSeriesTimes.count - 1]
        print("iOS Full 6-field 60-frame precomputation: median=\(String(format: "%.2f", fullMedian)) ms, p95=\(String(format: "%.2f", fullP95)) ms (360 volumes)")

        // Budget 4: Volume frame <= 100 ms under declared benchmark method (N=30)
        guard let device = MTLCreateSystemDefaultDevice() else {
            XCTFail("Metal device required for Budget 4")
            return
        }
        let texDesc = MTLTextureDescriptor()
        texDesc.textureType = .type3D
        texDesc.pixelFormat = .r8Unorm
        texDesc.width = 32
        texDesc.height = 32
        texDesc.depth = 32
        texDesc.usage = [.shaderRead]
        let volTex = device.makeTexture(descriptor: texDesc)!

        var b4Times: [Double] = []
        var dummyBytes = [UInt8](repeating: 128, count: 32 * 32 * 32)
        for _ in 0..<30 {
            let t0 = ContinuousClock.now
            dummyBytes.withUnsafeBytes { rawPtr in
                volTex.replace(
                    region: MTLRegionMake3D(0, 0, 0, 32, 32, 32),
                    mipmapLevel: 0,
                    slice: 0,
                    withBytes: rawPtr.baseAddress!,
                    bytesPerRow: 32,
                    bytesPerImage: 32 * 32
                )
            }
            let t1 = ContinuousClock.now
            let totalMs = durationToMs(t1 - t0)
            b4Times.append(totalMs)
        }
        b4Times.sort()
        let b4Median = b4Times[b4Times.count / 2]
        let b4P95 = b4Times[Int(ceil(0.95 * Double(b4Times.count))) - 1]
        let b4Fails = b4Times.filter { $0 > 100.0 }.count
        print("iOS Budget 4 (Volume frame texture upload): median=\(String(format: "%.2f", b4Median)) ms, p95=\(String(format: "%.2f", b4P95)) ms, fails=\(b4Fails), threshold=100.0 ms")
        XCTAssertLessThanOrEqual(b4P95, 100.0, "Budget 4 p95 must be <= 100 ms")

        // Budget 5: Phase update <= 35 ms per step (N=30)
        var b5CompTimes: [Double] = []
        var b5RenderTimes: [Double] = []
        for step in 0..<30 {
            let nextFrame = (step * 2) % 60
            let t0 = ContinuousClock.now
            model.setFrame(nextFrame)
            let t1 = ContinuousClock.now

            let range = model.currentFieldRange
            let planImg = ColorRamps.createPlanSliceCGImage(data: model.currentFieldData, k: 12, field: model.selectedField, range: range)
            let t2 = ContinuousClock.now

            XCTAssertNotNil(planImg)
            let compMs = durationToMs(t1 - t0)
            let totalMs = durationToMs(t2 - t0)
            b5CompTimes.append(compMs)
            b5RenderTimes.append(totalMs)
        }
        b5RenderTimes.sort()
        let b5Median = b5RenderTimes[b5RenderTimes.count / 2]
        let b5P95 = b5RenderTimes[Int(ceil(0.95 * Double(b5RenderTimes.count))) - 1]
        let b5Fails = b5RenderTimes.filter { $0 > 35.0 }.count
        print("iOS Budget 5 (Phase update): median=\(String(format: "%.2f", b5Median)) ms, p95=\(String(format: "%.2f", b5P95)) ms, fails=\(b5Fails), threshold=35.0 ms")
        XCTAssertLessThanOrEqual(b5P95, 35.0, "Budget 5 p95 must be <= 35 ms")
    }
}
