import Foundation
import CoreGraphics
@preconcurrency import OkloCore

public struct RGBColor: Sendable, Equatable {
    public let r: UInt8
    public let g: UInt8
    public let b: UInt8

    public init(r: UInt8, g: UInt8, b: UInt8) {
        self.r = r
        self.g = g
        self.b = b
    }

    public static func fromHex(_ hex: String) -> RGBColor {
        var clean = hex.trimmingCharacters(in: .whitespacesAndNewlines)
        if clean.hasPrefix("#") {
            clean.removeFirst()
        }
        guard let val = UInt32(clean, radix: 16) else {
            return RGBColor(r: 0, g: 0, b: 0)
        }
        let r = UInt8((val >> 16) & 0xFF)
        let g = UInt8((val >> 8) & 0xFF)
        let b = UInt8(val & 0xFF)
        return RGBColor(r: r, g: g, b: b)
    }
}

public enum ColorRamps {
    public static let infernoStops: [RGBColor] = [
        RGBColor.fromHex("#000004"), RGBColor.fromHex("#1b0c41"), RGBColor.fromHex("#4a0c6b"),
        RGBColor.fromHex("#781c6d"), RGBColor.fromHex("#a52c60"), RGBColor.fromHex("#cf4446"),
        RGBColor.fromHex("#ed6925"), RGBColor.fromHex("#fb9b06"), RGBColor.fromHex("#f7d13d"),
        RGBColor.fromHex("#fcffa4")
    ]

    public static let bluesStops: [RGBColor] = [
        RGBColor.fromHex("#f7fbff"), RGBColor.fromHex("#deebf7"), RGBColor.fromHex("#c6dbef"),
        RGBColor.fromHex("#9ecae1"), RGBColor.fromHex("#6baed6"), RGBColor.fromHex("#4292c6"),
        RGBColor.fromHex("#2171b5"), RGBColor.fromHex("#08519c"), RGBColor.fromHex("#08306b")
    ]

    public static let viridisStops: [RGBColor] = [
        RGBColor.fromHex("#440154"), RGBColor.fromHex("#472d7b"), RGBColor.fromHex("#3b528b"),
        RGBColor.fromHex("#2c728e"), RGBColor.fromHex("#21918c"), RGBColor.fromHex("#28ae80"),
        RGBColor.fromHex("#5ec962"), RGBColor.fromHex("#addc30"), RGBColor.fromHex("#fde725")
    ]

    public static let cividisStops: [RGBColor] = [
        RGBColor.fromHex("#00224e"), RGBColor.fromHex("#123570"), RGBColor.fromHex("#3b496c"),
        RGBColor.fromHex("#575d6d"), RGBColor.fromHex("#707173"), RGBColor.fromHex("#8a8678"),
        RGBColor.fromHex("#a59c74"), RGBColor.fromHex("#c3b369"), RGBColor.fromHex("#e1cc55"),
        RGBColor.fromHex("#fee838")
    ]

    public static let magmaStops: [RGBColor] = [
        RGBColor.fromHex("#000004"), RGBColor.fromHex("#180f3d"), RGBColor.fromHex("#440f76"),
        RGBColor.fromHex("#721f81"), RGBColor.fromHex("#9e2f7f"), RGBColor.fromHex("#cd4071"),
        RGBColor.fromHex("#f1605d"), RGBColor.fromHex("#fd9668"), RGBColor.fromHex("#feca8d"),
        RGBColor.fromHex("#fcfdbf")
    ]

    public static let zoneColors: [RGBColor] = [
        RGBColor.fromHex("#d8c79f"), // Zone 0 Sandstone
        RGBColor.fromHex("#6b2d82"), // Zone 1 Core
        RGBColor.fromHex("#8ea86c"), // Zone 2 Halo
        RGBColor.fromHex("#3c7fc0"), // Zone 3 Conduit
        RGBColor.fromHex("#f0eee8")  // Zone 4 Chimney
    ]

    public static let zoneAlphas: [UInt8] = [0, 230, 40, 170, 200]

    public static func sampleRamp(stops: [RGBColor], t: Float) -> RGBColor {
        let clamped = max(0.0, min(1.0, t))
        let scaled = clamped * Float(stops.count - 1)
        let idx = min(stops.count - 2, max(0, Int(scaled)))
        let frac = scaled - Float(idx)
        let c0 = stops[idx]
        let c1 = stops[idx + 1]

        let r = UInt8(round(Float(c0.r) + (Float(c1.r) - Float(c0.r)) * frac))
        let g = UInt8(round(Float(c0.g) + (Float(c1.g) - Float(c0.g)) * frac))
        let b = UInt8(round(Float(c0.b) + (Float(c1.b) - Float(c0.b)) * frac))
        return RGBColor(r: r, g: g, b: b)
    }

