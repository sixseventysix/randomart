use randomart_core::pixel_buffer::PixelBuffer;
use anyhow::{anyhow, Context, Result};
use std::ptr::NonNull;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSError, NSString};
use objc2_metal::{
    MTLCommandBuffer,
    MTLCommandEncoder,
    MTLCommandQueue,
    MTLCompileOptions,
    MTLComputeCommandEncoder,
    MTLComputePipelineState,
    MTLCreateSystemDefaultDevice,
    MTLDevice,
    MTLLibrary,
    MTLMathMode,
    MTLOrigin,
    MTLPixelFormat,
    MTLRegion,
    MTLSize,
    MTLTexture,
    MTLTextureDescriptor,
    MTLTextureUsage,
};


fn ns_error_msg(err: &Retained<NSError>) -> String {
    err.localizedDescription().to_string()
}

/// JIT-compile MSL `source`, dispatch the `art_gen` kernel over a `width x height`
/// rgba32Float texture, read back the pixels, and return a RGB PixelBuffer.
pub fn run_gpu_kernel(source: &str, width: u32, height: u32) -> Result<PixelBuffer> {
    let device = MTLCreateSystemDefaultDevice().context("no Metal device available")?;

    let queue: Retained<ProtocolObject<dyn MTLCommandQueue>> = device
        .newCommandQueue()
        .context("failed to create Metal command queue")?;

    // JIT-compile MSL source with fast-math disabled.
    let options = MTLCompileOptions::new();
    options.setMathMode(MTLMathMode::Safe);
    let source_str = NSString::from_str(source);
    let library: Retained<ProtocolObject<dyn MTLLibrary>> = device
        .newLibraryWithSource_options_error(&source_str, Some(&options))
        .map_err(|e| anyhow!("Metal JIT compile failed: {}", ns_error_msg(&e)))?;

    let fn_name = NSString::from_str("art_gen");
    let metal_fn = library
        .newFunctionWithName(&fn_name)
        .context("function 'art_gen' not found in compiled Metal library")?;

    let pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>> = device
        .newComputePipelineStateWithFunction_error(&metal_fn)
        .map_err(|e| anyhow!("newComputePipelineState failed: {}", ns_error_msg(&e)))?;

    // Create rgba32Float texture.
    // SAFETY: the descriptor constructor is marked unsafe only because it takes
    // raw dimensions; RGBA32Float with non-zero width/height is a valid texture.
    let desc = unsafe {
        MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            MTLPixelFormat::RGBA32Float,
            width as usize,
            height as usize,
            false,
        )
    };
    desc.setUsage(MTLTextureUsage::ShaderWrite | MTLTextureUsage::ShaderRead);
    let texture: Retained<ProtocolObject<dyn MTLTexture>> = device
        .newTextureWithDescriptor(&desc)
        .context("failed to allocate output texture")?;

    // Encode and dispatch.
    let cmd_buf: Retained<ProtocolObject<dyn MTLCommandBuffer>> = queue
        .commandBuffer()
        .context("failed to create command buffer")?;

    let encoder: Retained<ProtocolObject<dyn MTLComputeCommandEncoder>> = cmd_buf
        .computeCommandEncoder()
        .context("failed to create compute command encoder")?;

    encoder.setComputePipelineState(&pipeline);
    // SAFETY: index 0 matches `out [[texture(0)]]` in the kernel, and `texture`
    // outlives the encoder / command buffer's execution below.
    unsafe { encoder.setTexture_atIndex(Some(&texture), 0) };

    let exec_width = pipeline.threadExecutionWidth();
    let max_threads = pipeline.maxTotalThreadsPerThreadgroup();
    let tg_width = exec_width;
    let tg_height = max_threads / exec_width;
    let threads_per_tg = MTLSize { width: tg_width, height: tg_height, depth: 1 };
    let threadgroups = MTLSize {
        width:  (width  as usize + tg_width  - 1) / tg_width,
        height: (height as usize + tg_height - 1) / tg_height,
        depth: 1,
    };

    encoder.dispatchThreadgroups_threadsPerThreadgroup(threadgroups, threads_per_tg);
    encoder.endEncoding();
    cmd_buf.commit();
    cmd_buf.waitUntilCompleted();

    // Readback: 4 floats (RGBA) per pixel. Read straight into a Vec<f32> so the
    // buffer is f32-aligned and no byte-to-float reinterpret is needed.
    let bytes_per_row = width as usize * 4 * std::mem::size_of::<f32>();
    let float_count = width as usize * height as usize * 4;
    let mut float_pixels = vec![0.0f32; float_count];

    let region = MTLRegion {
        origin: MTLOrigin { x: 0, y: 0, z: 0 },
        size:   MTLSize { width: width as usize, height: height as usize, depth: 1 },
    };
    let ptr = NonNull::new(float_pixels.as_mut_ptr() as *mut std::ffi::c_void).unwrap();
    // SAFETY: `float_pixels` is `float_count` f32s == height * bytes_per_row bytes,
    // exactly the region Metal writes back at `bytes_per_row` stride, so the copy
    // stays in bounds and fully initializes the buffer.
    unsafe { texture.getBytes_bytesPerRow_fromRegion_mipmapLevel(ptr, bytes_per_row, region, 0) };

    // Convert RGBA32F → RGB8.
    // The Metal kernel already maps values into [0, 1] before writing.
    let mut buf = PixelBuffer::new(width, height);
    for (i, chunk) in float_pixels.chunks_exact(4).enumerate() {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        let to_u8 = |v: f32| if (v * 255.0).is_finite() { ((v * 255.0) as u32).min(255) as u8 } else { 0 };
        buf.put_pixel(x, y, to_u8(chunk[0]), to_u8(chunk[1]), to_u8(chunk[2]));
    }
    Ok(buf)
}
