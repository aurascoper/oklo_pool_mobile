package org.oklo.pool

import androidx.lifecycle.ViewModel
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import org.oklo.pool.bridge.OkloSim
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlin.math.PI
import kotlin.math.exp
import kotlin.math.roundToInt
import kotlin.math.sin

data class FixtureField(val id: Int, val title: String, val unit: String) {
    companion object {
        val all = listOf(
            FixtureField(0, "Temperature", "degrees C"),
            FixtureField(1, "Hydration", "dimensionless"),
            FixtureField(2, "H2 concentration", "index, labelled mmol/L"),
            FixtureField(3, "235U/238U ratio", "ratio"),
            FixtureField(4, "Polymer accumulation index", "arbitrary units"),
            FixtureField(5, "Compartments", "zone 0 to 4")
        )
    }
}

data class FieldStats(
    val phase: Double,
    val min: Float,
    val mean: Float,
    val max: Float,
    val zone3Share: Float,
    val zone4Share: Float
) {
    companion object {
        fun summarize(phase: Double, fieldId: Int, values: FloatArray): FieldStats {
            require(values.isNotEmpty())
            var minVal = Float.POSITIVE_INFINITY
            var maxVal = Float.NEGATIVE_INFINITY
            var sum = 0.0
            var z3 = 0
            var z4 = 0
            val isCompartments = (fieldId == 5)
            for (v in values) {
                if (v < minVal) minVal = v
                if (v > maxVal) maxVal = v
                sum += v.toDouble()
                if (isCompartments) {
                    val zi = v.toInt()
                    if (zi == 3) z3++ else if (zi == 4) z4++
                }
            }
            val voxelCount = values.size.toFloat()
            return FieldStats(
                phase,
                if (minVal.isFinite()) minVal else 0f,
                (sum / values.size).toFloat(),
                if (maxVal.isFinite()) maxVal else 0f,
                if (isCompartments) z3 / voxelCount else 0f,
                if (isCompartments) z4 / voxelCount else 0f
            )
        }
    }
}

object ViewerMath {
    const val FRAME_COUNT = 60
    const val MAX_FRAME_INDEX = FRAME_COUNT - 1
    const val SLIDER_STEPS = MAX_FRAME_INDEX - 1
    const val MAX_PHASE = MAX_FRAME_INDEX / 60.0
    val SLIDER_VALUE_RANGE = 0f..(MAX_FRAME_INDEX / 60f)

    fun clampPhase(phase: Double): Double = phase.coerceIn(0.0, MAX_PHASE)

    fun selectablePhases(): List<Double> =
        (0..MAX_FRAME_INDEX).map { index ->
            val fraction = index.toDouble() / (SLIDER_STEPS + 1).toDouble()
            clampPhase(fraction * MAX_PHASE)
        }

    fun snapSliderPhase(sliderValue: Float): Double {
        val clamped = sliderValue.toDouble().coerceIn(0.0, MAX_PHASE)
        val frameIndex = (clamped * FRAME_COUNT).roundToInt().coerceIn(0, MAX_FRAME_INDEX)
        return frameIndex / 60.0
    }

    fun frameNumber(phase: Double): Int =
        ((clampPhase(phase) * FRAME_COUNT).roundToInt().coerceIn(0, MAX_FRAME_INDEX)) + 1

    fun frameLabel(phase: Double): String =
        "Frame ${frameNumber(phase)} / $FRAME_COUNT"

    fun nextPhase(phase: Double, dtSeconds: Double, periodSeconds: Double = 10_800.0): Double {
        require(phase.isFinite() && phase in 0.0..1.0)
        require(dtSeconds.isFinite() && dtSeconds >= 0.0)
        require(periodSeconds.isFinite() && periodSeconds > 0.0)
        return (phase + dtSeconds / periodSeconds) % 1.0
    }

    fun pulse(phase: Double, boundary: Double): Double =
        if (phase < boundary) sin(PI * phase / boundary) else exp(-8.0 * (phase - boundary))

    fun desiccation(phase: Double): Double =
        if (phase in 0.10..0.35) sin(PI * (phase - 0.10) / 0.25) else 0.0
}

class ViewerModel : ViewModel() {
    var phase by mutableStateOf(0.15)
        private set
    var pulseBoundary by mutableStateOf(0.17)
        private set
    var nullScenario by mutableStateOf(false)
        private set
    var fieldId by mutableStateOf(0)
        private set
    var planDepth by mutableStateOf(12)
        private set
    var sectionRow by mutableStateOf(16)
        private set

    fun seek(value: Double) { phase = ViewerMath.clampPhase(value) }
    fun stepFrame() { phase = ViewerMath.nextPhase(phase, 180.0) }
    fun setBoundary(value: Double) { require(value == 0.17 || value == 0.1667); pulseBoundary = value }
    fun setNull(value: Boolean) { nullScenario = value }
    fun selectField(value: Int) { require(value in 0..5); fieldId = value }
    fun updatePlanDepth(value: Int) { planDepth = value.coerceIn(0, 31) }
    fun updateSectionRow(value: Int) { sectionRow = value.coerceIn(0, 31) }

    fun fieldValues(): FloatArray = OkloSim.create(phase, pulseBoundary, nullScenario).use { sim ->
        FloatArray(OkloSim.VOXELS).also { sim.copyField(fieldId, it) }
    }

    suspend fun fieldStatsSeries(id: Int, boundary: Double, nullCase: Boolean): List<FieldStats> =
        withContext(Dispatchers.Default) {
            val values = FloatArray(OkloSim.VOXELS)
            (0..59).map { frame ->
                val p = frame / 60.0
                OkloSim.create(p, boundary, nullCase).use { sim ->
                    sim.copyField(id, values)
                    FieldStats.summarize(p, id, values)
                }
            }
        }
}
