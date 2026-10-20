package org.oklo.pool

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.viewmodel.compose.viewModel
import kotlinx.coroutines.delay
import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent { MaterialTheme { ViewerScreen() } }
    }
}

private enum class DisplayMode { Plan, Section, Volume, Chart }

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun ViewerScreen(model: ViewerModel = viewModel()) {
    var playing by remember { mutableStateOf(false) }
    var showProvenance by remember { mutableStateOf(false) }
    var mode by remember { mutableStateOf(DisplayMode.Plan) }
    var fieldMenu by remember { mutableStateOf(false) }
    var voxelInfo by remember { mutableStateOf<String?>(null) }
    var chartStats by remember { mutableStateOf(emptyList<FieldStats>()) }
    val lifecycle = LocalLifecycleOwner.current.lifecycle

    DisposableEffect(lifecycle) {
        val observer = LifecycleEventObserver { _, event ->
            if (event == Lifecycle.Event.ON_STOP || event == Lifecycle.Event.ON_PAUSE) playing = false
        }
        lifecycle.addObserver(observer)
        onDispose { lifecycle.removeObserver(observer) }
    }
    LaunchedEffect(playing) {
        while (playing) {
            delay(500)
            model.stepFrame()
        }
    }
    LaunchedEffect(mode, model.fieldId, model.pulseBoundary, model.nullScenario) {
        if (mode == DisplayMode.Chart) {
            chartStats = model.fieldStatsSeries(model.fieldId, model.pulseBoundary, model.nullScenario)
        }
    }

    val field = FixtureField.all[model.fieldId]
    val result = remember(model.phase, model.pulseBoundary, model.nullScenario, model.fieldId) {
        runCatching { model.fieldValues() }
    }
    val values = result.getOrElse { FloatArray(32 * 32 * 32) }
    val low = values.minOrNull() ?: 0f
    val high = values.maxOrNull() ?: 1f
    val mean = if (values.isNotEmpty()) values.average() else 0.0

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Oklo Fixture Viewer") },
                actions = { TextButton(onClick = { showProvenance = true }) { Text("Scope") } }
            )
        }
    ) { padding ->
        Column(
            Modifier.fillMaxSize().padding(padding).verticalScroll(rememberScrollState()).padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            Text("Revision 0 prescribed fixture", style = MaterialTheme.typography.titleMedium)
            result.exceptionOrNull()?.let {
                Text("Unable to load fixture data: ${it.message ?: "native core error"}", color = MaterialTheme.colorScheme.error)
            }
            Box {
                OutlinedButton(onClick = { fieldMenu = true }, modifier = Modifier.fillMaxWidth()) {
                    Text("${field.title}  •  ${field.unit}")
                }
                DropdownMenu(expanded = fieldMenu, onDismissRequest = { fieldMenu = false }) {
                    FixtureField.all.forEach { item ->
                        DropdownMenuItem(
                            text = { Text("${item.title} (${item.unit})") },
                            onClick = { model.selectField(item.id); fieldMenu = false }
                        )
                    }
                }
            }

            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                DisplayMode.entries.forEach { option ->
                    FilterChip(
                        selected = mode == option,
                        onClick = { mode = option },
                        label = { Text(option.name) }
                    )
                }
            }

            if (mode == DisplayMode.Plan) {
                Text("Plan depth k = ${model.planDepth}")
                Slider(value = model.planDepth.toFloat(), onValueChange = { model.updatePlanDepth(it.toInt()) }, valueRange = 0f..31f, steps = 30, modifier = Modifier.semantics { contentDescription = "Plan slice depth" })
            } else if (mode == DisplayMode.Section) {
                Text("Section row j = ${model.sectionRow}")
                Slider(value = model.sectionRow.toFloat(), onValueChange = { model.updateSectionRow(it.toInt()) }, valueRange = 0f..31f, steps = 30, modifier = Modifier.semantics { contentDescription = "Section slice row" })
            }

            if (mode == DisplayMode.Chart) {
                PrescribedChart(field, model.phase, model.pulseBoundary, model.nullScenario, chartStats)
                if (model.fieldId == 5) {
                    val selected = chartStats.minByOrNull { abs(it.phase - model.phase) }
                    Text("Nearest frame zone 3 share: ${"%.1f".format((selected?.zone3Share ?: 0f) * 100f)}%; zone 4 share: ${"%.1f".format((selected?.zone4Share ?: 0f) * 100f)}%")
                } else {
                    Text("Current field range: ${"%.4g".format(low)} to ${"%.4g".format(high)} ${field.unit}")
                    Text("Current field mean: ${"%.5g".format(mean)}")
                }
            } else {
                Text(
                    when (mode) {
                        DisplayMode.Plan -> "Plan slice, X by Y at Z = ${model.planDepth}"
                        DisplayMode.Section -> "Section slice, X by Z at Y = ${model.sectionRow}"
                        DisplayMode.Volume -> "Volume projection, 16 isometric depth layers"
                        DisplayMode.Chart -> ""
                    },
                    fontWeight = FontWeight.SemiBold
                )
                FieldCanvas(values, low, high, model.fieldId, mode, model.planDepth, model.sectionRow, Modifier.fillMaxWidth().height(300.dp)) { x, y, z, value ->
                    voxelInfo = "Voxel (x=$x, y=$y, z=$z): ${"%.6g".format(value)} ${field.unit}"
                }
                voxelInfo?.let { Text(it, Modifier.semantics { contentDescription = it }) }
                Text("Range ${"%.4g".format(low)} to ${"%.4g".format(high)} ${field.unit}   Mean ${"%.5g".format(mean)}")
            }

            Text("Phase ${"%.4f".format(model.phase)}  •  Frame ${((model.phase * 60).toInt() % 60) + 1} / 60")
            Row(verticalAlignment = Alignment.CenterVertically) {
                Button(onClick = { playing = !playing }, modifier = Modifier.heightIn(min = 48.dp)) {
                    Text(if (playing) "Pause" else "Play")
                }
                Spacer(Modifier.width(12.dp))
                Slider(
                    value = model.phase.toFloat(),
                    onValueChange = { model.seek(it.toDouble()) },
                    valueRange = 0f..1f,
                    steps = 58,
                    modifier = Modifier.weight(1f).semantics { contentDescription = "Cycle phase scrubber" }
                )
            }
            Text("Elapsed at inherited 3 hour scale: ${"%.1f".format(model.phase * 180.0)} minutes")
            Text("Pulse boundary")
            Row(verticalAlignment = Alignment.CenterVertically) {
                listOf(0.17, 0.1667).forEach { boundary ->
                    Row(
                        Modifier.selectable(
                            selected = model.pulseBoundary == boundary,
                            role = Role.RadioButton,
                            onClick = { model.setBoundary(boundary) }
                        ).padding(end = 16.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        RadioButton(selected = model.pulseBoundary == boundary, onClick = null)
                        Text(boundary.toString())
                    }
                }
            }
            Row(
                Modifier.toggleable(
                    value = model.nullScenario,
                    role = Role.Checkbox,
                    onValueChange = model::setNull
                ),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Checkbox(checked = model.nullScenario, onCheckedChange = null)
                Text("Null scenario (pulse and drying disabled)")
            }
            Text(
                "Phase navigation follows prescribed functions. It does not solve physical dynamics.",
                style = MaterialTheme.typography.bodySmall
            )
        }
    }

    if (showProvenance) {
        ModalBottomSheet(
            onDismissRequest = { showProvenance = false },
            sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)
        ) {
            Column(
                Modifier.fillMaxWidth().verticalScroll(rememberScrollState()).padding(24.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Text("Assumptions and provenance", style = MaterialTheme.typography.titleLarge)
                Text("This app is an educational scenario explorer. Every number it draws comes from a fixed formula chosen to give a plausible shape, not from measurements of Oklo and not from a physical simulation. The formulas are frozen so that the app, the original web viewer and the exported data files can be checked against each other.")
                Text("The natural fission reactors at Oklo in Gabon operated roughly two billion years ago, long after the earliest evidence of life on Earth. The pool shown here is therefore an analog scenario for ideas about early chemistry, not a reconstruction of a place where life began. The idea that wet and dry cycles at a natural reactor could drive the joining of small molecules is a published hypothesis, not an established finding.")
                Text("The roughly three-hour on and off cycle comes from the xenon record studied by Meshik and colleagues in 2004, and it has been reproduced in a computer model by Ibekwe and colleagues in 2020. The hydrogen map shows where radiolysis would happen, not how much hydrogen formed. Temperatures, water content and molecule amounts are illustrative indices. Units are shown as the original viewer labelled them, and several of those units have no derivation yet.")
                Text("Polymer accumulation index (arbitrary units): Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.")
                Spacer(Modifier.height(18.dp))
            }
        }
    }
}

