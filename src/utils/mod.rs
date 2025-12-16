#[cfg(windows)]
pub mod hex;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "ios",
    target_os = "macos",
    target_os = "tvos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "illumos",
))]
mod unix;

#[cfg(windows)]
pub(crate) mod ffialloc;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "ios",
    target_os = "macos",
    target_os = "tvos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "illumos",
))]
pub use unix::*;