    public static func stops(for name: String) -> [RGBColor] {
        switch name {
        case "inferno": return infernoStops
        case "blues": return bluesStops
        case "viridis": return viridisStops
        case "cividis": return cividisStops
        case "magma": return magmaStops
        default: return infernoStops
        }
    }

    public static func color(for field: OkloSim.Field, value: Float, range: (min: Float, max: Float)) -> (r: UInt8, g: UInt8, b: UInt8, a: UInt8) {
        if field == .compartments {
            let zoneIdx = Int(round(value))
            if zoneIdx >= 0 && zoneIdx < zoneColors.count {
                let c = zoneColors[zoneIdx]
                return (c.r, c.g, c.b, 255)
            }
            return (0, 0, 0, 255)
        }

        let meta = FieldCatalog.metadata(for: field)
        let stops = stops(for: meta.colormapName)
        let denom = range.max - range.min
        let norm = denom > 1e-6 ? (value - range.min) / denom : 0.0
        let c = sampleRamp(stops: stops, t: norm)
        return (c.r, c.g, c.b, 255)
    }

    public static func createPlanSliceCGImage(
        data: [Float],
        k: Int,
        field: OkloSim.Field,
        range: (min: Float, max: Float),
        dim: Int = 32
    ) -> CGImage? {
        guard data.count == dim * dim * dim, k >= 0, k < dim else { return nil }

        var pixels = [UInt8](repeating: 0, count: dim * dim * 4)
        for r in 0..<dim {
            let j = dim - 1 - r
            for c in 0..<dim {
                let i = c
                let voxelIndex = k * dim * dim + j * dim + i
                let val = data[voxelIndex]
                let rgba = color(for: field, value: val, range: range)
                let pixelOffset = (r * dim + c) * 4
                pixels[pixelOffset + 0] = rgba.r
                pixels[pixelOffset + 1] = rgba.g
                pixels[pixelOffset + 2] = rgba.b
                pixels[pixelOffset + 3] = rgba.a
            }
        }

        return makeCGImage(pixels: pixels, width: dim, height: dim)
    }

    public static func createSectionSliceCGImage(
        data: [Float],
        j: Int,
        field: OkloSim.Field,
        range: (min: Float, max: Float),
        dim: Int = 32
    ) -> CGImage? {
        guard data.count == dim * dim * dim, j >= 0, j < dim else { return nil }

        var pixels = [UInt8](repeating: 0, count: dim * dim * 4)
        for r in 0..<dim {
            let k = dim - 1 - r
            for c in 0..<dim {
                let i = c
                let voxelIndex = k * dim * dim + j * dim + i
                let val = data[voxelIndex]
                let rgba = color(for: field, value: val, range: range)
                let pixelOffset = (r * dim + c) * 4
                pixels[pixelOffset + 0] = rgba.r
                pixels[pixelOffset + 1] = rgba.g
                pixels[pixelOffset + 2] = rgba.b
                pixels[pixelOffset + 3] = rgba.a
            }
        }

        return makeCGImage(pixels: pixels, width: dim, height: dim)
    }

    public static func makeColormapLUT(field: OkloSim.Field) -> [UInt8] {
        var lut = [UInt8](repeating: 0, count: 256 * 4)
        if field == .compartments {
            for x in 0..<256 {
                let offset = x * 4
                if x < zoneColors.count {
                    let c = zoneColors[x]
                    let a = zoneAlphas[x]
                    lut[offset + 0] = c.r
                    lut[offset + 1] = c.g
                    lut[offset + 2] = c.b
                    lut[offset + 3] = a
                } else {
                    lut[offset + 0] = 0
                    lut[offset + 1] = 0
                    lut[offset + 2] = 0
                    lut[offset + 3] = 0
                }
            }
        } else {
            let meta = FieldCatalog.metadata(for: field)
            let st = stops(for: meta.colormapName)
            for x in 0..<256 {
                let t = Float(x) / 255.0
                let c = sampleRamp(stops: st, t: t)
                let offset = x * 4
                lut[offset + 0] = c.r
                lut[offset + 1] = c.g
                lut[offset + 2] = c.b
                lut[offset + 3] = 255
            }
        }
        return lut
    }

    private static func makeCGImage(pixels: [UInt8], width: Int, height: Int) -> CGImage? {
        let colorSpace = CGColorSpaceCreateDeviceRGB()
        let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue)

        guard let provider = CGDataProvider(data: Data(pixels) as CFData) else {
            return nil
        }

        return CGImage(
            width: width,
            height: height,
            bitsPerComponent: 8,
            bitsPerPixel: 32,
            bytesPerRow: width * 4,
            space: colorSpace,
            bitmapInfo: bitmapInfo,
            provider: provider,
            decode: nil,
            shouldInterpolate: false,
            intent: .defaultIntent
        )
    }
}
