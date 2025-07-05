import Foundation
import Metal
import MetalKit
import CoreGraphics
import ImageIO
import CoreImage
import UniformTypeIdentifiers // 👈 for UTType.png

let width = 512
let height = 512

let device = MTLCreateSystemDefaultDevice()!
let commandQueue = device.makeCommandQueue()!

// 🔧 Use the non-deprecated newLibrary(URL:)
let metallibURL = URL(fileURLWithPath: "randomart_spiderman_2.metallib")
let library = try! device.makeLibrary(URL: metallibURL)
let function = library.makeFunction(name: "art_gen")!
let pipeline = try! device.makeComputePipelineState(function: function)

let textureDescriptor = MTLTextureDescriptor.texture2DDescriptor(
    pixelFormat: .rgba32Float,
    width: width,
    height: height,
    mipmapped: false
)
textureDescriptor.usage = [.shaderWrite, .shaderRead]
let outputTexture = device.makeTexture(descriptor: textureDescriptor)!

let commandBuffer = commandQueue.makeCommandBuffer()!
let encoder = commandBuffer.makeComputeCommandEncoder()! // not optional

encoder.setComputePipelineState(pipeline)
encoder.setTexture(outputTexture, index: 0)

let w = pipeline.threadExecutionWidth
let h = pipeline.maxTotalThreadsPerThreadgroup / w
let threadsPerThreadgroup = MTLSizeMake(w, h, 1)
let threadgroups = MTLSizeMake((width + w - 1) / w, (height + h - 1) / h, 1)

let start = CFAbsoluteTimeGetCurrent()
encoder.dispatchThreadgroups(threadgroups, threadsPerThreadgroup: threadsPerThreadgroup)
encoder.endEncoding()
commandBuffer.commit()
commandBuffer.waitUntilCompleted()
let end = CFAbsoluteTimeGetCurrent()

print("⏱ GPU kernel execution took \(String(format: "%.3f", (end - start) * 1000)) ms")

// 🧠 Read back texture
let byteCount = width * height * 4 * MemoryLayout<Float>.size
let raw = UnsafeMutableRawPointer.allocate(byteCount: byteCount, alignment: 0x1000)
defer { raw.deallocate() }

let region = MTLRegionMake2D(0, 0, width, height)
outputTexture.getBytes(raw, bytesPerRow: width * 4 * MemoryLayout<Float>.size, from: region, mipmapLevel: 0)

// 🎨 Convert to 8-bit RGBA
var rgba8 = [UInt8](repeating: 0, count: width * height * 4)
let floatPixels = raw.bindMemory(to: Float.self, capacity: width * height * 4)
for i in 0..<width * height {
    for c in 0..<4 {
        let val = floatPixels[i * 4 + c]
        let scaled = val * 255
        if scaled.isFinite {
            rgba8[i * 4 + c] = UInt8(clamping: Int(scaled))
        } else {
            rgba8[i * 4 + c] = 0
        }
    }
}

// 🖼 Save as PNG
let colorSpace = CGColorSpaceCreateDeviceRGB()
var data = rgba8 // we need a mutable copy
let ctx = CGContext(data: &data,
                    width: width,
                    height: height,
                    bitsPerComponent: 8,
                    bytesPerRow: width * 4,
                    space: colorSpace,
                    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!

let image = ctx.makeImage()!
let url = URL(fileURLWithPath: "out.png")

// ✅ Use modern UTType.png
let dest = CGImageDestinationCreateWithURL(url as CFURL, UTType.png.identifier as CFString, 1, nil)!
CGImageDestinationAddImage(dest, image, nil)
CGImageDestinationFinalize(dest)

print("✅ Image saved to out.png")