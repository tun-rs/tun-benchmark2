use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::thread;
use tun_rs::{DeviceBuilder, SyncDevice};

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
fn main() {
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
        .build_sync()
        .unwrap();
    let device2 = DeviceBuilder::new()
        .name(iface2)
        .ipv4(ip2, 24, None)
        .mtu(1500)
        .build_sync()
        .unwrap();
    let device1 = Arc::new(device1);
    let device2 = Arc::new(device2);
    println!(
        "{:?},{:?}",
        device1.name().unwrap(),
        device2.name().unwrap()
    );

    {
        let device1 = device1.clone();
        let device2 = device2.clone();
        thread::spawn(move || {
            copy(device1, device2);
        });
    }
    thread::spawn(move || copy(device2, device1));

    let (tx, rx) = std::sync::mpsc::channel();

    let _handle = ctrlc2::set_handler(move || {
        tx.send(()).expect("Signal error.");
        true
    });
    _ = rx.recv();
}
fn copy(device1: Arc<SyncDevice>, device2: Arc<SyncDevice>) {
    let mut buf = [0; 65536];
    loop {
        let len = device1.recv(&mut buf).unwrap();
        device2.send(&buf[..len]).unwrap();
    }
}