@Composable
private fun FieldCanvas(
    data: FloatArray, minValue: Float, maxValue: Float, fieldId: Int,
    mode: DisplayMode, planDepth: Int, sectionRow: Int, modifier: Modifier = Modifier,
    onVoxelSelected: (Int, Int, Int, Float) -> Unit
) {
    val tapModifier = if (mode == DisplayMode.Volume) Modifier else Modifier.pointerInput(data, mode, planDepth, sectionRow) {
        detectTapGestures { point ->
            val x = (point.x / size.width * 32).toInt().coerceIn(0, 31)
            val y = (point.y / size.height * 32).toInt().coerceIn(0, 31)
            val (voxelY, voxelZ) = if (mode == DisplayMode.Plan) y to planDepth else sectionRow to y
            val value = data[voxelZ * 1024 + voxelY * 32 + x]
            onVoxelSelected(x, voxelY, voxelZ, value)
        }
    }
    Canvas(modifier.then(tapModifier).background(Color(0xFF101820), RoundedCornerShape(8.dp)).semantics {
        contentDescription = "${mode.name} visualization of ${FixtureField.all[fieldId].title}"
    }) {
        val span = max(1e-12f, maxValue - minValue)
        when (mode) {
            DisplayMode.Plan, DisplayMode.Section -> {
                val cellX = size.width / 32f
                val cellY = size.height / 32f
                for (y in 0 until 32) for (x in 0 until 32) {
                    val index = when (mode) {
                        DisplayMode.Plan -> planDepth * 1024 + y * 32 + x
                        DisplayMode.Section -> y * 1024 + sectionRow * 32 + x
                        else -> 0
                    }
                    drawRect(valueColor(data.getOrElse(index) { 0f }, minValue, span, fieldId), Offset(x * cellX, y * cellY), Size(cellX + 0.5f, cellY + 0.5f))
                }
                drawLine(Color.White.copy(alpha = .8f), Offset(size.width / 2f, 0f), Offset(size.width / 2f, size.height), strokeWidth = 1f)
                drawLine(Color.White.copy(alpha = .8f), Offset(0f, size.height / 2f), Offset(size.width, size.height / 2f), strokeWidth = 1f)
            }
            DisplayMode.Volume -> {
                val cell = min(size.width, size.height) / 46f
                for (k in 30 downTo 0 step 2) for (y in 0 until 32) for (x in 0 until 32) {
                    val px = size.width / 2f + (x - y) * cell * .55f
                    val py = size.height * .78f - (x + y) * cell * .25f - k * cell * .32f
                    val index = k * 1024 + y * 32 + x
                    drawRect(valueColor(data.getOrElse(index) { 0f }, minValue, span, fieldId), Offset(px, py), Size(cell * .62f, cell * .48f))
                }
            }
            DisplayMode.Chart -> Unit
        }
    }
}

