use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tun_rs::{AsyncDevice, DeviceBuilder};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// First interface name (e.g., tun11)
    #[arg(long)]
    iface1: String,
    /// IP address for first interface (e.g., 10.0.1.1)
    #[arg(long)]
    ip1: Ipv4Addr,
    /// Second interface name (e.g., tun22)
    #[arg(long)]
    iface2: String,

    /// IP address for second interface (e.g., 10.0.2.1)
    #[arg(long)]
    ip2: Ipv4Addr,
}

#[tokio::main]
async fn main() {
    let Args {
        iface1,
        ip1,
        iface2,
        ip2,
    } = Args::parse();

    let device1 = DeviceBuilder::new()
        .name(iface1)
        .ipv4(ip1, 24, None)
        .mtu(1500)
        .build_async()
        .unwrap();
    let device2 = DeviceBuilder::new()
        .name(iface2)
        .ipv4(ip2, 24, None)
        .mtu(1500)
        .build_async()
        .unwrap();
    let device1 = Arc::new(device1);
    let device2 = Arc::new(device2);
    println!(
        "{:?},{:?}",
        device1.name().unwrap(),
        device2.name().unwrap()
    );
    let handle1 = tokio::spawn(copy(device1.clone(), device2.clone()));
    let handle2 = tokio::spawn(copy(device2, device1));
    tokio::try_join!(handle1, handle2).unwrap();
}
async fn copy(device1: Arc<AsyncDevice>, device2: Arc<AsyncDevice>) {
    let mut buf = [0; 65536];
    loop {
        let len = device1.recv(&mut buf).await.unwrap();
        device2.send(&buf[..len]).await.unwrap();
    }
}
