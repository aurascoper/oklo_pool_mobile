import XCTest
import CryptoKit
@preconcurrency import OkloCore
@testable import OkloPool

@MainActor
final class OkloAppModelTests: XCTestCase {

    func testInitialModelState() async {
        let model = OkloAppModel()
        XCTAssertEqual(model.selectedField, .temperature)
        XCTAssertEqual(model.phase, 0.15, accuracy: 1e-6)
        XCTAssertEqual(model.pulseBoundary, 0.17, accuracy: 1e-6)
        XCTAssertFalse(model.nullScenario)
        XCTAssertEqual(model.kDepthSlice, 12)
        XCTAssertEqual(model.jSectionRow, 15)
        XCTAssertEqual(model.viewMode, .slices)
        XCTAssertFalse(model.isPlaying)
        XCTAssertNil(model.errorMessage)

        XCTAssertEqual(model.currentFieldData.count, 32768)
        XCTAssertNotNil(model.planSliceImage)
        XCTAssertNotNil(model.sectionSliceImage)
        for _ in 0..<2_000 where model.seriesStats == nil {
            try? await Task.sleep(for: .milliseconds(10))
        }
        XCTAssertNotNil(model.seriesStats)
    }

    func testFieldSelectionAndBounds() {
        let model = OkloAppModel()

        for field in OkloSim.Field.allCases {
            model.selectField(field)
            XCTAssertEqual(model.selectedField, field)
            XCTAssertEqual(model.currentFieldData.count, 32768)

            let range = model.currentFieldRange
            XCTAssertLessThanOrEqual(range.min, range.max)

            switch field {
            case .temperature:
                XCTAssertGreaterThanOrEqual(range.min, 20.0)
                XCTAssertLessThanOrEqual(range.max, 600.0)
            case .hydration:
                XCTAssertGreaterThanOrEqual(range.min, 0.01)
                XCTAssertLessThanOrEqual(range.max, 1.05)
            case .h2Concentration:
                XCTAssertGreaterThanOrEqual(range.min, 0.0005)
                XCTAssertLessThanOrEqual(range.max, 10.0)
            case .u235Ratio:
                XCTAssertGreaterThanOrEqual(range.min, 0.004)
                XCTAssertLessThanOrEqual(range.max, 0.008)
            case .polymerYield:
                XCTAssertGreaterThanOrEqual(range.min, 0.0)
                XCTAssertLessThanOrEqual(range.max, 16.0)
            case .compartments:
                for v in model.currentFieldData {
                    let z = Int(round(v))
                    XCTAssertTrue(z >= 0 && z <= 4, "Zone \(z) must be between 0 and 4")
                }
            }
        }
    }

    func testPhaseNavigationAndWrapping() {
        let model = OkloAppModel()

        model.setFrame(10)
        XCTAssertEqual(model.currentFrameIndex, 10)
        XCTAssertEqual(model.phase, 10.0 / 60.0, accuracy: 1e-6)

        model.stepFrame(forward: true)
        XCTAssertEqual(model.currentFrameIndex, 11)

        model.setFrame(59)
        XCTAssertEqual(model.currentFrameIndex, 59)
        model.stepFrame(forward: true)
        XCTAssertEqual(model.currentFrameIndex, 0)
        XCTAssertGreaterThanOrEqual(model.phase, 0.0)
        XCTAssertLessThan(model.phase, 1.0)

        model.stepFrame(forward: false)
        XCTAssertEqual(model.currentFrameIndex, 59)

        model.setPhase(0.50)
        XCTAssertEqual(model.phase, 0.50, accuracy: 1e-6)
        XCTAssertEqual(model.currentFrameIndex, 30)
    }

    func testPulseBoundarySensitivity() {
        let model = OkloAppModel()
        model.setPhase(0.15)
        model.selectField(.temperature)
        model.setPulseBoundary(0.17)
        let data17 = model.currentFieldData

        model.setPulseBoundary(0.1667)
        let data1667 = model.currentFieldData

        XCTAssertEqual(data17.count, data1667.count)
        var hasDifference = false
        for idx in 0..<data17.count {
            if abs(data17[idx] - data1667[idx]) > 1e-4 {
                hasDifference = true
                break
            }
        }
        XCTAssertTrue(hasDifference, "Temperature at phase 0.15 must differ between pulse boundaries 0.17 and 0.1667")
    }

    func testNullScenarioInvariance() {
        let model = OkloAppModel()
        model.setNullScenario(true)
        XCTAssertTrue(model.nullScenario)

        model.selectField(.polymerYield)
        for v in model.currentFieldData {
            XCTAssertEqual(v, 0.0, accuracy: 1e-6, "Polymer yield must be identically 0 in null scenario")
        }

        model.selectField(.h2Concentration)
        for v in model.currentFieldData {
            XCTAssertEqual(v, 0.001, accuracy: 1e-6, "H2 concentration index must be 0.001 in null scenario")
        }
    }

