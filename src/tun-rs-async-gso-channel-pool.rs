use bytes::BytesMut;
use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tun_rs::{AsyncDevice, DeviceBuilder, GROTable, IDEAL_BATCH_SIZE, VIRTIO_NET_HDR_LEN};

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
    let (pool_s1, pool_r1) = std::sync::mpsc::sync_channel(2048);
    let (pool_s2, pool_r2) = std::sync::mpsc::sync_channel(2048);

    let handle1 = tokio::spawn(dev_to_channel(device1.clone(), s1, pool_r1));
    let handle2 = tokio::spawn(dev_to_channel(device2.clone(), s2, pool_r2));

    let handle3 = tokio::spawn(channel_to_dev(r1, device2, pool_s1));
    let handle4 = tokio::spawn(channel_to_dev(r2, device1, pool_s2));
    tokio::try_join!(handle1, handle2, handle3, handle4).unwrap();
}
async fn dev_to_channel(
    dev: Arc<AsyncDevice>,
    sender: Sender<BytesMut>,
    pool_r: std::sync::mpsc::Receiver<BytesMut>,
) {
    let mut original_buffer = vec![0; VIRTIO_NET_HDR_LEN + 65535];
    let mut bufs = Vec::with_capacity(IDEAL_BATCH_SIZE);
    for _ in 0..IDEAL_BATCH_SIZE {
        bufs.push(BytesMut::zeroed(1500));
    }
    let mut sizes = vec![0; IDEAL_BATCH_SIZE];
    loop {
        let num = dev
            .recv_multiple(&mut original_buffer, &mut bufs, &mut sizes, 0)
            .await
            .unwrap();
        if num == 0 {
            panic!("eof")
        }
        for i in 0..num {
            let mut buf = pool_r
                .try_recv()
                .unwrap_or_else(|_| BytesMut::with_capacity(1500));
            buf.extend_from_slice(&bufs[i][..sizes[i]]);
            sender.send(buf).await.unwrap();
        }
    }
}
async fn channel_to_dev(
    mut receiver: Receiver<BytesMut>,
    dev: Arc<AsyncDevice>,
    pool_s: std::sync::mpsc::SyncSender<BytesMut>,
) {
    let mut send_bufs = Vec::with_capacity(IDEAL_BATCH_SIZE);
    for _ in 0..IDEAL_BATCH_SIZE {
        // Reserve sufficient space in the buffer to avoid reallocations during send_multiple execution.
        // This is critical for performance, especially under high throughput scenarios.
        send_bufs.push(BytesMut::with_capacity(VIRTIO_NET_HDR_LEN + 65536));
    }
    let mut gro_table = GROTable::default();

    loop {
        let mut n = 0;
        let mut bytes_mut = receiver.recv().await.unwrap();
        send_bufs[n].clear();
        send_bufs[n].resize(VIRTIO_NET_HDR_LEN, 0);
        send_bufs[n].extend_from_slice(&bytes_mut);
        bytes_mut.clear();
        _ = pool_s.try_send(bytes_mut);
        n += 1;
        while let Ok(mut buf) = receiver.try_recv() {
            send_bufs[n].clear();
            send_bufs[n].resize(VIRTIO_NET_HDR_LEN, 0);
            send_bufs[n].extend_from_slice(&buf);
            buf.clear();
            _ = pool_s.try_send(buf);
            n += 1;
            if n >= IDEAL_BATCH_SIZE {
                break;
            }
        }
        dev.send_multiple(&mut gro_table, &mut send_bufs[..n], VIRTIO_NET_HDR_LEN)
            .await
            .unwrap();
    }
}
