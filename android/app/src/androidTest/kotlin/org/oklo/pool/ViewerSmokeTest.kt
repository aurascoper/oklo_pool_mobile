package org.oklo.pool

import android.graphics.Bitmap
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsOff
import androidx.compose.ui.test.assertIsOn
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.lifecycle.Lifecycle
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import java.io.FileOutputStream
import org.junit.Rule
import org.junit.Test

class ViewerSmokeTest {
    @get:Rule val compose = createAndroidComposeRule<MainActivity>()

    private fun saveScreen(name: String) {
        compose.waitForIdle()
        Thread.sleep(400)
        val bmp = InstrumentationRegistry.getInstrumentation().uiAutomation.takeScreenshot() ?: return
        val dir = InstrumentationRegistry.getInstrumentation().targetContext.filesDir
        FileOutputStream(File(dir, "$name.png")).use { out ->
            bmp.compress(Bitmap.CompressFormat.PNG, 100, out)
        }
    }

    @Test fun allSixFieldsCanBeSelected() {
        compose.onNodeWithText("Oklo Fixture Viewer").assertIsDisplayed()
        compose.onNodeWithText("Temperature  •  degrees C").assertIsDisplayed()
        compose.onNodeWithText("Play").assertIsDisplayed()
        compose.onNodeWithText("Scope").assertIsDisplayed()

        val fields = listOf(
            "Temperature  •  degrees C" to "Temperature (degrees C)",
            "Hydration  •  dimensionless" to "Hydration (dimensionless)",
            "H2 concentration  •  index, labelled mmol/L" to "H2 concentration (index, labelled mmol/L)",
            "235U/238U ratio  •  ratio" to "235U/238U ratio (ratio)",
            "Polymer yield  •  index, labelled umol/L" to "Polymer yield (index, labelled umol/L)",
            "Compartments  •  zone 0 to 4" to "Compartments (zone 0 to 4)"
        )
        var currentSelected = "Temperature  •  degrees C"
        fields.forEach { (selected, menuItem) ->
            compose.onNodeWithText(currentSelected).performClick()
            compose.onNodeWithText(menuItem).performClick()
            compose.onNodeWithText(selected).assertIsDisplayed()
            currentSelected = selected
        }
    }

    @Test fun planSectionVolumeAndChartModesRenderLabels() {
        compose.onNodeWithContentDescription("Plan visualization of Temperature").assertIsDisplayed()
        saveScreen("android_plan_slice")

        compose.onNodeWithText("Section").performClick()
        compose.onNodeWithContentDescription("Section visualization of Temperature").assertIsDisplayed()
        compose.onNodeWithText("Section slice, X by Z at Y = 16").assertIsDisplayed()
        saveScreen("android_section_slice")

        compose.onNodeWithText("Volume").performClick()
        compose.onNodeWithContentDescription("Volume visualization of Temperature").assertIsDisplayed()
        saveScreen("android_volume")

        compose.onNodeWithText("Chart").performClick()
        compose.onNodeWithText("Temperature: mean and min/max envelope (degrees C)").assertIsDisplayed()
        compose.onNodeWithContentDescription("Prescribed pulse and desiccation curves over cycle phase").assertIsDisplayed()
        Thread.sleep(1500)
        saveScreen("android_chart")
    }

    @Test fun provenanceSheetOpensAndDisplaysLimitations() {
        compose.onNodeWithText("Scope").performClick()
        compose.onNodeWithText("Assumptions and provenance").assertIsDisplayed()
        saveScreen("android_provenance")
    }

    @Test fun playbackStopsWhenActivityLeavesForeground() {
        compose.onNodeWithText("Play").performClick()
        compose.onNodeWithText("Pause").assertIsDisplayed()

        compose.activityRule.scenario.moveToState(Lifecycle.State.CREATED)
        compose.activityRule.scenario.moveToState(Lifecycle.State.RESUMED)

        compose.onNodeWithText("Play").assertIsDisplayed()
    }

    @Test fun pulseAndNullControlsHaveAccessibleLabelsAndUpdate() {
        compose.onNodeWithText("0.17").performScrollTo().assertIsSelected()
        compose.onNodeWithText("0.1667").performScrollTo().performClick().assertIsSelected()

        val nullScenario = compose.onNodeWithText("Null scenario (pulse and drying disabled)")
        nullScenario.performScrollTo().assertIsOff()
        nullScenario.performScrollTo().performClick().assertIsOn()
    }
}
