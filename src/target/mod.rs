#[allow(unused_imports)]
#[cfg(any(target_os = "android", target_os = "linux"))]
mod linux;

#[allow(unused_imports)]
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use linux::*;

#[allow(unused_imports)]
#[cfg(target_os = "hurd")]
mod hurd;

#[allow(unused_imports)]
#[cfg(target_os = "hurd")]
pub use hurd::*;

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "illumos",
))]
mod unix;

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "illumos",
))]
pub use unix::*;

#[allow(unused_imports)]
#[cfg(target_os = "windows")]
mod windows;

#[allow(unused_imports)]
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(target_os = "windows"))]
mod getifaddrs;

#[cfg(not(target_os = "windows"))]
pub use getifaddrs::*;
