use bytes::BytesMut;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tun_rs::async_framed::{BytesCodec, DeviceFramed};
use tun_rs::{AsyncDevice, DeviceBuilder, IDEAL_BATCH_SIZE};

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
    /// Worker thread num
    #[arg(long)]
    thread: Option<usize>,
}
fn main() {
    let args = Args::parse();
    println!("args={:?}", args);
    if let Some(thread) = args.thread {
        if thread == 1 {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    run().await;
                })
        } else {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(thread)
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    run().await;
                })
        }
    } else {
        main0();
    }
}

#[tokio::main]
async fn main0() {
    run().await;
}
async fn run() {
    let Args {
        iface1,
        ip1,
        iface2,
        ip2,
        ..
    } = Args::parse();

    let device1 = DeviceBuilder::new()
        .name(iface1)
        .ipv4(ip1, 24, None)
        .mtu(1500)
        .offload(true)
        .build_async()
        .unwrap();
    let device2 = DeviceBuilder::new()
        .name(iface2)
        .ipv4(ip2, 24, None)
        .mtu(1500)
        .offload(true)
        .build_async()
        .unwrap();
    let device1 = Arc::new(device1);
    let device2 = Arc::new(device2);
    println!(
        "{:?},{:?}",
        device1.name().unwrap(),
        device2.name().unwrap()
    );
    println!(
        "TCP-GSO:{},UDP-GSO:{}",
        device1.tcp_gso(),
        device1.udp_gso()
    );
    let (s1, r1) = tokio::sync::mpsc::channel(2048);
    let (s2, r2) = tokio::sync::mpsc::channel(2048);
    let framed1_1 = DeviceFramed::new(device1.clone(), BytesCodec::new());
    let framed1_2 = DeviceFramed::new(device1, BytesCodec::new());
    let framed2_1 = DeviceFramed::new(device2.clone(), BytesCodec::new());
    let framed2_2 = DeviceFramed::new(device2, BytesCodec::new());
    let handle1 = tokio::spawn(dev_to_channel(framed1_1, s1));
    let handle2 = tokio::spawn(dev_to_channel(framed2_1, s2));

    let handle3 = tokio::spawn(channel_to_dev(r1, framed2_2));
    let handle4 = tokio::spawn(channel_to_dev(r2, framed1_2));
    tokio::try_join!(handle1, handle2, handle3, handle4).unwrap();
}
async fn dev_to_channel(
    mut dev: DeviceFramed<BytesCodec, Arc<AsyncDevice>>,
    sender: Sender<BytesMut>,
) {
    loop {
        let buf = dev.next().await.unwrap().unwrap();
        sender.send(buf).await.unwrap();
    }
}
async fn channel_to_dev(
    mut receiver: Receiver<BytesMut>,
    mut dev: DeviceFramed<BytesCodec, Arc<AsyncDevice>>,
) {
    loop {
        let mut count = 1;
        let buf = receiver.recv().await.unwrap();
        dev.feed(buf).await.unwrap();
        while let Ok(buf) = receiver.try_recv() {
            dev.feed(buf).await.unwrap();
            count += 1;
            if count >= IDEAL_BATCH_SIZE {
                break;
            }
        }
        <DeviceFramed<BytesCodec, Arc<AsyncDevice>> as SinkExt<BytesMut>>::flush(&mut dev)
            .await
            .unwrap();
    }
}
