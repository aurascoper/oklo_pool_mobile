package org.oklo.pool

import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.runBlocking
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import kotlin.math.ceil

@RunWith(AndroidJUnit4::class)
class OnRenderGateInstrumentedTest {

    @Test
    fun onRenderGateSixItemsAndTimingBudgetsOnEmulator() = runBlocking {
        val model = ViewerModel()

        // Item 1: Cold launch initial state & first useful frame
        assertEquals(0.15, model.phase, 1e-6)
        assertEquals(0.17, model.pulseBoundary, 1e-6)
        assertFalse(model.nullScenario)
        assertEquals(0, model.fieldId)

        // Item 2: Initial-field rendering correctness & pixel diversity
        val initialValues = model.fieldValues()
        assertEquals(32768, initialValues.size)
        val minTemp = initialValues.minOrNull() ?: Float.NaN
        val maxTemp = initialValues.maxOrNull() ?: Float.NaN
        assertTrue(minTemp >= 20.0f)
        assertTrue(maxTemp <= 600.0f)
        assertTrue(maxTemp > minTemp)
        val distinctCount = initialValues.take(4096).toSet().size
        assertTrue(distinctCount >= 50)

        // Item 3: All six field transitions and limitation records
        for (field in FixtureField.all) {
            model.selectField(field.id)
            assertEquals(field.id, model.fieldId)
            val vals = model.fieldValues()
            assertEquals(32768, vals.size)
            val rec = fieldLimitationRecords[field.id]
            assertEquals(field.id, rec.id)
            assertTrue(rec.limitation.isNotEmpty())
            assertFalse(rec.limitation.contains("\u2013"))
            assertFalse(rec.limitation.contains("\u2014"))
        }
        model.selectField(0)

        // Item 4: First chart entry and 60-frame series
        val series = model.fieldStatsSeries(0, 0.17, false)
        assertEquals(60, series.size)
        assertEquals(0.0, series[0].phase, 1e-6)
        assertTrue(series[0].max >= series[0].min)

        // Item 5: 3D volume readiness (16 isometric depth layers)
        var volumeSamples = 0
        for (layer in 0 until 16) {
            val z = layer * 2
            for (y in 0 until 32) {
                for (x in 0 until 32) {
                    assertTrue(initialValues[z * 1024 + y * 32 + x].isFinite())
                    volumeSamples++
                }
            }
        }
        assertEquals(16384, volumeSamples)

        // Item 6: Phase checkpoints 0/60, 9/60, 15/60, 30/60, 59/60
        val checkpoints = listOf(0, 9, 15, 30, 59)
        for (frameIdx in checkpoints) {
            val expectedPhase = frameIdx / 60.0
            model.seek(expectedPhase)
            assertEquals(expectedPhase, model.phase, 1e-6)
            assertEquals("Frame ${frameIdx + 1} / 60", ViewerMath.frameLabel(model.phase))
            val v = model.fieldValues()
            assertEquals(32768, v.size)
        }
        model.seek(0.15)

        // Warm-up: 3 unmeasured runs
        repeat(3) {
            model.fieldValues()
            model.fieldStatsSeries(0, 0.17, false)
        }

        // Budget 1: Initial model + slice generation <= 100 ms (N=30)
        val b1Times = DoubleArray(30)
        for (i in 0 until 30) {
            val t0 = System.nanoTime()
            val m = ViewerModel()
            val vals = m.fieldValues()
            val plan = FloatArray(1024) { idx -> vals[12 * 1024 + idx] }
            val sec = FloatArray(1024) { idx -> vals[(idx / 32) * 1024 + 16 * 32 + (idx % 32)] }
            val t1 = System.nanoTime()
            b1Times[i] = (t1 - t0) / 1_000_000.0
            assertTrue(plan.isNotEmpty() && sec.isNotEmpty())
        }
        b1Times.sort()
        val b1P95 = b1Times[ceil(0.95 * b1Times.size).toInt() - 1]
        assertTrue("Emulator Budget 1 p95 ($b1P95 ms) <= 100 ms", b1P95 <= 100.0)

        // Budget 2: Field switching <= 50 ms (N=30)
        val b2Times = DoubleArray(30)
        for (i in 0 until 30) {
            val nextField = (i + 1) % 6
            val t0 = System.nanoTime()
            model.selectField(nextField)
            val vals = model.fieldValues()
            val plan = FloatArray(1024) { idx -> vals[12 * 1024 + idx] }
            val t1 = System.nanoTime()
            b2Times[i] = (t1 - t0) / 1_000_000.0
            assertTrue(plan.isNotEmpty())
        }
        b2Times.sort()
        val b2P95 = b2Times[ceil(0.95 * b2Times.size).toInt() - 1]
        assertTrue("Emulator Budget 2 p95 ($b2P95 ms) <= 50 ms", b2P95 <= 50.0)

        // Budget 3: Uncached 60-frame chart calculation <= 400 ms (N=20)
        val b3Times = DoubleArray(20)
        for (i in 0 until 20) {
            val t0 = System.nanoTime()
            val s = model.fieldStatsSeries(0, 0.17, false)
            val t1 = System.nanoTime()
            b3Times[i] = (t1 - t0) / 1_000_000.0
            assertEquals(60, s.size)
        }
        b3Times.sort()
        val b3P95 = b3Times[ceil(0.95 * b3Times.size).toInt() - 1]
        assertTrue("Emulator Budget 3 p95 ($b3P95 ms) <= 400 ms", b3P95 <= 400.0)

        // Budget 4: Volume frame <= 100 ms (N=30)
        val b4Times = DoubleArray(30)
        val volData = model.fieldValues()
        for (i in 0 until 30) {
            val t0 = System.nanoTime()
            var acc = 0f
            for (layer in 0 until 16) {
                val z = layer * 2
                for (y in 0 until 32) {
                    for (x in 0 until 32) {
                        acc += volData[z * 1024 + y * 32 + x]
                    }
                }
            }
            val t1 = System.nanoTime()
            b4Times[i] = (t1 - t0) / 1_000_000.0
            assertTrue(acc.isFinite())
        }
        b4Times.sort()
        val b4P95 = b4Times[ceil(0.95 * b4Times.size).toInt() - 1]
        assertTrue("Emulator Budget 4 p95 ($b4P95 ms) <= 100 ms", b4P95 <= 100.0)

        // Budget 5: Phase update <= 35 ms (N=30)
        val b5Times = DoubleArray(30)
        for (i in 0 until 30) {
            val p = (i % 60) / 60.0
            val t0 = System.nanoTime()
            model.seek(p)
            val vals = model.fieldValues()
            val plan = FloatArray(1024) { idx -> vals[12 * 1024 + idx] }
            val t1 = System.nanoTime()
            b5Times[i] = (t1 - t0) / 1_000_000.0
            assertTrue(plan.isNotEmpty())
        }
        b5Times.sort()
        val b5P95 = b5Times[ceil(0.95 * b5Times.size).toInt() - 1]
        assertTrue("Emulator Budget 5 p95 ($b5P95 ms) <= 35 ms", b5P95 <= 35.0)
    }
}
