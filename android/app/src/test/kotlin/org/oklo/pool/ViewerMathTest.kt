package org.oklo.pool

import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test

class ViewerMathTest {
    @Test fun phaseAdvancesOneFrame() {
        assertEquals(0.15 + 1.0 / 60.0, ViewerMath.nextPhase(0.15, 180.0), 1e-12)
    }

    @Test fun phaseWrapsAtCycleEnd() {
        assertEquals(0.0, ViewerMath.nextPhase(0.99, 10_800.0 * 0.01), 1e-12)
    }

    @Test fun negativeStepIsRejected() {
        assertThrows(IllegalArgumentException::class.java) { ViewerMath.nextPhase(0.3, -1.0) }
    }

    @Test fun pulseChangesAtSelectedBoundary() {
        assertEquals(1.0, ViewerMath.pulse(0.17, 0.17), 1e-12)
        assertEquals(1.0, ViewerMath.pulse(0.1667, 0.1667), 1e-12)
    }

    @Test fun nullScenarioDoesNotChangePhaseMath() {
        assertEquals(0.25, ViewerMath.nextPhase(0.2, 540.0), 1e-12)
    }

    @Test fun compartmentStatsCountZoneThreeAndFourShares() {
        val stats = FieldStats.summarize(0.5, 5, floatArrayOf(0f, 1f, 2f, 3f, 4f, 3f, 4f, 4f, 1f, 1f))

        assertEquals(0.2f, stats.zone3Share, 1e-6f)
        assertEquals(0.3f, stats.zone4Share, 1e-6f)
    }

    @Test fun continuousFieldStatsDoNotReportZoneShares() {
        val stats = FieldStats.summarize(0.5, 0, floatArrayOf(2f, 4f, 6f))

        assertEquals(0f, stats.zone3Share, 0f)
        assertEquals(0f, stats.zone4Share, 0f)
        assertEquals(4f, stats.mean, 0f)
    }
}
