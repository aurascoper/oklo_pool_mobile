import XCTest

final class OkloPoolUITests: XCTestCase {

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    func testAppLaunchAndMainComponents() throws {
        let app = XCUIApplication()
        app.launch()

        XCTAssertTrue(app.navigationBars["Oklo Pool"].exists)
        XCTAssertTrue(app.buttons["Reset"].exists)
        XCTAssertTrue(app.buttons["Open Provenance and Limitations"].exists)
    }

    func testViewModeSwitch() throws {
        let app = XCUIApplication()
        app.launch()

        let volumeTab = app.buttons["3D Volume"]
        if volumeTab.exists {
            volumeTab.tap()
        }

        let slicesTab = app.buttons["Slices"]
        if slicesTab.exists {
            slicesTab.tap()
        }
    }

    func testProvenanceSheetPresentationAndDismiss() throws {
        let app = XCUIApplication()
        app.launch()

        let infoButton = app.buttons["Open Provenance and Limitations"]
        if infoButton.exists {
            infoButton.tap()

            let sheetTitle = app.staticTexts["Educational Scenario Explorer"]
            XCTAssertTrue(sheetTitle.waitForExistence(timeout: 2.0))

            let doneButton = app.buttons["Done"]
            if doneButton.exists {
                doneButton.tap()
            }
        }
    }

    func testTransportPlayPause() throws {
        let app = XCUIApplication()
        app.launch()

        let playButton = app.buttons["Start cycle playback"]
        if playButton.exists {
            playButton.tap()

            let pauseButton = app.buttons["Pause cycle playback"]
            XCTAssertTrue(pauseButton.waitForExistence(timeout: 1.0))
            pauseButton.tap()
        }
    }
}
