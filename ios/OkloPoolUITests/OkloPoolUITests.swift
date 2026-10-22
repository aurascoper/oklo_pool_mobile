import XCTest

final class OkloPoolUITests: XCTestCase {

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testAppLaunchAndMainComponents() throws {
        let app = XCUIApplication()
        app.launch()

        XCTAssertTrue(app.navigationBars["Oklo Pool"].exists)
        XCTAssertTrue(app.buttons["Reset controls to default values"].exists)
        XCTAssertTrue(app.buttons["Open Provenance and Limitations"].exists)
    }

    @MainActor
    func testViewModeSwitch() throws {
        let app = XCUIApplication()
        app.launch()

        let volumeTab = app.buttons["3D Volume"]
        XCTAssertTrue(volumeTab.waitForExistence(timeout: 3.0))
        addScreenshot(of: app, named: "iPhone 17 Slices")

        volumeTab.tap()
        Thread.sleep(forTimeInterval: 1.0)
        addScreenshot(of: app, named: "iPhone 17 3D Volume")

        let slicesTab = app.buttons["Slices"]
        XCTAssertTrue(slicesTab.waitForExistence(timeout: 3.0))
        slicesTab.tap()
        Thread.sleep(forTimeInterval: 2.0)
        app.swipeUp()
        app.swipeUp()
        XCTAssertTrue(app.staticTexts["Field Selection"].waitForExistence(timeout: 3.0))
        XCTAssertTrue(app.staticTexts["Prescribed Field Strip Chart"].waitForExistence(timeout: 3.0))
        addScreenshot(of: app, named: "iPhone 17 Fields and Chart")
    }

    @MainActor
    func testProvenanceSheetPresentationAndDismiss() throws {
        let app = XCUIApplication()
        app.launch()

        let infoButton = app.buttons["Open Provenance and Limitations"]
        XCTAssertTrue(infoButton.waitForExistence(timeout: 3.0))
        infoButton.tap()

        let sheetTitle = app.staticTexts["Educational Scenario Explorer"]
        XCTAssertTrue(sheetTitle.waitForExistence(timeout: 3.0))
        addScreenshot(of: app, named: "iPhone 17 Provenance")

        let doneButton = app.buttons["Done"]
        XCTAssertTrue(doneButton.waitForExistence(timeout: 3.0))
        doneButton.tap()
    }

    @MainActor
    func testTransportPlayPause() throws {
        let app = XCUIApplication()
        app.launch()

        let playButton = app.buttons["Start cycle playback"]
        XCTAssertTrue(playButton.waitForExistence(timeout: 3.0))
        playButton.tap()

        let pauseButton = app.buttons["Pause cycle playback"]
        XCTAssertTrue(pauseButton.waitForExistence(timeout: 3.0))
        pauseButton.tap()
    }

    @MainActor
    func testOnRenderDisclosuresAndPrivacyPolicySection() throws {
        let app = XCUIApplication()
        app.launch()

        XCTAssertTrue(app.staticTexts["Revision-0 prescribed fields; not a physical simulation."].waitForExistence(timeout: 3.0))
        XCTAssertTrue(app.staticTexts["Not a thermal-hydraulic solution. Must not be used to state temperatures at Oklo."].waitForExistence(timeout: 3.0))

        let infoButton = app.buttons["Open Provenance and Limitations"]
        XCTAssertTrue(infoButton.waitForExistence(timeout: 3.0))
        infoButton.tap()

        XCTAssertTrue(app.staticTexts["Field-by-Field Limitations"].waitForExistence(timeout: 3.0))
        app.swipeUp()
        app.swipeUp()
        XCTAssertTrue(app.staticTexts["Privacy Policy & Data Handling"].waitForExistence(timeout: 3.0))
        XCTAssertTrue(app.staticTexts["URL remains unverified until the account owner confirms live hosting."].waitForExistence(timeout: 3.0))
    }

    @MainActor
    private func addScreenshot(of app: XCUIApplication, named name: String) {
        let attachment = XCTAttachment(screenshot: app.screenshot())
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
