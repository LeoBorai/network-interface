mod error;
mod interface;
mod target;
mod test;

mod utils;

pub use error::*;
pub use interface::*;

pub type Result<T> = std::result::Result<T, error::Error>;
pub trait NetworkInterfaceConfig {
    /// List system's network interfaces configuration
    fn show() -> Result<Vec<interface::NetworkInterface>>;

    /// Filter network interfaces by flags
    /// # Arguments
    /// * `ifas` - Network interfaces to filter
    /// * `flags` - Flags to filter
    /// # Example
    /// ```
    /// use network_interface::{NetworkInterface, IFF_ETH | IFF_RUNNING};
    /// let ifas = NetworkInterface::show().unwrap();
    /// let ifas = NetworkInterface::filter(ifas, IFF_VPN | IFF_RUNNING);
    /// ```
    fn filter(
        ifas: Vec<interface::NetworkInterface>,
        flags: i32,
    ) -> Vec<interface::NetworkInterface>;
}
