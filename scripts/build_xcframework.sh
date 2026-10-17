#!/bin/sh
# Build liboklo_core.a for iOS device, iOS simulator and macOS, and bundle the
# three with the C header into ios/OkloCoreKit/OkloCoreFFI.xcframework.
# Needs the three rustup targets already installed. Installs nothing.
set -eu
cd "$(dirname "$0")/.."

export IPHONEOS_DEPLOYMENT_TARGET=17.0
export MACOSX_DEPLOYMENT_TARGET=14.0
OUT=ios/OkloCoreKit/OkloCoreFFI.xcframework

for t in aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin; do
    cargo build --offline --release -p oklo_core --target "$t"
done

rm -rf "$OUT"
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/liboklo_core.a -headers crates/oklo_core/include \
    -library target/aarch64-apple-ios-sim/release/liboklo_core.a -headers crates/oklo_core/include \
    -library target/aarch64-apple-darwin/release/liboklo_core.a -headers crates/oklo_core/include \
    -output "$OUT"
