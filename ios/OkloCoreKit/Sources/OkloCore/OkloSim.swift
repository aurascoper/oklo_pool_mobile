import OkloCoreFFI

/// Swift owner of one oklo_core fixture. Not thread-safe: use it from one
/// thread or one actor. Fields are revision 0 prescribed analytic values,
/// not a physical simulation (docs/PROVENANCE.md).
public final class OkloSim {
    public enum Field: UInt32, CaseIterable {
        case temperature = 0
        case hydration
        case h2Concentration
        case u235Ratio
        case polymerYield
        case compartments
    }

    public struct Config {
        public var nx: UInt32
        public var ny: UInt32
        public var nz: UInt32
        public var phase: Double
        public var nullScenario: Bool
        /// Required. Revision 0 has 0.1667 (GUI) and 0.17 (MRC exporter);
        /// choosing one is a revision 1 decision (finding R0-1).
        public var pulseBoundary: Double
        public var cyclePeriodSeconds: Double

        public init(nx: UInt32, ny: UInt32, nz: UInt32, phase: Double, nullScenario: Bool,
                    pulseBoundary: Double, cyclePeriodSeconds: Double) {
            self.nx = nx
            self.ny = ny
            self.nz = nz
            self.phase = phase
            self.nullScenario = nullScenario
            self.pulseBoundary = pulseBoundary
            self.cyclePeriodSeconds = cyclePeriodSeconds
        }
    }

    public static var abiVersion: UInt32 { oklo_core_abi_version() }

    private let handle: OpaquePointer

    /// nil if the core rejects the configuration.
    public init?(_ config: Config) {
        var c = OkloSimConfig(
            nx: config.nx, ny: config.ny, nz: config.nz,
            null_scenario: config.nullScenario ? 1 : 0,
            phase: config.phase,
            pulse_boundary: config.pulseBoundary,
            cycle_period_s: config.cyclePeriodSeconds)
        guard let h = oklo_sim_create(&c) else { return nil }
        handle = h
    }

    deinit {
        oklo_sim_destroy(handle)
    }

    public var phase: Double { oklo_sim_phase(handle) }

    /// Advance by `seconds` along the cycle, wrapping at its end. Returns false,
    /// and changes nothing, for a negative or non-finite step.
    @discardableResult
    public func step(seconds: Double) -> Bool {
        oklo_sim_step(handle, seconds) == OKLO_OK
    }

    /// A copy of one field, X fastest: index = k*nx*ny + j*nx + i.
    public func field(_ field: Field) -> [Float] {
        var len = 0
        guard let p = oklo_sim_field(handle, field.rawValue, &len) else { return [] }
        return Array(UnsafeBufferPointer(start: p, count: len))
    }
}
