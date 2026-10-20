package org.oklo.pool.bridge

internal object OkloCoreNative {
    init { System.loadLibrary("oklo_core_jni") }

    external fun abiVersion(): Int
    external fun create(
        nx: Int, ny: Int, nz: Int, nullScenario: Int,
        phase: Double, pulseBoundary: Double, cyclePeriodS: Double
    ): Long
    external fun step(handle: Long, dtS: Double): Int
    external fun copyField(handle: Long, fieldId: Int, out: FloatArray): Int
    external fun phase(handle: Long): Double
    external fun destroy(handle: Long)
}

class OkloSim private constructor(private var handle: Long) : AutoCloseable {
    private val lock = Any()

    fun step(dtSeconds: Double): Double = synchronized(lock) {
        check(handle != 0L) { "Simulation is closed" }
        val status = OkloCoreNative.step(handle, dtSeconds)
        check(status == 0) { "Core rejected the phase step ($status)" }
        OkloCoreNative.phase(handle)
    }

    fun phase(): Double = synchronized(lock) {
        check(handle != 0L) { "Simulation is closed" }
        OkloCoreNative.phase(handle)
    }

    fun copyField(fieldId: Int, out: FloatArray): Int = synchronized(lock) {
        check(handle != 0L) { "Simulation is closed" }
        val count = OkloCoreNative.copyField(handle, fieldId, out)
        check(count >= 0) { "Core field copy failed ($count)" }
        count
    }

    override fun close() = synchronized(lock) {
        if (handle != 0L) {
            OkloCoreNative.destroy(handle)
            handle = 0L
        }
    }

    companion object {
        const val VOXELS = 32 * 32 * 32

        init {
            check(OkloCoreNative.abiVersion() == 1) {
                "Incompatible oklo_core ABI: expected 1, received ${OkloCoreNative.abiVersion()}"
            }
        }

        fun create(phase: Double, pulseBoundary: Double, nullScenario: Boolean): OkloSim {
            require(phase in 0.0..1.0)
            require(pulseBoundary > 0.0 && pulseBoundary < 1.0)
            val handle = OkloCoreNative.create(
                32, 32, 32, if (nullScenario) 1 else 0,
                phase, pulseBoundary, 10_800.0
            )
            check(handle != 0L) { "oklo_core rejected the simulation configuration" }
            return OkloSim(handle)
        }
    }
}
