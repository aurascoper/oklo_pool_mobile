package org.oklo.pool

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.lifecycle.Lifecycle
import org.junit.Rule
import org.junit.Test

class ViewerSmokeTest {
    @get:Rule val compose = createAndroidComposeRule<MainActivity>()

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
        fields.forEach { (selected, menuItem) ->
            compose.onNodeWithText(selected).performClick()
            compose.onNodeWithText(menuItem).performClick()
            compose.onNodeWithText(selected).assertIsDisplayed()
        }
    }

    @Test fun planSectionVolumeAndChartModesRenderLabels() {
        compose.onNodeWithContentDescription("Plan visualization of Temperature").assertIsDisplayed()

        compose.onNodeWithText("Section").performClick()
        compose.onNodeWithContentDescription("Section visualization of Temperature").assertIsDisplayed()
        compose.onNodeWithText("Section slice, X by Z at Y = 16").assertIsDisplayed()

        compose.onNodeWithText("Volume").performClick()
        compose.onNodeWithContentDescription("Volume visualization of Temperature").assertIsDisplayed()

        compose.onNodeWithText("Chart").performClick()
        compose.onNodeWithText("Temperature: mean and min/max envelope (degrees C)").assertIsDisplayed()
        compose.onNodeWithContentDescription("Prescribed pulse and desiccation curves over cycle phase").assertIsDisplayed()
    }

    @Test fun playbackStopsWhenActivityLeavesForeground() {
        compose.onNodeWithText("Play").performClick()
        compose.onNodeWithText("Pause").assertIsDisplayed()

        compose.activityRule.scenario.moveToState(Lifecycle.State.CREATED)
        compose.activityRule.scenario.moveToState(Lifecycle.State.RESUMED)

        compose.onNodeWithText("Play").assertIsDisplayed()
    }
}