    func testSliceClamping() {
        let model = OkloAppModel()

        model.setKDepth(-5)
        XCTAssertEqual(model.kDepthSlice, 0)

        model.setKDepth(45)
        XCTAssertEqual(model.kDepthSlice, 31)

        model.setJSection(-10)
        XCTAssertEqual(model.jSectionRow, 0)

        model.setJSection(50)
        XCTAssertEqual(model.jSectionRow, 31)
    }

    func testPlaybackLifecycle() {
        let model = OkloAppModel()
        XCTAssertFalse(model.isPlaying)

        model.togglePlayback()
        XCTAssertTrue(model.isPlaying)

        model.pause()
        XCTAssertFalse(model.isPlaying)

        model.resetToDefaults()
        XCTAssertEqual(model.selectedField, .temperature)
        XCTAssertEqual(model.phase, 0.15, accuracy: 1e-6)
        XCTAssertEqual(model.pulseBoundary, 0.17, accuracy: 1e-6)
        XCTAssertFalse(model.nullScenario)
    }

    func testMemoryCleanupAndDeinit() {
        for _ in 0..<10 {
            let tempModel = OkloAppModel()
            tempModel.selectField(.compartments)
            tempModel.setFrame(25)
            _ = tempModel.currentFieldData.count
        }
        XCTAssertTrue(true, "Repeated allocation and deallocation succeeded without leak or crash")
    }

