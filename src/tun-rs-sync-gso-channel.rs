use bytes::BytesMut;
use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use tun_rs::{DeviceBuilder, GROTable, IDEAL_BATCH_SIZE, SyncDevice, VIRTIO_NET_HDR_LEN};

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
        .offload(true)
        .build_sync()
        .unwrap();
    let device2 = DeviceBuilder::new()
        .name(iface2)
        .ipv4(ip2, 24, None)
        .mtu(1500)
        .offload(true)
        .build_sync()
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
    let (s1, r1) = std::sync::mpsc::sync_channel(2048);
    let (s2, r2) = std::sync::mpsc::sync_channel(2048);
    {
        let device1 = device1.clone();
        thread::spawn(move || {
            dev_to_channel(device1, s1);
        });
    }
    {
        let device2 = device2.clone();
        thread::spawn(move || {
            dev_to_channel(device2, s2);
        });
    }
    thread::spawn(move || channel_to_dev(r1, device2));
    thread::spawn(move || channel_to_dev(r2, device1));

    let (tx, rx) = std::sync::mpsc::channel();

    let _handle = ctrlc2::set_handler(move || {
        tx.send(()).expect("Signal error.");
        true
    });
    _ = rx.recv();
}
fn dev_to_channel(dev: Arc<SyncDevice>, sender: SyncSender<BytesMut>) {
    let mut original_buffer = vec![0; VIRTIO_NET_HDR_LEN + 65535];
    let mut bufs = Vec::with_capacity(IDEAL_BATCH_SIZE);
    for _ in 0..IDEAL_BATCH_SIZE {
        bufs.push(BytesMut::zeroed(1500));
    }
    let mut sizes = vec![0; IDEAL_BATCH_SIZE];
    loop {
        let num = dev
            .recv_multiple(&mut original_buffer, &mut bufs, &mut sizes, 0)
            .unwrap();
        if num == 0 {
            panic!("eof")
        }
        for i in 0..num {
            let buf = BytesMut::from(&bufs[i][..sizes[i]]);
            sender.send(buf).unwrap();
        }
    }
}
fn channel_to_dev(receiver: Receiver<BytesMut>, dev: Arc<SyncDevice>) {
    let mut send_bufs = Vec::with_capacity(IDEAL_BATCH_SIZE);
    for _ in 0..IDEAL_BATCH_SIZE {
        // Reserve sufficient space in the buffer to avoid reallocations during send_multiple execution.
        // This is critical for performance, especially under high throughput scenarios.
        send_bufs.push(BytesMut::with_capacity(VIRTIO_NET_HDR_LEN + 65536));
    }
    let mut gro_table = GROTable::default();

    loop {
        let mut n = 0;
        let bytes_mut = receiver.recv().unwrap();
        send_bufs[n].clear();
        send_bufs[n].resize(VIRTIO_NET_HDR_LEN, 0);
        send_bufs[n].extend_from_slice(&bytes_mut);
        n += 1;
        while let Ok(buf) = receiver.try_recv() {
            send_bufs[n].clear();
            send_bufs[n].resize(VIRTIO_NET_HDR_LEN, 0);
            send_bufs[n].extend_from_slice(&buf);
            n += 1;
            if n >= IDEAL_BATCH_SIZE {
                break;
            }
        }
        dev.send_multiple(&mut gro_table, &mut send_bufs[..n], VIRTIO_NET_HDR_LEN)
            .unwrap();
    }
}
