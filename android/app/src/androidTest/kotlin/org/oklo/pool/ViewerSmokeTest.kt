package org.oklo.pool

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import org.junit.Rule
import org.junit.Test

class ViewerSmokeTest {
    @get:Rule val compose = createAndroidComposeRule<MainActivity>()

    @Test fun controlsAndAllFieldsAreAvailable() {
        compose.onNodeWithText("Oklo Fixture Viewer").assertIsDisplayed()
        compose.onNodeWithText("Temperature  •  degrees C").assertIsDisplayed()
        compose.onNodeWithText("Play").assertIsDisplayed()
        compose.onNodeWithText("Scope").assertIsDisplayed()
        compose.onNodeWithText("Temperature  •  degrees C").performClick()
        compose.onNodeWithText("Hydration (dimensionless)").assertIsDisplayed()
    }
}
