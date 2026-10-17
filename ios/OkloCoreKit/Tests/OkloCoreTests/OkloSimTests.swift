import Foundation
import Testing
import OkloCore

// Swift -> C ABI -> Rust, checked against the revision 0 MRC files.
// Fixture dir: $OKLO_FIXTURE_DIR, default the Antigravity scratch copy under $HOME.

private let fixtureDir = ProcessInfo.processInfo.environment["OKLO_FIXTURE_DIR"]
    ?? NSHomeDirectory() + "/.gemini/antigravity/scratch/oklo_gui_scratch"

/// The configuration export_oklo_mrc.py used for the files on disk (b = 0.17).
private func pythonExportConfig() -> OkloSim.Config {
    .init(nx: 32, ny: 32, nz: 32, phase: 0.15, nullScenario: false,
          pulseBoundary: 0.17, cyclePeriodSeconds: 10800)
}

private func mrcData(_ name: String) throws -> [Float] {
    let bytes = try Data(contentsOf: URL(fileURLWithPath: "\(fixtureDir)/oklo_\(name).mrc"))
    try #require(bytes.count == 1024 + 4 * 32768)
    return bytes.dropFirst(1024).withUnsafeBytes { raw in
        (0..<32768).map { Float(bitPattern: UInt32(littleEndian: raw.loadUnaligned(fromByteOffset: $0 * 4, as: UInt32.self))) }
    }
}

@Test func abiVersionIsOne() {
    #expect(OkloSim.abiVersion == 1)
}

@Test func rejectsInvalidPulseBoundary() {
    var config = pythonExportConfig()
    config.pulseBoundary = 0
    #expect(OkloSim(config) == nil)
}

@Test(arguments: [
    (OkloSim.Field.temperature, "temperature"),
    (.hydration, "hydration"),
    (.h2Concentration, "h2_concentration"),
    (.u235Ratio, "u235_ratio"),
    (.polymerYield, "polymer_yield"),
    (.compartments, "compartments"),
])
func fieldMatchesPythonExportBitForBit(field: OkloSim.Field, name: String) throws {
    let sim = try #require(OkloSim(pythonExportConfig()))
    let ours = sim.field(field)
    let reference = try mrcData(name)
    #expect(ours.map(\.bitPattern) == reference.map(\.bitPattern))
}

@Test func zoneCountsMatchRecordedSnapshot() throws {
    let sim = try #require(OkloSim(pythonExportConfig()))
    var counts = [Int](repeating: 0, count: 5)
    for zone in sim.field(.compartments) {
        counts[Int(zone)] += 1
    }
    #expect(counts == [7560, 3356, 20656, 0, 1196])
}
