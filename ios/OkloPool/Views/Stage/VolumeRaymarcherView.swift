import SwiftUI
import MetalKit
import simd

struct RaymarcherUniforms {
    var camPos: SIMD3<Float>
    var aspect: Float
    var camRight: SIMD3<Float>
    var opacity: Float
    var camUp: SIMD3<Float>
    var invertA: Int32
    var camFwd: SIMD3<Float>
    var categorical: Int32
}

public struct VolumeRaymarcherView: View {
    @Bindable var model: OkloAppModel
    @State private var yaw: Float = -0.85
    @State private var pitch: Float = 0.42
    @State private var dist: Float = 2.7

    public init(model: OkloAppModel) {
        self.model = model
    }

    public var body: some View {
        VStack(spacing: 8) {
            ZStack(alignment: .bottomLeading) {
                MetalVolumeRepresentable(
                    model: model,
                    yaw: yaw,
                    pitch: pitch,
                    dist: dist
                )
                .frame(maxWidth: .infinity)
                .aspectRatio(1, contentMode: .fit)
                .background(Color.black.opacity(0.9))
                .cornerRadius(8)
                .gesture(
                    DragGesture()
                        .onChanged { value in
                            let deltaX = Float(value.translation.width) * 0.005
                            let deltaY = Float(value.translation.height) * 0.005
                            yaw -= deltaX
                            pitch = max(-1.4, min(1.4, pitch + deltaY))
                        }
                )
                .gesture(
                    MagnificationGesture()
                        .onChanged { scale in
                            let s = Float(scale)
                            dist = max(1.5, min(6.0, dist / s))
                        }
                )

                // Hint overlay
                HStack {
                    Text("Drag to rotate, pinch to zoom. k points up.")
                        .font(.caption2)
                        .foregroundStyle(.white.opacity(0.75))
                    Spacer()
                    Button(action: {
                        yaw = -0.85
                        pitch = 0.42
                        dist = 2.7
                    }) {
                        Label("Reset View", systemImage: "arrow.counterclockwise")
                            .font(.caption2)
                    }
                    .buttonStyle(.bordered)
                    .tint(.white)
                }
                .padding(8)
            }

            // Opacity slider
            HStack(spacing: 12) {
                Text("Opacity:")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Slider(
                    value: $model.volumeOpacity,
                    in: 0.2...3.0,
                    step: 0.1
                )
                .accessibilityLabel("Volume opacity slider")
                .accessibilityValue(String(format: "%.1f", model.volumeOpacity))

                Text(String(format: "%.1f", model.volumeOpacity))
                    .font(.caption)
                    .fontDesign(.monospaced)
                    .foregroundStyle(.secondary)
                    .frame(width: 32, alignment: .trailing)
            }
            .padding(.horizontal, 4)
        }
    }
}

struct MetalVolumeRepresentable: UIViewRepresentable {
    let model: OkloAppModel
    let yaw: Float
    let pitch: Float
    let dist: Float

