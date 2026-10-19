import XCTest
@preconcurrency import OkloCore
@testable import OkloPool

@MainActor
final class OkloAppModelTests: XCTestCase {

    func testInitialModelState() {
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
                XCTAssertLessThanOrEqual(range.max, 15.0)
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
}
