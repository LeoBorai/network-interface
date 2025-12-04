//! Network Interface abstraction from commonly used fields for nodes from the
//! linked list provided by system functions like `getifaddrs` and
//! `GetAdaptersAddresses`.
use std::fmt::Debug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// An alias for an `Option` that wraps either a `Ipv4Addr` or a `Ipv6Addr`
/// representing the IP for a Network Interface netmask
pub type Netmask<T> = Option<T>;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Status {
    /// Could not retrieve status
    Unknown,
    /// device is up
    Up,
    /// deivce down, by default you can't get device markd `down`
    Down,
    // Status is not `up` or `down`
    Unavailable,
}

/// Filter Running deivces, on Windows, it has no effect
#[cfg(not(target_os = "windows"))]
pub const IFF_RUNNING: i32 = libc::IFF_RUNNING;
/// Filter Ethernet interfaces, on *unix is can be `IFF_UP | IFF_BROADCAST | IFF_MULTICAST`, on windows adapter type is used
#[cfg(not(target_os = "windows"))]
pub const IFF_ETH: i32 = libc::IFF_UP | libc::IFF_BROADCAST | libc::IFF_MULTICAST;
/// Filter Wireless interfaces sometimes it sames as Eth, on windows adapter `ifType` is used.
#[cfg(not(target_os = "windows"))]
pub const IFF_WIRELESS: i32 = libc::IFF_UP | libc::IFF_BROADCAST | libc::IFF_MULTICAST;
///Filter out TUN interfaces. Note! This is only a hypothesis. on windows adapter `ifType` is used.
#[cfg(not(target_os = "windows"))]
pub const IFF_TUN: i32 = libc::IFF_UP | libc::IFF_POINTOPOINT;
///Filter out LOOPBACK interfaces,on windows adapter `ifType` is used.
#[cfg(not(target_os = "windows"))]
pub const IFF_LOOPBACK: i32 = libc::IFF_UP | libc::IFF_LOOPBACK;

/// Filter Running deivces, on Windows, it has no effect
#[cfg(target_os = "windows")]
pub const IFF_RUNNING: i32 = 0x40;
/// Filter Ethernet interfaces, on *unix is can be `IFF_UP | IFF_BROADCAST | IFF_MULTICAST`, on windows adapter `ifType` is used
#[cfg(target_os = "windows")]
pub const IFF_ETH: i32 = 0x1;
/// Filter Wireless interfaces sometimes it sames as Eth, on windows adapter `ifType` is used.
#[cfg(target_os = "windows")]
pub const IFF_WIRELESS: i32 = 0x2;
///Filter out TUN interfaces. Note! This is only a hypothesis. on windows adapter `ifType` is used.
#[cfg(target_os = "windows")]
pub const IFF_TUN: i32 = 0x1 | 0x4;
///Filter out LOOPBACK interfaces,on windows adapter `ifType` is used.
#[cfg(target_os = "windows")]
pub const IFF_LOOPBACK: i32 = 0x8;

// Note: libc::MASTER might not be available, you may need to define it manually
// pub const IFF_BRIDGE: i32 = libc::IFF_UP | libc::MASTER;
/// A system's network interface
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct NetworkInterface {
    /// Interface's name
    pub name: String,
    /// Interface's address
    pub addr: Vec<Addr>,
    /// MAC Address
    pub mac_addr: Option<String>,
    /// Interface's index
    pub index: u32,
    /// Interface's status
    pub status: Status,
    /// Interface's flags
    pub(crate) flags: i32,
}

/// Network interface address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Addr {
    /// IPV4 Interface from the AFINET network interface family
    V4(V4IfAddr),
    /// IPV6 Interface from the AFINET6 network interface family
    V6(V6IfAddr),
}

/// IPV4 Interface from the AFINET network interface family
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct V4IfAddr {
    /// The IP address for this network interface
    pub ip: Ipv4Addr,
    /// The broadcast address for this interface
    pub broadcast: Option<Ipv4Addr>,
    /// The netmask for this interface
    pub netmask: Netmask<Ipv4Addr>,
}

/// IPV6 Interface from the AFINET6 network interface family
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct V6IfAddr {
    /// The IP address for this network interface
    pub ip: Ipv6Addr,
    /// The broadcast address for this interface
    pub broadcast: Option<Ipv6Addr>,
    /// The netmask for this interface
    pub netmask: Netmask<Ipv6Addr>,
}

impl NetworkInterface {
    pub fn new_afinet(
        name: &str,
        addr: Ipv4Addr,
        netmask: Netmask<Ipv4Addr>,
        broadcast: Option<Ipv4Addr>,
        index: u32,
        status: Status,
        flags: i32,
    ) -> NetworkInterface {
        let ifaddr_v4 = V4IfAddr {
            ip: addr,
            broadcast,
            netmask,
        };

        NetworkInterface {
            name: name.to_string(),
            addr: vec![Addr::V4(ifaddr_v4)],
            mac_addr: None,
            index,
            status,
            flags,
        }
    }

    pub fn new_afinet6(
        name: &str,
        addr: Ipv6Addr,
        netmask: Netmask<Ipv6Addr>,
        broadcast: Option<Ipv6Addr>,
        index: u32,
        status: Status,
        flags: i32,
    ) -> NetworkInterface {
        let ifaddr_v6 = V6IfAddr {
            ip: addr,
            broadcast,
            netmask,
        };

        NetworkInterface {
            name: name.to_string(),
            addr: vec![Addr::V6(ifaddr_v6)],
            mac_addr: None,
            index,
            status,
            flags,
        }
    }

    pub fn with_mac_addr(self, mac_addr: Option<String>) -> Self {
        Self { mac_addr, ..self }
    }

    pub fn is_up(&self) -> bool {
        self.status == Status::Up
    }
}

impl Addr {
    pub fn ip(self) -> IpAddr {
        match self {
            Addr::V4(ifaddr_v4) => ifaddr_v4.ip.into(),
            Addr::V6(ifaddr_v6) => ifaddr_v6.ip.into(),
        }
    }

    pub fn broadcast(self) -> Option<IpAddr> {
        match self {
            Addr::V4(ifaddr_v4) => ifaddr_v4.broadcast.map(Into::into),
            Addr::V6(ifaddr_v6) => ifaddr_v6.broadcast.map(Into::into),
        }
    }

    pub fn netmask(self) -> Netmask<IpAddr> {
        match self {
            Addr::V4(ifaddr_v4) => ifaddr_v4.netmask.map(Into::into),
            Addr::V6(ifaddr_v6) => ifaddr_v6.netmask.map(Into::into),
        }
    }
}
