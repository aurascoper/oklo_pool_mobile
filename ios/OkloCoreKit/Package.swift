// swift-tools-version:5.9
// Build OkloCoreFFI.xcframework first: scripts/build_xcframework.sh
import PackageDescription

let package = Package(
    name: "OkloCoreKit",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(name: "OkloCore", targets: ["OkloCore"]),
    ],
    targets: [
        .binaryTarget(name: "OkloCoreFFI", path: "OkloCoreFFI.xcframework"),
        .target(name: "OkloCore", dependencies: ["OkloCoreFFI"]),
        .testTarget(name: "OkloCoreTests", dependencies: ["OkloCore"]),
    ]
)
