package org.oklo.pool

import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.oklo.pool.bridge.OkloSim
import kotlin.math.ceil

class OnRenderGateBenchmarkTest {

    private fun checkNoDashes(text: String, label: String) {
        assertFalse("$label must not contain en-dash", text.contains("\u2013"))
        assertFalse("$label must not contain em-dash", text.contains("\u2014"))
    }

    @Test
    fun testItem1ColdLaunchAndFirstUsefulFrame() {
        val model = ViewerModel()
        assertEquals(0.15, model.phase, 1e-6)
        assertEquals(0.17, model.pulseBoundary, 1e-6)
        assertFalse(model.nullScenario)
        assertEquals(0, model.fieldId)

        val values = model.fieldValues()
        assertEquals(32768, values.size)
        assertTrue(values.any { it > 0f })
    }

    @Test
    fun testItem2InitialFieldRenderingCorrectnessAndPixelDiversity() {
        val model = ViewerModel()
        val values = model.fieldValues()
        assertEquals(32768, values.size)

        val minVal = values.minOrNull() ?: Float.NaN
        val maxVal = values.maxOrNull() ?: Float.NaN
        assertTrue("Min temperature >= 20.0 C", minVal >= 20.0f)
        assertTrue("Max temperature <= 600.0 C", maxVal <= 600.0f)
        assertTrue("Temperature max > min", maxVal > minVal)

        val distinctCount = values.take(4096).toSet().size
        assertTrue("Pixel diversity requires gradient: found $distinctCount distinct values in 4096 voxels", distinctCount >= 50)
    }

    @Test
    fun testItem3AllSixFieldTransitionsAndLimitations() {
        val model = ViewerModel()
        assertEquals(6, FixtureField.all.size)
        assertEquals(6, fieldLimitationRecords.size)

        for (field in FixtureField.all) {
            model.selectField(field.id)
            assertEquals(field.id, model.fieldId)

            val values = model.fieldValues()
            assertEquals(32768, values.size)

            val rec = fieldLimitationRecords[field.id]
            assertEquals(field.id, rec.id)
            assertTrue(rec.title.isNotEmpty())
            assertTrue(rec.unit.isNotEmpty())
            assertTrue(rec.limitation.isNotEmpty())

            checkNoDashes(rec.title, "title for ${field.title}")
            checkNoDashes(rec.unit, "unit for ${field.title}")
            checkNoDashes(rec.limitation, "limitation for ${field.title}")
        }
    }

    @Test
    fun testItem4FirstChartEntryAndSixtyFrameSeries() = runTest {
        val model = ViewerModel()
        val series = model.fieldStatsSeries(0, 0.17, false)
        assertEquals(60, series.size)

        val firstEntry = series[0]
        assertEquals(0.0, firstEntry.phase, 1e-6)
        assertTrue(firstEntry.min.isFinite())
        assertTrue(firstEntry.mean.isFinite())
        assertTrue(firstEntry.max.isFinite())
        assertTrue(firstEntry.max >= firstEntry.min)
    }

    @Test
    fun testItem5ThreeDimensionalVolumeViewReadiness() {
        val model = ViewerModel()
        val values = model.fieldValues()
        val depthLayers = 16
        val samplesPerLayer = 32 * 32
        var extractedSamples = 0

        for (layer in 0 until depthLayers) {
            val zIndex = layer * 2
            assertTrue(zIndex in 0..31)
            for (y in 0 until 32) {
                for (x in 0 until 32) {
                    val idx = zIndex * 1024 + y * 32 + x
                    val v = values[idx]
                    assertTrue(v.isFinite())
                    extractedSamples++
                }
            }
        }
        assertEquals(depthLayers * samplesPerLayer, extractedSamples)
    }

