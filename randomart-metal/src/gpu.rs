use std::path::Path;
use std::process::Command;
use image::{ImageBuffer, RgbaImage};
use std::ptr::NonNull;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSError, NSString, NSURL};
use objc2_metal::{
    MTLCommandBuffer,
    MTLCommandEncoder,
    MTLCommandQueue,
    MTLComputeCommandEncoder,
    MTLComputePipelineState,
    MTLCreateSystemDefaultDevice,
    MTLDevice,
    MTLLibrary,
    MTLOrigin,
    MTLPixelFormat,
    MTLRegion,
    MTLSize,
    MTLTexture,
    MTLTextureDescriptor,
    MTLTextureUsage,
};

/// Compile `src_path` (MSL source) to a `.metallib` at `lib_path` using xcrun.
pub fn compile_metal(src_path: &Path, lib_path: &Path) -> Result<(), String> {
    let status = Command::new("xcrun")
        .args([
            "-sdk", "macosx", "metal",
            src_path.to_str().unwrap(),
            "-o", lib_path.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| format!("failed to run xcrun: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("xcrun metal exited with {status}"))
    }
}

fn ns_error_msg(err: &Retained<NSError>) -> String {
    unsafe { err.localizedDescription().to_string() }
}

/// Load a `.metallib`, dispatch the `art_gen` kernel over a `width x height`
/// rgba32Float texture, read back the pixels, and return an RGBA image.
pub fn run_gpu_kernel(metallib_path: &Path, width: u32, height: u32) -> RgbaImage {
    unsafe {
        let device = MTLCreateSystemDefaultDevice().expect("no Metal device");

        let queue: Retained<ProtocolObject<dyn MTLCommandQueue>> = device
            .newCommandQueue()
            .expect("newCommandQueue failed");

        // Load library from file URL.
        let path_str = NSString::from_str(metallib_path.to_str().unwrap());
        let url = NSURL::fileURLWithPath(&path_str);
        let library: Retained<ProtocolObject<dyn MTLLibrary>> = device
            .newLibraryWithURL_error(&url)
            .unwrap_or_else(|e| panic!("newLibraryWithURL failed: {}", ns_error_msg(&e)));

        let fn_name = NSString::from_str("art_gen");
        let metal_fn = library
            .newFunctionWithName(&fn_name)
            .expect("function 'art_gen' not found in metallib");

        let pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>> = device
            .newComputePipelineStateWithFunction_error(&metal_fn)
            .unwrap_or_else(|e| panic!("newComputePipelineState failed: {}", ns_error_msg(&e)));

        // Create rgba32Float texture.
        let desc = MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            MTLPixelFormat::RGBA32Float,
            width as usize,
            height as usize,
            false,
        );
        desc.setUsage(MTLTextureUsage::ShaderWrite | MTLTextureUsage::ShaderRead);
        let texture: Retained<ProtocolObject<dyn MTLTexture>> = device
            .newTextureWithDescriptor(&desc)
            .expect("newTexture failed");

        // Encode and dispatch.
        let cmd_buf: Retained<ProtocolObject<dyn MTLCommandBuffer>> = queue
            .commandBuffer()
            .expect("commandBuffer failed");

        let encoder: Retained<ProtocolObject<dyn MTLComputeCommandEncoder>> = cmd_buf
            .computeCommandEncoder()
            .expect("computeCommandEncoder failed");

        encoder.setComputePipelineState(&pipeline);
        encoder.setTexture_atIndex(Some(&texture), 0);

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

        // Readback: each pixel is 4 x f32 = 16 bytes.
        let bytes_per_row = width as usize * 4 * std::mem::size_of::<f32>();
        let byte_count = height as usize * bytes_per_row;
        let mut raw = vec![0u8; byte_count];

        let region = MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size:   MTLSize { width: width as usize, height: height as usize, depth: 1 },
        };
        let ptr = NonNull::new(raw.as_mut_ptr() as *mut std::ffi::c_void).unwrap();
        texture.getBytes_bytesPerRow_fromRegion_mipmapLevel(ptr, bytes_per_row, region, 0);

        // Convert RGBA32F → RGBA8.
        // The Metal kernel already maps values into [0, 1] before writing.
        let float_pixels: &[f32] = std::slice::from_raw_parts(
            raw.as_ptr() as *const f32,
            width as usize * height as usize * 4,
        );
        let rgba8: Vec<u8> = float_pixels.iter().map(|&v| {
            let scaled = v * 255.0;
            if scaled.is_finite() { (scaled as u32).min(255) as u8 } else { 0 }
        }).collect();

        ImageBuffer::from_raw(width, height, rgba8).expect("ImageBuffer::from_raw failed")
    }
}