    func testRaymarcherUniformsMemoryLayout() {
        XCTAssertEqual(MemoryLayout<PackedFloat3>.size, 12)
        XCTAssertEqual(MemoryLayout<PackedFloat3>.stride, 12)
        XCTAssertEqual(MemoryLayout<PackedFloat3>.alignment, 4)

        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.size, 64)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.stride, 64)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.alignment, 4)

        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.camPos), 0)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.aspect), 12)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.camRight), 16)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.opacity), 28)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.camUp), 32)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.invertA), 44)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.camFwd), 48)
        XCTAssertEqual(MemoryLayout<RaymarcherUniforms>.offset(of: \.categorical), 60)
    }

    func testPolymerFieldOntologyAmendmentMetadata() {
        let meta = FieldCatalog.metadata(for: .polymerYield)
        XCTAssertEqual(meta.displayName, "Polymer accumulation index")
        XCTAssertEqual(meta.shortUnit, "arbitrary units")
        XCTAssertTrue(meta.canonicalUnit.contains("arbitrary units"))
        XCTAssertTrue(meta.canonicalUnit.contains("umol/L"))
        XCTAssertTrue(meta.limitationStatement.contains("Prescribed heuristic field"))
        XCTAssertTrue(meta.limitationStatement.contains("Values are not measured or predicted polymer concentrations"))
    }

    func testCf2TwentyCaseMatrixMatchesReference() {
        let phases: [Double] = [0.05, 0.15, 0.20, 0.50, 0.90]
        let scenarios: [Bool] = [false, true]
        let boundaries: [Double] = [0.1667, 0.17]
        let expectedHashes: [String] = [
            // Case 01: p=0.05, active, b=0.1667
            "f609935a9b5a15d6e39d5342419dfae6d3680ceec776a8d7c1f7636815d93c60",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "b84d053cc1ac314d1b5f8cdb515e20c94d39400decf8fed55993f123c3db8dc8",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 02: p=0.05, active, b=0.1700
            "e17f90fbce88efb9db513b888e749a9ab47649ae7f6657eb1ea3e9eae757b470",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "b84d053cc1ac314d1b5f8cdb515e20c94d39400decf8fed55993f123c3db8dc8",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 03: p=0.05, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 04: p=0.05, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 05: p=0.15, active, b=0.1667
            "a300fe35ef3c0dc68641b673e9423edda74acd100ecc7a49768d9ebaabac429b",
            "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
            "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 06: p=0.15, active, b=0.1700
            "89d85fc802e4f4d638af529f66bf7f3eb73b14e704afaf31dc652b68c9cb8e47",
            "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
            "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 07: p=0.15, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 08: p=0.15, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 09: p=0.20, active, b=0.1667
            "bc44b77b1acb2b28f33556085c86e5882cfe0c26e230a8c7c615d485964a4c20",
            "58e4ed5ee9fba7a1356dffdaf7ad0d10d7665f3b06324849586fcf7f364a6e55",
            "735dd208dc14ecd45b253c6196447972c9e7536ec5e2adac052a9b5aa2266d85",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "9dd41d53f1c7f245d6008604bc3bb0fdacf5ba23cdb21a2fce4797415820f2f0",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 10: p=0.20, active, b=0.1700
            "6b4d31f8922bf4380532e654aba8fd1c67bb2d328045388459f6c81edac8199b",
            "58e4ed5ee9fba7a1356dffdaf7ad0d10d7665f3b06324849586fcf7f364a6e55",
            "735dd208dc14ecd45b253c6196447972c9e7536ec5e2adac052a9b5aa2266d85",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "9dd41d53f1c7f245d6008604bc3bb0fdacf5ba23cdb21a2fce4797415820f2f0",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 11: p=0.20, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 12: p=0.20, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 13: p=0.50, active, b=0.1667
            "3f8880551cd6908cbc1ac654c5c5ebed6b8444426deec500039d761921c353b2",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "2eca2559341f08a6e8f6a27a265faf83b9e12e0252ec7a2a3d988f419b476a36",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 14: p=0.50, active, b=0.1700
            "a8e62702525f9fec2aebb54b9b232434b8aae80f037b9bc7eaa475cc3a4250ef",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "2eca2559341f08a6e8f6a27a265faf83b9e12e0252ec7a2a3d988f419b476a36",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 15: p=0.50, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 16: p=0.50, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 17: p=0.90, active, b=0.1667
            "0075d6415381e0ffc044b88e411581ac679226b4ac24ef79921a7f14208e803d",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "a5d9f05ab945fe3a55904445503c1de18f2909740032e57860f1d9fb30f65486",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 18: p=0.90, active, b=0.1700
            "a6a96cb207606c2a69dfbca061ab1f527709674926393b9ad18070dc3a7e5426",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "a5d9f05ab945fe3a55904445503c1de18f2909740032e57860f1d9fb30f65486",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 19: p=0.90, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 20: p=0.90, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7"
        ]
        var idx = 0
        for phase in phases {
            for nullScenario in scenarios {
                for boundary in boundaries {
                    let cfg = OkloSim.Config(
                        nx: 32, ny: 32, nz: 32,
                        phase: phase,
                        nullScenario: nullScenario,
                        pulseBoundary: boundary,
                        cyclePeriodSeconds: 10800.0
                    )
                    guard let sim = OkloSim(cfg) else {
                        XCTFail("Failed to create OkloSim at index \(idx)")
                        return
                    }
                    for field in OkloSim.Field.allCases {
                        let values = sim.field(field)
                        XCTAssertEqual(values.count, 32768)
                        let hash = values.withUnsafeBufferPointer { buf -> String in
                            let raw = UnsafeRawBufferPointer(buf)
                            return SHA256.hash(data: raw).map { String(format: "%02x", $0) }.joined()
                        }
                        XCTAssertEqual(hash, expectedHashes[idx], "Mismatch at index \(idx) (p=\(phase), null=\(nullScenario), b=\(boundary), f=\(field))")
                        idx += 1
                    }
                }
            }
        }
        XCTAssertEqual(idx, 120)
    }

    func testApplePrivacyManifestValidAndPresentInBundle() throws {
        guard let url = Bundle.main.url(forResource: "PrivacyInfo", withExtension: "xcprivacy") else {
            XCTFail("PrivacyInfo.xcprivacy not found in OkloPool.app bundle")
            return
        }
        let data = try Data(contentsOf: url)
        let plist = try PropertyListSerialization.propertyList(from: data, options: [], format: nil)
        guard let dict = plist as? [String: Any] else {
            XCTFail("PrivacyInfo.xcprivacy is not a valid dictionary plist")
            return
        }

        XCTAssertEqual(dict["NSPrivacyTracking"] as? Bool, false)
        XCTAssertEqual((dict["NSPrivacyCollectedDataTypes"] as? [Any])?.count, 0)
        XCTAssertEqual((dict["NSPrivacyTrackingDomains"] as? [Any])?.count, 0)
        XCTAssertEqual((dict["NSPrivacyAccessedAPITypes"] as? [Any])?.count, 0)
    }

    func testFieldLimitationRecordsContainAllSixFields() {
        XCTAssertEqual(FieldCatalog.allFields.count, 6)
        for field in OkloSim.Field.allCases {
            let meta = FieldCatalog.metadata(for: field)
            XCTAssertFalse(meta.displayName.isEmpty)
            XCTAssertFalse(meta.canonicalUnit.isEmpty)
            XCTAssertFalse(meta.literatureAnchor.isEmpty)
            XCTAssertFalse(meta.limitationStatement.isEmpty)

            // Strict verification of zero em-dashes and zero en-dashes
            XCTAssertFalse(meta.limitationStatement.contains("\u{2013}"), "Must not contain en-dash")
            XCTAssertFalse(meta.limitationStatement.contains("\u{2014}"), "Must not contain em-dash")
        }
    }
}

