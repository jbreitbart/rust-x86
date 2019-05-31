#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#![allow(stable_features)]
#![feature(const_fn, asm, repr_transparent)]
#![no_std]
#![cfg_attr(test, allow(unused_features))]

#[cfg(target_arch = "x86")]
pub(crate) use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
pub(crate) use core::arch::x86_64 as arch;

macro_rules! bit {
    ($x:expr) => {
        1 << $x
    };
}

pub mod bits16;
pub mod bits32;
pub mod bits64;

pub mod controlregs;
pub mod dtables;
pub mod io;
pub mod irq;
pub mod msr;
pub mod random;
pub mod segmentation;
pub mod task;
pub mod time;
pub mod tlb;
pub mod xapic;

#[cfg(feature = "performance-counter")]
pub mod perfcnt;

/// A short-cut to the architecture (bits32 or bits64) this crate was compiled for.
pub mod current {
    #[cfg(target_arch = "x86")]
    pub use crate::bits32::*;
    #[cfg(target_arch = "x86_64")]
    pub use crate::bits64::*;
}

/// Support for the CPUID instructions.
pub mod cpuid {
    pub use raw_cpuid::*;
}

#[cfg(not(test))]
mod std {
    pub use core::fmt;
    pub use core::ops;
    pub use core::option;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
/// x86 Protection levels
///
/// # Note
/// This should not contain values larger than 2 bits, otherwise
/// segment descriptor code needs to be adjusted accordingly.
pub enum Ring {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11,
}

/// Stops instruction execution and places the processor in a HALT state.
///
/// An enabled interrupt (including NMI and SMI), a debug exception, the BINIT#
/// signal, the INIT# signal, or the RESET# signal will resume execution. If an
/// interrupt (including NMI) is used to resume execution after a HLT instruction,
/// the saved instruction pointer (CS:EIP) points to the instruction following
/// the HLT instruction.
///
/// # Unsafe
/// Will cause a general protection fault if used outside of ring 0.
#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt" :::: "volatile");
}

/// Read Processor ID
///
/// Reads the value of the IA32_TSC_AUX MSR (address C0000103H)
/// into the destination register.
///
/// # Unsafe
/// May fail with #UD if rdpid is not supported (check CPUID).
#[inline(always)]
pub unsafe fn rdpid() -> u64 {
    let mut pid: u64;
    asm!("rdpid $0" : "=r"(pid));
    return pid;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rdpid() {
        let rdpid_support = cpuid::CpuId::new()
            .get_extended_feature_info()
            .map_or(false, |finfo| finfo.has_rdpid());
        unsafe {
            if rdpid_support {
                let pid1 = rdpid();
                let pid2 = rdpid();
                // Let's hope we didn't migrate
                assert!(pid1 == pid2, "RDPID not consistent values?");
            }
        }
    }
}