private fun valueColor(value: Float, minValue: Float, span: Float, field: Int): Color {
    if (field == 5) return listOf(Color(0xFFD8C79F), Color(0xFF6B2D82), Color(0xFF8EA86C), Color(0xFF3C7FC0), Color(0xFFF0EEE8))[value.toInt().coerceIn(0, 4)]
    val t = ((value - minValue) / span).coerceIn(0f, 1f)
    return when (field) {
        0 -> Color(red = t, green = .18f + .68f * t, blue = .1f + .12f * (1f - t))
        1 -> Color(red = .08f + .6f * t, green = .25f + .65f * t, blue = .45f + .5f * t)
        2 -> Color(red = .2f + .1f * t, green = .15f + .8f * t, blue = .3f + .5f * (1f - t))
        3 -> Color(red = .1f + .8f * (1f - t), green = .25f + .55f * (1f - t), blue = .5f + .4f * t)
        else -> Color(red = .2f + .8f * t, green = .06f + .35f * t, blue = .35f + .4f * t)
    }
}

@Composable
private fun PrescribedChart(field: FixtureField, phase: Double, boundary: Double, nullScenario: Boolean, fieldStats: List<FieldStats>) {
    Text(if (field.id == 5) "${field.title}: zone 3 and zone 4 shares" else "${field.title}: mean and min/max envelope (${field.unit})", fontWeight = FontWeight.SemiBold)
    Canvas(Modifier.fillMaxWidth().height(170.dp).background(Color(0xFF101820)).semantics {
        contentDescription = if (field.id == 5) "Compartments zone 3 and zone 4 shares over cycle phase" else "${field.title} minimum, mean, and maximum over cycle phase"
    }) {
        val left = 30f
        val right = size.width - 12f
        val top = 12f
        val bottom = size.height - 18f
        drawLine(Color.LightGray, Offset(left, bottom), Offset(right, bottom), 2f)
        drawLine(Color.LightGray, Offset(left, top), Offset(left, bottom), 2f)
        if (fieldStats.isNotEmpty()) {
            val xAt: (FieldStats) -> Float = { left + (right - left) * it.phase.toFloat() }
            if (field.id == 5) {
                fun zonePath(color: Color, selector: (FieldStats) -> Float) {
                    val path = Path()
                    fieldStats.forEachIndexed { index, item ->
                        val y = bottom - (bottom - top) * selector(item).coerceIn(0f, 1f)
                        if (index == 0) path.moveTo(xAt(item), y) else path.lineTo(xAt(item), y)
                    }
                    drawPath(path, color, style = Stroke(2f))
                }
                zonePath(Color(0xFF64B5F6)) { it.zone3Share }
                zonePath(Color(0xFFFFB74D)) { it.zone4Share }
            } else {
                val statLow = fieldStats.minOf { it.min }
                val statHigh = fieldStats.maxOf { it.max }
                val statSpan = max(1e-12f, statHigh - statLow)
                fun statY(value: Float): Float = bottom - (bottom - top) * ((value - statLow) / statSpan).coerceIn(0f, 1f)
                val envelope = Path()
                fieldStats.forEachIndexed { index, item ->
                    val point = Offset(xAt(item), statY(item.max))
                    if (index == 0) envelope.moveTo(point.x, point.y) else envelope.lineTo(point.x, point.y)
                }
                fieldStats.asReversed().forEach { item -> envelope.lineTo(xAt(item), statY(item.min)) }
                envelope.close()
                drawPath(envelope, Color(0xFF81C784).copy(alpha = .2f))
                fun statPath(color: Color, selector: (FieldStats) -> Float) {
                    val path = Path()
                    fieldStats.forEachIndexed { index, item ->
                        val x = xAt(item)
                        val y = statY(selector(item))
                        if (index == 0) path.moveTo(x, y) else path.lineTo(x, y)
                    }
                    drawPath(path, color, style = Stroke(2f))
                }
                statPath(Color(0xFFFFF176)) { it.min }
                statPath(Color(0xFF81C784)) { it.mean }
                statPath(Color(0xFFE57373)) { it.max }
            }
        }
        val x = left + (right - left) * phase.toFloat()
        drawLine(Color.White, Offset(x, top), Offset(x, bottom), 2f)
    }
    Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
        if (field.id == 5) {
            Text("Zone 3", color = Color(0xFF64B5F6))
            Text("Zone 4", color = Color(0xFFFFB74D))
        } else {
            Text("Min", color = Color(0xFFFFF176))
            Text("Mean", color = Color(0xFF81C784))
            Text("Max", color = Color(0xFFE57373))
        }
    }
    Text("Prescribed driver curves", fontWeight = FontWeight.SemiBold)
    Canvas(Modifier.fillMaxWidth().height(145.dp).background(Color(0xFF101820)).semantics {
        contentDescription = "Prescribed pulse and desiccation curves over cycle phase"
    }) {
        val left = 30f
        val right = size.width - 12f
        val top = 12f
        val bottom = size.height - 18f
        drawLine(Color.LightGray, Offset(left, bottom), Offset(right, bottom), 2f)
        drawLine(Color.LightGray, Offset(left, top), Offset(left, bottom), 2f)
        fun drawCurve(color: Color, fn: (Double) -> Double) {
            val path = Path()
            for (index in 0..120) {
                val p = index / 120.0
                val x = left + (right - left) * p.toFloat()
                val y = bottom - (bottom - top) * fn(p).toFloat().coerceIn(0f, 1f)
                if (index == 0) path.moveTo(x, y) else path.lineTo(x, y)
            }
            drawPath(path, color, style = Stroke(3f))
        }
        drawCurve(Color(0xFFFF7043)) { p -> if (nullScenario) 0.0 else ViewerMath.pulse(p, boundary) }
        drawCurve(Color(0xFF64B5F6)) { p -> if (nullScenario) 0.0 else ViewerMath.desiccation(p) }
        val x = left + (right - left) * phase.toFloat()
        drawLine(Color.White, Offset(x, top), Offset(x, bottom), 2f)
    }
    Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
        Text("P(p)", color = Color(0xFFFF7043))
        Text("D(p)", color = Color(0xFF64B5F6))
    }
}
