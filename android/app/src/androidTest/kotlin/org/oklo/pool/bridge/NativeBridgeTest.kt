package org.oklo.pool.bridge

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class NativeBridgeTest {
    @Test fun abiVersionAndSixFieldsLoad() {
        assertEquals(1, OkloCoreNative.abiVersion())
        OkloSim.create(0.15, 0.17, false).use { sim ->
            for (field in 0..5) {
                val values = FloatArray(OkloSim.VOXELS)
                assertEquals(OkloSim.VOXELS, sim.copyField(field, values))
                assertEquals(OkloSim.VOXELS, values.size)
            }
        }
    }

    @Test fun invalidConfigIsRejected() {
        assertThrows(IllegalArgumentException::class.java) { OkloSim.create(Double.NaN, 0.17, false) }
        assertEquals(0L, OkloCoreNative.create(0, 32, 32, 0, 0.15, 0.17, 10_800.0))
        assertEquals(0L, OkloCoreNative.create(32, 32, 32, 2, 0.15, 0.17, 10_800.0))
    }

    @Test fun steppingWrapsAndRejectsNegativeTime() {
        OkloSim.create(0.15, 0.17, false).use { sim ->
            assertEquals(0.15, sim.step(10_800.0), 1e-12)
            assertThrows(IllegalStateException::class.java) { sim.step(-1.0) }
        }
    }

    @Test fun shortDestinationIsRejected() {
        OkloSim.create(0.15, 0.17, false).use { sim ->
            assertThrows(IllegalStateException::class.java) { sim.copyField(0, FloatArray(16)) }
        }
    }

    @Test fun closeIsIdempotentAtWrapperBoundary() {
        val sim = OkloSim.create(0.15, 0.17, false)
        sim.close()
        sim.close()
    }
}
