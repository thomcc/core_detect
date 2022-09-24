//! This crate provides a `no_std` version of std's [`is_x86_feature_detected!`]
//! macro.
//!
//! This is possible because x86 chips can just use the `cpuid` instruction to
//! detect CPU features, whereas most other architectures require either reading
//! files or querying the OS.
//!
//! # Usage
//!
//! ```
//! # #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//! # fn main() {
//! if core_detect::is_x86_feature_detected!("ssse3") {
//!     println!("SSSE3 is available");
//! }
//! # }
//! # #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
//! # fn main() {}
//! ```
//!
//! Note that like the [equivalent macro in `std`][stddetect], this will error
//! on architectures other than x86/x86_64, so you should put the code behind a
//! `#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]` check.
//!
//! [stddetect]:
//! https://doc.rust-lang.org/nightly/std/macro.is_x86_feature_detected.html
//!
//! (In the future, this crate may provide another macro which returns false in
//! these cases instead, and supports testing multiple features simultaneously).
#![no_std]
#![allow(dead_code)]

#[macro_use]
mod macros;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "arch/x86.rs"]
#[macro_use]
mod arch;

// Unimplemented architecture:
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
mod arch {
    #[doc(hidden)]
    pub(crate) enum Feature {
        Null,
    }

    #[doc(hidden)]
    pub mod __is_feature_detected {}

    impl Feature {
        #[doc(hidden)]
        pub(crate) fn from_str(_s: &str) -> Result<Feature, ()> {
            Err(())
        }
        #[doc(hidden)]
        pub(crate) fn to_str(self) -> &'static str {
            ""
        }
    }
}

pub(crate) use crate::arch::Feature;
#[doc(hidden)]
pub use crate::arch::__is_feature_detected;

/// Performs run-time feature detection.
#[inline]
#[allow(dead_code)]
fn check_for(x: Feature) -> bool {
    cache::test(x as u32)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "os/x86.rs"]
mod os;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
mod os {
    #[inline]
    pub(crate) fn detect_features() -> crate::cache::Initializer {
        Default::default()
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_export]
macro_rules! is_x86_feature_detected {
    ($t: tt) => {
        compile_error!(
            r#"
        is_x86_feature_detected can only be used on x86 and x86_64 targets.
        You can prevent it from being used in other architectures by
        guarding it behind a cfg(target_arch) as follows:

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                if is_x86_feature_detected(...) { ... }
            }
        "#
        )
    };
}

mod cache;
