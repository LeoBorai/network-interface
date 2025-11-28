use network_interface::{NetworkInterface, NetworkInterfaceConfig,IFF_TUN,IFF_RUNNING};

fn main() {
    let interfaces = NetworkInterface::filter(NetworkInterface::show().unwrap(), IFF_TUN | IFF_RUNNING);
    println!("{interfaces:#?}");
}
