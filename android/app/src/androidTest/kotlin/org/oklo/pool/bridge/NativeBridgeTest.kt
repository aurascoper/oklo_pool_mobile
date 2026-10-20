package org.oklo.pool.bridge

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class NativeBridgeTest {
    @Test fun abiVersionAndSixFieldsLoad() {
        assertEquals(1, OkloCoreNative.abiVersion())
        OkloSim.create(0.15, 0.17, false).use { sim ->
            for (field in 0..5) {
                val values = FloatArray(OkloSim.VOXELS)
                assertEquals(OkloSim.VOXELS, sim.copyField(field, values))
                assertEquals(OkloSim.VOXELS, values.size)
            }
        }
    }

    @Test fun invalidConfigIsRejected() {
        assertThrows(IllegalArgumentException::class.java) { OkloSim.create(Double.NaN, 0.17, false) }
        assertEquals(0L, OkloCoreNative.create(0, 32, 32, 0, 0.15, 0.17, 10_800.0))
        assertEquals(0L, OkloCoreNative.create(32, 32, 32, 2, 0.15, 0.17, 10_800.0))
    }

    @Test fun steppingWrapsAndRejectsNegativeTime() {
        OkloSim.create(0.15, 0.17, false).use { sim ->
            assertEquals(0.15, sim.step(10_800.0), 1e-12)
            assertThrows(IllegalStateException::class.java) { sim.step(-1.0) }
        }
    }

    @Test fun shortDestinationIsRejected() {
        OkloSim.create(0.15, 0.17, false).use { sim ->
            assertThrows(IllegalStateException::class.java) { sim.copyField(0, FloatArray(16)) }
        }
    }

    @Test fun closeIsIdempotentAtWrapperBoundary() {
        val sim = OkloSim.create(0.15, 0.17, false)
        sim.close()
        sim.close()
    }

    @Test fun cf2TwentyCaseMatrixMatchesReference() {
        val phases = doubleArrayOf(0.05, 0.15, 0.20, 0.50, 0.90)
        val scenarios = booleanArrayOf(false, true)
        val boundaries = doubleArrayOf(0.1667, 0.17)
        val expectedHashes = arrayOf(
            // Case 01: p=0.05, active, b=0.1667
            "f609935a9b5a15d6e39d5342419dfae6d3680ceec776a8d7c1f7636815d93c60",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "b84d053cc1ac314d1b5f8cdb515e20c94d39400decf8fed55993f123c3db8dc8",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 02: p=0.05, active, b=0.1700
            "e17f90fbce88efb9db513b888e749a9ab47649ae7f6657eb1ea3e9eae757b470",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "b84d053cc1ac314d1b5f8cdb515e20c94d39400decf8fed55993f123c3db8dc8",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 03: p=0.05, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 04: p=0.05, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 05: p=0.15, active, b=0.1667
            "a300fe35ef3c0dc68641b673e9423edda74acd100ecc7a49768d9ebaabac429b",
            "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
            "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 06: p=0.15, active, b=0.1700
            "89d85fc802e4f4d638af529f66bf7f3eb73b14e704afaf31dc652b68c9cb8e47",
            "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
            "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 07: p=0.15, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 08: p=0.15, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 09: p=0.20, active, b=0.1667
            "bc44b77b1acb2b28f33556085c86e5882cfe0c26e230a8c7c615d485964a4c20",
            "58e4ed5ee9fba7a1356dffdaf7ad0d10d7665f3b06324849586fcf7f364a6e55",
            "735dd208dc14ecd45b253c6196447972c9e7536ec5e2adac052a9b5aa2266d85",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "9dd41d53f1c7f245d6008604bc3bb0fdacf5ba23cdb21a2fce4797415820f2f0",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 10: p=0.20, active, b=0.1700
            "6b4d31f8922bf4380532e654aba8fd1c67bb2d328045388459f6c81edac8199b",
            "58e4ed5ee9fba7a1356dffdaf7ad0d10d7665f3b06324849586fcf7f364a6e55",
            "735dd208dc14ecd45b253c6196447972c9e7536ec5e2adac052a9b5aa2266d85",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "9dd41d53f1c7f245d6008604bc3bb0fdacf5ba23cdb21a2fce4797415820f2f0",
            "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
            // Case 11: p=0.20, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 12: p=0.20, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 13: p=0.50, active, b=0.1667
            "3f8880551cd6908cbc1ac654c5c5ebed6b8444426deec500039d761921c353b2",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "2eca2559341f08a6e8f6a27a265faf83b9e12e0252ec7a2a3d988f419b476a36",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 14: p=0.50, active, b=0.1700
            "a8e62702525f9fec2aebb54b9b232434b8aae80f037b9bc7eaa475cc3a4250ef",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "2eca2559341f08a6e8f6a27a265faf83b9e12e0252ec7a2a3d988f419b476a36",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 15: p=0.50, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 16: p=0.50, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 17: p=0.90, active, b=0.1667
            "0075d6415381e0ffc044b88e411581ac679226b4ac24ef79921a7f14208e803d",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "a5d9f05ab945fe3a55904445503c1de18f2909740032e57860f1d9fb30f65486",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 18: p=0.90, active, b=0.1700
            "a6a96cb207606c2a69dfbca061ab1f527709674926393b9ad18070dc3a7e5426",
            "25cc5e3b9780c662260bdeac839dacfc6efea9973c438c1015b9b2123d2f67df",
            "c9c0a9066b8fb41065c424cd8ac6d8786ccf5ea6248fb53d85753a8c8747cb78",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "a5d9f05ab945fe3a55904445503c1de18f2909740032e57860f1d9fb30f65486",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 19: p=0.90, null, b=0.1667
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7",
            // Case 20: p=0.90, null, b=0.1700
            "f070c96edce3512fe10a9869e067a43dc020940a918c492738a15d965f273670",
            "aef144e835222f02f8844b3c1cfab7d5a66bc8bf3009ddaa9b9c313e50eb68f9",
            "d1058699e2f4e952bfd0aa4ce44b3ed76e5bee28f1887c9e9cb4962fd3bbda50",
            "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "fa43239bcee7b97ca62f007cc68487560a39e19f74f3dde7486db3f98df8e471",
            "6c9975d323371e57af506d8976efde9a735c50c197919be426c92e22c06566d7"
        )
        var idx = 0
        val values = FloatArray(OkloSim.VOXELS)
        val bytes = java.nio.ByteBuffer.allocate(OkloSim.VOXELS * 4).order(java.nio.ByteOrder.LITTLE_ENDIAN)
        val digest = java.security.MessageDigest.getInstance("SHA-256")
        for (phase in phases) {
            for (nullScenario in scenarios) {
                for (boundary in boundaries) {
                    OkloSim.create(phase, boundary, nullScenario).use { sim ->
                        for (field in 0..5) {
                            assertEquals(OkloSim.VOXELS, sim.copyField(field, values))
                            bytes.clear()
                            bytes.asFloatBuffer().put(values)
                            val hash = digest.digest(bytes.array()).joinToString("") { "%02x".format(it) }
                            assertEquals("Mismatch at index $idx (p=$phase, null=$nullScenario, b=$boundary, f=$field)", expectedHashes[idx], hash)
                            idx++
                        }
                    }
                }
            }
        }
        assertEquals(120, idx)
    }
}
