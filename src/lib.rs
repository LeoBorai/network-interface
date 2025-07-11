mod error;
mod interface;
mod target;
mod test;

mod utils;

pub use error::*;
pub use interface::*;
pub use mac_addr::*;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait NetworkInterfaceConfig {
    /// List system's network interfaces configuration
    fn show() -> Result<Vec<interface::NetworkInterface>>;
}
