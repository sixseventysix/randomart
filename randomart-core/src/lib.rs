pub mod node;
pub mod grammar;
pub mod statistics;
pub mod rng;
pub mod pixel_buffer;
pub mod math;

/// Disable Flush-to-Zero (FTZ) and Denormals-Are-Zero (DAZ) in the MXCSR register.
/// This ensures subnormal floats are handled correctly (IEEE 754 compliant).
/// Must be called on each thread that performs floating-point computation.
pub unsafe fn disable_ftz() {
    #[cfg(target_arch = "x86_64")]
    {
        let mut mxcsr: u32;
        std::arch::asm!("stmxcsr [{0}]", out(reg) mxcsr, options(nostack));
        mxcsr &= !(1 << 15); // clear FTZ (bit 15)
        mxcsr &= !(1 << 6);  // clear DAZ (bit 6)
        std::arch::asm!("ldmxcsr [{0}]", in(reg) &mxcsr, options(nostack));
    }
    #[cfg(target_arch = "aarch64")]
    {
        let mut fpcr: u64;
        std::arch::asm!(
            "mrs {0}, fpcr",
            out(reg) fpcr,
            options(nostack)
        );
        fpcr &= !(1 << 24); // clear FZ (bit 24)
        std::arch::asm!(
            "msr fpcr, {0}",
            in(reg) fpcr,
            options(nostack)
        );
    }
}