    func makeUIView(context: Context) -> MTKView {
        let view = MTKView()
        guard let device = MTLCreateSystemDefaultDevice() else {
            return view
        }
        view.device = device
        view.delegate = context.coordinator
        view.isOpaque = false
        view.clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 0)
        view.enableSetNeedsDisplay = false
        view.isPaused = false
        return view
    }

    func updateUIView(_ uiView: MTKView, context: Context) {
        context.coordinator.update(
            model: model,
            yaw: yaw,
            pitch: pitch,
            dist: dist
        )
    }

    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }

    final class Coordinator: NSObject, MTKViewDelegate {
        var parent: MetalVolumeRepresentable
        var device: MTLDevice?
        var commandQueue: MTLCommandQueue?
        var pipelineState: MTLRenderPipelineState?
        var volTexture: MTLTexture?
        var cmapTexture: MTLTexture?
        var linearSampler: MTLSamplerState?
        var nearestSampler: MTLSamplerState?

        var currentYaw: Float = -0.85
        var currentPitch: Float = 0.42
        var currentDist: Float = 2.7
        var currentOpacity: Float = 1.0
        var currentInvertA: Int32 = 0
        var currentCategorical: Int32 = 0

        init(_ parent: MetalVolumeRepresentable) {
            self.parent = parent
            super.init()
            setupMetal()
        }

        private func setupMetal() {
            guard let dev = MTLCreateSystemDefaultDevice() else { return }
            device = dev
            commandQueue = dev.makeCommandQueue()

            let library = dev.makeDefaultLibrary()
            let vertexFn = library?.makeFunction(name: "volumeVertexShader")
            let fragmentFn = library?.makeFunction(name: "volumeFragmentShader")

            let desc = MTLRenderPipelineDescriptor()
            desc.vertexFunction = vertexFn
            desc.fragmentFunction = fragmentFn
            desc.colorAttachments[0].pixelFormat = .bgra8Unorm
            desc.colorAttachments[0].isBlendingEnabled = true
            desc.colorAttachments[0].rgbBlendOperation = .add
            desc.colorAttachments[0].alphaBlendOperation = .add
            desc.colorAttachments[0].sourceRGBBlendFactor = .sourceAlpha
            desc.colorAttachments[0].sourceAlphaBlendFactor = .one
            desc.colorAttachments[0].destinationRGBBlendFactor = .oneMinusSourceAlpha
            desc.colorAttachments[0].destinationAlphaBlendFactor = .oneMinusSourceAlpha

            pipelineState = try? dev.makeRenderPipelineState(descriptor: desc)

            let samplerDesc = MTLSamplerDescriptor()
            samplerDesc.minFilter = .linear
            samplerDesc.magFilter = .linear
            samplerDesc.sAddressMode = .clampToEdge
            samplerDesc.tAddressMode = .clampToEdge
            samplerDesc.rAddressMode = .clampToEdge
            linearSampler = dev.makeSamplerState(descriptor: samplerDesc)

            samplerDesc.minFilter = .nearest
            samplerDesc.magFilter = .nearest
            nearestSampler = dev.makeSamplerState(descriptor: samplerDesc)
        }

        @MainActor
        func update(model: OkloAppModel, yaw: Float, pitch: Float, dist: Float) {
            self.currentYaw = yaw
            self.currentPitch = pitch
            self.currentDist = dist
            self.currentOpacity = model.volumeOpacity
            self.currentInvertA = model.currentFieldMetadata.isInvertedIn3D ? 1 : 0
            self.currentCategorical = (model.selectedField == .compartments) ? 1 : 0

            guard let dev = device else { return }

            // Upload 3D Volume Texture (32 x 32 x 32, R8Unorm)
            let dim = 32
            let totalVoxels = dim * dim * dim
            if model.currentFieldData.count == totalVoxels {
                var bytes = [UInt8](repeating: 0, count: totalVoxels)
                let range = model.currentFieldRange
                let denom = range.max - range.min

                if model.selectedField == .compartments {
                    for i in 0..<totalVoxels {
                        bytes[i] = UInt8(max(0, min(255, Int(round(model.currentFieldData[i])))))
                    }
                } else {
                    let scale = denom > 1e-6 ? 255.0 / denom : 0.0
                    for i in 0..<totalVoxels {
                        let norm = max(0.0, min(255.0, (model.currentFieldData[i] - range.min) * scale))
                        bytes[i] = UInt8(round(norm))
                    }
                }

                if volTexture == nil {
                    let texDesc = MTLTextureDescriptor()
                    texDesc.textureType = .type3D
                    texDesc.pixelFormat = .r8Unorm
                    texDesc.width = dim
                    texDesc.height = dim
                    texDesc.depth = dim
                    texDesc.usage = [.shaderRead]
                    volTexture = dev.makeTexture(descriptor: texDesc)
                }

                bytes.withUnsafeBytes { rawPtr in
                    volTexture?.replace(
                        region: MTLRegionMake3D(0, 0, 0, dim, dim, dim),
                        mipmapLevel: 0,
                        slice: 0,
                        withBytes: rawPtr.baseAddress!,
                        bytesPerRow: dim,
                        bytesPerImage: dim * dim
                    )
                }
            }

            // Upload 2D Colormap LUT Texture (256 x 1, RGBA8Unorm)
            let lutBytes = ColorRamps.makeColormapLUT(field: model.selectedField)
            if cmapTexture == nil {
                let lutDesc = MTLTextureDescriptor.texture2DDescriptor(
                    pixelFormat: .rgba8Unorm,
                    width: 256,
                    height: 1,
                    mipmapped: false
                )
                lutDesc.usage = [.shaderRead]
                cmapTexture = dev.makeTexture(descriptor: lutDesc)
            }
            lutBytes.withUnsafeBytes { rawPtr in
                cmapTexture?.replace(
                    region: MTLRegionMake2D(0, 0, 256, 1),
                    mipmapLevel: 0,
                    withBytes: rawPtr.baseAddress!,
                    bytesPerRow: 256 * 4
                )
            }
        }

        func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {}

        func draw(in view: MTKView) {
            guard let drawable = view.currentDrawable,
                  let renderPassDesc = view.currentRenderPassDescriptor,
                  let pipeline = pipelineState,
                  let queue = commandQueue,
                  let vol = volTexture,
                  let cmap = cmapTexture else { return }

            guard let cmdBuf = queue.makeCommandBuffer(),
                  let encoder = cmdBuf.makeRenderCommandEncoder(descriptor: renderPassDesc) else { return }

            encoder.setRenderPipelineState(pipeline)

            let w = Float(max(1.0, view.drawableSize.width))
            let h = Float(max(1.0, view.drawableSize.height))
            let aspect = w / h

            // Camera math matching WebGL camera()
            let center = SIMD3<Float>(0.5, 0.5, 0.5)
            let cosP = cos(currentPitch)
            let pos = center + SIMD3<Float>(
                currentDist * cosP * cos(currentYaw),
                currentDist * cosP * sin(currentYaw),
                currentDist * sin(currentPitch)
            )

            let fwd = normalize(center - pos)
            let right = normalize(cross(fwd, SIMD3<Float>(0, 0, 1)))
            let up = cross(right, fwd)

            var uniforms = RaymarcherUniforms(
                camPos: pos,
                aspect: aspect,
                camRight: right,
                opacity: currentOpacity,
                camUp: up,
                invertA: currentInvertA,
                camFwd: fwd,
                categorical: currentCategorical
            )

            encoder.setFragmentBytes(&uniforms, length: MemoryLayout<RaymarcherUniforms>.size, index: 0)
            encoder.setFragmentTexture(vol, index: 0)
            encoder.setFragmentTexture(cmap, index: 1)
            let sampler = (currentCategorical == 1) ? nearestSampler : linearSampler
            encoder.setFragmentSamplerState(sampler, index: 0)
            encoder.setFragmentSamplerState(nearestSampler, index: 1)

            encoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4)
            encoder.endEncoding()

            cmdBuf.present(drawable)
            cmdBuf.commit()
        }
    }
}