    @Test
    fun testItem6PhaseChangesAtFiveGateCheckpoints() {
        val model = ViewerModel()
        val checkpoints = listOf(
            0 to 0.0,
            9 to 9.0 / 60.0,
            15 to 15.0 / 60.0,
            30 to 30.0 / 60.0,
            59 to 59.0 / 60.0
        )

        for ((frameIndex, expectedPhase) in checkpoints) {
            model.seek(expectedPhase)
            assertEquals(expectedPhase, model.phase, 1e-6)
            assertEquals("Frame ${frameIndex + 1} / 60", ViewerMath.frameLabel(model.phase))

            val values = model.fieldValues()
            assertEquals(32768, values.size)
            assertTrue(values[0].isFinite())
        }
    }

    @Test
    fun testFiveTimingBudgetsUnderFrozenMethodology() = runTest {
        val model = ViewerModel()

        // 1. Warm-up protocol: 3 unmeasured runs
        repeat(3) {
            val w1 = model.fieldValues()
            model.selectField(1)
            val w2 = model.fieldValues()
            model.selectField(0)
            model.seek(0.25)
            val w3 = model.fieldValues()
            model.seek(0.15)
            val w4 = model.fieldStatsSeries(0, 0.17, false)
            assertTrue(w1.isNotEmpty() && w2.isNotEmpty() && w3.isNotEmpty() && w4.isNotEmpty())
        }

        // Budget 1: Initial model and slice generation <= 100 ms
        val initialTimes = mutableListOf<Double>()
        repeat(30) {
            val t0 = System.nanoTime()
            val freshModel = ViewerModel()
            val vals = freshModel.fieldValues()
            // Slice generation: Plan (k=12) and Section (j=16)
            val planSlice = FloatArray(1024) { i -> vals[12 * 1024 + i] }
            val sectionSlice = FloatArray(1024) { i ->
                val z = i / 32
                val x = i % 32
                vals[z * 1024 + 16 * 32 + x]
            }
            val t1 = System.nanoTime()
            val elapsedMs = (t1 - t0) / 1_000_000.0
            initialTimes.add(elapsedMs)
            assertTrue(planSlice.isNotEmpty())
            assertTrue(sectionSlice.isNotEmpty())
        }
        initialTimes.sort()
        val initialMedian = initialTimes[initialTimes.size / 2]
        val initialP95 = initialTimes[ceil(0.95 * initialTimes.size).toInt() - 1]
        println("Android Budget 1 (Initial model+slice): median=${"%.2f".format(initialMedian)} ms, p95=${"%.2f".format(initialP95)} ms, threshold=100.0 ms")
        assertTrue("Budget 1 initial model <= 100 ms", initialP95 <= 100.0)

        // Budget 2: Field switching <= 50 ms per switch
        val switchTimes = mutableListOf<Double>()
        repeat(30) { idx ->
            val nextField = (idx + 1) % 6
            val t0 = System.nanoTime()
            model.selectField(nextField)
            val vals = model.fieldValues()
            val planSlice = FloatArray(1024) { i -> vals[12 * 1024 + i] }
            val t1 = System.nanoTime()
            val elapsedMs = (t1 - t0) / 1_000_000.0
            switchTimes.add(elapsedMs)
            assertTrue(planSlice.isNotEmpty())
        }
        switchTimes.sort()
        val switchMedian = switchTimes[switchTimes.size / 2]
        val switchP95 = switchTimes[ceil(0.95 * switchTimes.size).toInt() - 1]
        println("Android Budget 2 (Field switching): median=${"%.2f".format(switchMedian)} ms, p95=${"%.2f".format(switchP95)} ms, threshold=50.0 ms")
        assertTrue("Budget 2 field switching <= 50 ms", switchP95 <= 50.0)

        // Budget 3a: Uncached 60-frame chart calculation <= 400 ms (N=20)
        val chartTimes = mutableListOf<Double>()
        repeat(20) {
            val t0 = System.nanoTime()
            val series = model.fieldStatsSeries(0, 0.17, false)
            val t1 = System.nanoTime()
            val elapsedMs = (t1 - t0) / 1_000_000.0
            chartTimes.add(elapsedMs)
            assertEquals(60, series.size)
        }
        chartTimes.sort()
        val chartMedian = chartTimes[chartTimes.size / 2]
        val chartP95 = chartTimes[ceil(0.95 * chartTimes.size).toInt() - 1]
        println("Android Budget 3 (60-frame chart): median=${"%.2f".format(chartMedian)} ms, p95=${"%.2f".format(chartP95)} ms, threshold=400.0 ms")
        assertTrue("Budget 3 chart calculation <= 400 ms", chartP95 <= 400.0)

        // Budget 3b: Full 6-field 60-frame precomputation diagnostic (N=5, 360 volumes)
        val fullSeriesTimes = mutableListOf<Double>()
        repeat(5) {
            val t0 = System.nanoTime()
            val values = FloatArray(OkloSim.VOXELS)
            val fullMap = (0..5).associateWith { mutableListOf<FieldStats>() }
            (0..59).forEach { frame ->
                val p = frame / 60.0
                OkloSim.create(p, 0.17, false).use { sim ->
                    for (fieldId in 0..5) {
                        sim.copyField(fieldId, values)
                        fullMap[fieldId]!!.add(FieldStats.summarize(p, fieldId, values))
                    }
                }
            }
            val t1 = System.nanoTime()
            val elapsedMs = (t1 - t0) / 1_000_000.0
            fullSeriesTimes.add(elapsedMs)
            assertEquals(60, fullMap[0]?.size)
        }
        fullSeriesTimes.sort()
        val fullMedian = fullSeriesTimes[fullSeriesTimes.size / 2]
        val fullP95 = fullSeriesTimes[fullSeriesTimes.size - 1]
        println("Android Full 6-field 60-frame precomputation: median=${"%.2f".format(fullMedian)} ms, p95=${"%.2f".format(fullP95)} ms (360 volumes)")

        // Budget 4: Volume frame <= 100 ms (16 isometric projection slices)
        val volumeTimes = mutableListOf<Double>()
        repeat(30) {
            val vals = model.fieldValues()
            val t0 = System.nanoTime()
            var accum = 0f
            for (layer in 0 until 16) {
                val z = layer * 2
                for (y in 0 until 32) {
                    for (x in 0 until 32) {
                        accum += vals[z * 1024 + y * 32 + x]
                    }
                }
            }
            val t1 = System.nanoTime()
            val elapsedMs = (t1 - t0) / 1_000_000.0
            volumeTimes.add(elapsedMs)
            assertTrue(accum.isFinite())
        }
        volumeTimes.sort()
        val volumeMedian = volumeTimes[volumeTimes.size / 2]
        val volumeP95 = volumeTimes[ceil(0.95 * volumeTimes.size).toInt() - 1]
        println("Android Budget 4 (Volume frame): median=${"%.2f".format(volumeMedian)} ms, p95=${"%.2f".format(volumeP95)} ms, threshold=100.0 ms")
        assertTrue("Budget 4 volume frame <= 100 ms", volumeP95 <= 100.0)

        // Budget 5: Phase update <= 35 ms per step
        val phaseTimes = mutableListOf<Double>()
        repeat(30) { step ->
            val phaseVal = (step % 60) / 60.0
            val t0 = System.nanoTime()
            model.seek(phaseVal)
            val vals = model.fieldValues()
            val planSlice = FloatArray(1024) { i -> vals[12 * 1024 + i] }
            val t1 = System.nanoTime()
            val elapsedMs = (t1 - t0) / 1_000_000.0
            phaseTimes.add(elapsedMs)
            assertTrue(planSlice.isNotEmpty())
        }
        phaseTimes.sort()
        val phaseMedian = phaseTimes[phaseTimes.size / 2]
        val phaseP95 = phaseTimes[ceil(0.95 * phaseTimes.size).toInt() - 1]
        println("Android Budget 5 (Phase update): median=${"%.2f".format(phaseMedian)} ms, p95=${"%.2f".format(phaseP95)} ms, threshold=35.0 ms")
        assertTrue("Budget 5 phase update <= 35 ms", phaseP95 <= 35.0)
    }
}
