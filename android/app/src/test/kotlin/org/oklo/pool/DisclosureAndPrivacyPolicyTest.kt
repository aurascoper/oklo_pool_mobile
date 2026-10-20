package org.oklo.pool

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import java.io.File

class DisclosureAndPrivacyPolicyTest {

    @Test
    fun allSixFieldLimitationRecordsMatchSpecification() {
        assertEquals(6, fieldLimitationRecords.size)

        val expected = listOf(
            0 to ("Temperature" to "Prescribed thermal profile; not coupled to hydrodynamic or convective cooling."),
            1 to ("Hydration" to "Prescribed saturation index; not a multiphase porous flow solve."),
            2 to ("H2 Concentration" to "Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve."),
            3 to ("235U/238U Ratio" to "Prescribed isotopic depletion curve; not a 3D neutron transport solve."),
            4 to ("Polymer accumulation index" to "Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations."),
            5 to ("Compartments" to "Prescribed geometric bounding cells; not a physical membrane simulation.")
        )

        expected.forEach { (expectedId, pair) ->
            val (expectedTitle, expectedLimitation) = pair
            val record = fieldLimitationRecords[expectedId]
            assertEquals(expectedId, record.id)
            assertEquals(expectedTitle, record.title)
            assertEquals(expectedLimitation, record.limitation)
            assertTrue(record.unit.isNotBlank())
        }
    }

    @Test
    fun bundledPolicyContainsAuditSummaryAndLink() {
        assertTrue(bundledAndroidPrivacyPolicyText.isNotBlank())
        assertTrue(bundledAndroidPrivacyPolicyText.contains("Oklo Fixture Viewer - Offline Privacy Policy"))
        assertTrue(bundledAndroidPrivacyPolicyText.contains("does not collect, store, transmit, or share personal data"))
        assertTrue(bundledAndroidPrivacyPolicyText.contains("android.permission.INTERNET"))
        assertTrue(bundledAndroidPrivacyPolicyText.contains("zero open network sockets"))

        // Strict verification of zero em-dashes and zero en-dashes
        assertFalse("Must not contain en-dash", bundledAndroidPrivacyPolicyText.contains("\u2013"))
        assertFalse("Must not contain em-dash", bundledAndroidPrivacyPolicyText.contains("\u2014"))
    }

    @Test
    fun manifestContainsZeroInternetPermission() {
        val manifestFile = File("src/main/AndroidManifest.xml")
        if (manifestFile.exists()) {
            val content = manifestFile.readText()
            assertFalse("Must not contain android.permission.INTERNET", content.contains("android.permission.INTERNET"))
            assertFalse("Must not contain uses-permission", content.contains("<uses-permission"))
        }
    }
}
