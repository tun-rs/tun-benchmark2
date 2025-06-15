# TUN Benchmark Tool

This is a benchmarking tool for measuring the performance of TUN interface implementations on Linux using different
configurations and libraries.

## Environment

- **OS:** Linux
- **CPU:** 3.8GHz, 16-core
- **Benchmark Tool:** iperf3
- **TUN Libraries:** [`tun-rs`](https://github.com/tun-rs/tun-rs)

## Configurations Tested

Each test uses `iperf3` to send traffic from `10.0.1.1` (via `tun11`) to `10.0.2.1` (via `tun22`). All interfaces are
handled using a Rust-based TUN forwarder, either in async or sync mode, with optional channel buffering and offload.


---

## Benchmark Summary Table

| # | Mode  | Offload | Channel | Bitrate (Gbps) | Retransmissions | Avg CPU (%) | Max CPU (%) | Avg Mem (MB) | Max Mem (MB) |
|---|-------|---------|---------|----------------|-----------------|-------------|-------------|--------------|--------------|
| 1 | Async | ❌       | ❌       | 6.01           | 1429            | 76.31       | 114.00      | 3.09         | 3.09         |
| 2 | Async | ❌       | ✅       | 6.88           | 2421            | 96.51       | 149.00      | 7.18         | 8.06         |
| 3 | Async | ✅       | ❌       | 12.4           | 0               | 57.91       | 86.70       | 21.31        | 21.31        |
| 4 | Async | ✅       | ✅       | 10.2           | 0               | 84.21       | 126.00      | 180.98       | 225.55       |
| 5 | Sync  | ❌       | ❌       | 7.25           | 8227            | 67.26       | 100.00      | 2.57         | 2.57         |
| 6 | Sync  | ❌       | ✅       | 7.91           | 3543            | 111.01      | 167.00      | 5.45         | 5.73         |
| 7 | Sync  | ✅       | ❌       | 12.5           | 0               | 49.22       | 75.90       | 20.45        | 20.45        |
| 8 | Sync  | ✅       | ✅       | 15.1           | 0               | 89.98       | 135.00      | 118.36       | 140.84       |

### 1. Basic TUN Read/Write (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 49850 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  7.01 GBytes  6.01 Gbits/sec  1429    669 KBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  7.01 GBytes  6.01 Gbits/sec  1429            sender
[  5]   0.00-10.01  sec  7.01 GBytes  6.01 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    76.31 %
Max CPU:    114.00 %
Avg Memory: 3.09 MB
Max Memory: 3.09 MB
```

### 2. Basic TUN with Channel Buffering (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 43606 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  8.02 GBytes  6.88 Gbits/sec  2421    902 KBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  8.02 GBytes  6.88 Gbits/sec  2421            sender
[  5]   0.00-10.01  sec  8.02 GBytes  6.88 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    96.51 %
Max CPU:    149.00 %
Avg Memory: 7.18 MB
Max Memory: 8.06 MB
```

### 3. TUN with Offload Enabled (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 40180 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  14.5 GBytes  12.4 Gbits/sec    0   4.19 MBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  14.5 GBytes  12.4 Gbits/sec    0            sender
[  5]   0.00-10.01  sec  14.5 GBytes  12.4 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    57.91 %
Max CPU:    86.70 %
Avg Memory: 21.31 MB
Max Memory: 21.31 MB
```

### 4. TUN with Offload + Channel Buffering (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 44966 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  11.9 GBytes  10.2 Gbits/sec    0   4.11 MBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  11.9 GBytes  10.2 Gbits/sec    0            sender
[  5]   0.00-10.01  sec  11.9 GBytes  10.2 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    84.21 %
Max CPU:    126.00 %
Avg Memory: 180.98 MB
Max Memory: 225.55 MB
```

### 5. Basic TUN Read/Write (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 54434 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  8.45 GBytes  7.25 Gbits/sec  8227    551 KBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  8.45 GBytes  7.25 Gbits/sec  8227            sender
[  5]   0.00-10.01  sec  8.44 GBytes  7.25 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    67.26 %
Max CPU:    100.00 %
Avg Memory: 2.57 MB
Max Memory: 2.57 MB
```

### 6. Basic TUN with Channel Buffering (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 43096 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  9.22 GBytes  7.91 Gbits/sec  3543    856 KBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  9.22 GBytes  7.91 Gbits/sec  3543            sender
[  5]   0.00-10.01  sec  9.22 GBytes  7.91 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    111.01 %
Max CPU:    167.00 %
Avg Memory: 5.45 MB
Max Memory: 5.73 MB
```

### 7. TUN with Offload Enabled (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 56684 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  14.5 GBytes  12.5 Gbits/sec    0   4.16 MBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  14.5 GBytes  12.5 Gbits/sec    0            sender
[  5]   0.00-10.01  sec  14.5 GBytes  12.5 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    49.22 %
Max CPU:    75.90 %
Avg Memory: 20.45 MB
Max Memory: 20.45 MB
```

### 8. TUN with Offload + Channel Buffering (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 52910 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-10.01  sec  17.6 GBytes  15.1 Gbits/sec    0   4.14 MBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.01  sec  17.6 GBytes  15.1 Gbits/sec    0            sender
[  5]   0.00-10.01  sec  17.6 GBytes  15.1 Gbits/sec                  receiver

iperf Done.

=== Monitor Summary ===
Avg CPU:    89.98 %
Max CPU:    135.00 %
Avg Memory: 118.36 MB
Max Memory: 140.84 MB
```
