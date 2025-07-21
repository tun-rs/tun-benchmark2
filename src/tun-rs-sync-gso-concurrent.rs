use bytes::BytesMut;
use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::Arc;
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
    for _ in 0..2 {
        let device1 = device1.clone();
        let device2 = device2.clone();
        thread::spawn(move || {
            copy(device1, device2);
        });
    }
    for _ in 0..2 {
        let device1 = device1.clone();
        let device2 = device2.clone();
        thread::spawn(move || {
            copy(device2, device1);
        });
    }
    let (tx, rx) = std::sync::mpsc::channel();

    let _handle = ctrlc2::set_handler(move || {
        tx.send(()).expect("Signal error.");
        true
    });
    _ = rx.recv();
}
fn copy(device1: Arc<SyncDevice>, device2: Arc<SyncDevice>) {
    let mut original_buffer = vec![0; VIRTIO_NET_HDR_LEN + 65535];
    let mut bufs = Vec::with_capacity(IDEAL_BATCH_SIZE);
    for _ in 0..IDEAL_BATCH_SIZE {
        bufs.push(BytesMut::zeroed(VIRTIO_NET_HDR_LEN + 65535));
    }
    let mut sizes = vec![0; IDEAL_BATCH_SIZE];
    let mut gro_table = GROTable::default();
    loop {
        for x in &mut bufs {
            x.resize(VIRTIO_NET_HDR_LEN + 1500, 0u8);
        }
        let num = device1
            .recv_multiple(
                &mut original_buffer,
                &mut bufs,
                &mut sizes,
                VIRTIO_NET_HDR_LEN,
            )
            .unwrap();
        if num == 0 {
            panic!("eof")
        }
        for i in 0..num {
            bufs[i].truncate(VIRTIO_NET_HDR_LEN + sizes[i]);
        }
        device2
            .send_multiple(&mut gro_table, &mut bufs[..num], VIRTIO_NET_HDR_LEN)
            .unwrap();
    }
}
