//! Verifies that `randomart_core::disable_ftz()` actually changes floating-point
//! behavior — i.e. the FTZ fix is not a silent no-op. The difference is only
//! observable when the subnormal is *runtime*-computed (so the compiler can't
//! constant-fold the operation and skip the FPU); `black_box` enforces that.

use std::hint::black_box;

#[cfg(target_arch = "aarch64")]
unsafe fn enable_ftz() {
    let mut fpcr: u64;
    std::arch::asm!("mrs {0}, fpcr", out(reg) fpcr, options(nostack));
    std::arch::asm!("msr fpcr, {0}", in(reg) fpcr | (1 << 24), options(nostack));
}

#[cfg(target_arch = "x86_64")]
unsafe fn enable_ftz() {
    let mut mxcsr: u32;
    std::arch::asm!("stmxcsr [{0}]", out(reg) mxcsr, options(nostack));
    std::arch::asm!("ldmxcsr [{0}]", in(reg) &(mxcsr | 0x8040), options(nostack));
}

/// Multiply the smallest positive subnormal by 1.0. With FTZ the input is
/// flushed to zero (result 0x0); without FTZ it passes through unchanged.
#[inline(never)]
fn subnormal_passthrough() -> f32 {
    let tiny = f32::from_bits(1); // smallest positive subnormal
    black_box(tiny) * black_box(1.0f32)
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[test]
fn disable_ftz_changes_subnormal_handling() {
    // FTZ enabled: subnormal flushed to zero.
    unsafe { enable_ftz() };
    let with_ftz = subnormal_passthrough();

    // FTZ disabled (the state the renderer sets on every worker thread).
    unsafe { randomart_core::disable_ftz() };
    let without_ftz = subnormal_passthrough();

    assert_eq!(
        with_ftz.to_bits(),
        0x0000_0000,
        "expected FTZ to flush the subnormal to zero"
    );
    assert_eq!(
        without_ftz.to_bits(),
        0x0000_0001,
        "expected disable_ftz() to preserve the subnormal"
    );
    assert_ne!(
        with_ftz.to_bits(),
        without_ftz.to_bits(),
        "disable_ftz() must actually change FP behavior"
    );
}
