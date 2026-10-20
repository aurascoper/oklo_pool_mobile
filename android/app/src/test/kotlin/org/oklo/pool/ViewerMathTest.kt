package org.oklo.pool

import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Assert.assertTrue
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

    @Test fun phaseScrubberSelectsSixtyFramesAndNeverReachesOne() {
        assertEquals(58, ViewerMath.SLIDER_STEPS)
        assertEquals(59f / 60f, ViewerMath.SLIDER_VALUE_RANGE.endInclusive, 1e-7f)

        val phases = ViewerMath.selectablePhases()
        assertEquals(60, phases.size)
        for (n in 0..59) {
            val expected = n / 60.0
            assertEquals(expected, phases[n], 1e-12)
            val tickFloat = ViewerMath.SLIDER_VALUE_RANGE.start +
                (ViewerMath.SLIDER_VALUE_RANGE.endInclusive - ViewerMath.SLIDER_VALUE_RANGE.start) * (n / 59f)
            assertEquals(expected, ViewerMath.snapSliderPhase(tickFloat), 1e-12)
            assertEquals(n + 1, ViewerMath.frameNumber(expected))
        }

        assertEquals("Frame 1 / 60", ViewerMath.frameLabel(0.0))
        assertEquals("Frame 10 / 60", ViewerMath.frameLabel(0.15))
        assertEquals("Frame 60 / 60", ViewerMath.frameLabel(59.0 / 60.0))

        val model = ViewerModel()
        model.seek(1.0)
        assertEquals(59.0 / 60.0, model.phase, 1e-12)
        assertTrue(model.phase < 1.0)
        assertEquals("Frame 60 / 60", ViewerMath.frameLabel(model.phase))
    }
}
