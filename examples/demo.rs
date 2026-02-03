use network_interface::{NetworkInterface, NetworkInterfaceConfig, IFF_ETH, IFF_RUNNING};

fn main() {
    let interfaces =
        NetworkInterface::filter(NetworkInterface::show().unwrap(), IFF_ETH | IFF_RUNNING);
    println!("{interfaces:#?}");
}
