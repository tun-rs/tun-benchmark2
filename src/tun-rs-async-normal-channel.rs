use bytes::BytesMut;
use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
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
    let (s1, r1) = tokio::sync::mpsc::channel(2048);
    let (s2, r2) = tokio::sync::mpsc::channel(2048);
    let handle1 = tokio::spawn(dev_to_channel(device1.clone(), s1));
    let handle2 = tokio::spawn(dev_to_channel(device2.clone(), s2));

    let handle3 = tokio::spawn(channel_to_dev(r1, device2));
    let handle4 = tokio::spawn(channel_to_dev(r2, device1));
    tokio::try_join!(handle1, handle2, handle3, handle4).unwrap();
}
async fn dev_to_channel(dev: Arc<AsyncDevice>, sender: Sender<BytesMut>) {
    let mut buf = [0; 65536];
    loop {
        let len = dev.recv(&mut buf).await.unwrap();
        sender.send(BytesMut::from(&buf[..len])).await.unwrap();
    }
}
async fn channel_to_dev(mut receiver: Receiver<BytesMut>, dev: Arc<AsyncDevice>) {
    loop {
        let bytes_mut = receiver.recv().await.unwrap();
        dev.send(&bytes_mut).await.unwrap();
    }
}
