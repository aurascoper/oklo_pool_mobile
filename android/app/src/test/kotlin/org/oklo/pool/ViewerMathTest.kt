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
}
