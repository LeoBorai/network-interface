use network_interface::{NetworkInterface, NetworkInterfaceConfig, IFF_TEH, IFF_RUNNING};

fn main() {
    let interfaces =
        NetworkInterface::filter(NetworkInterface::show().unwrap(), IFF_TEH | IFF_RUNNING);
    println!("{interfaces:#?}");
}
