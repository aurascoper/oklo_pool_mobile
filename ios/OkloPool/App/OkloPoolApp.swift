import SwiftUI

@main
struct OkloPoolApp: App {
    @Environment(\.scenePhase) private var scenePhase
    @State private var model = OkloAppModel()

    var body: some Scene {
        WindowGroup {
            ContentView(model: model)
                .onChange(of: scenePhase) { oldPhase, newPhase in
                    switch newPhase {
                    case .background, .inactive:
                        // Automatically pause active playback to prevent background processing
                        if model.isPlaying {
                            model.pause()
                        }
                    case .active:
                        // Restore active state without modifying configuration
                        break
                    @unknown default:
                        break
                    }
                }
        }
    }
}
