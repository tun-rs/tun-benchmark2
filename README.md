# TUN Benchmark Tool

This is a benchmarking tool for measuring the performance of TUN interface implementations on Linux using different
configurations and libraries.

## Environment

- **OS:** Ubuntu 20.04.6 LTS(6.14.0-24-generic)
- **CPU:** i7-13700K
- **Memory:** DDR5 32GB（2×16GB, 4800 MT/s）
- **Benchmark Tool:** iperf3
- **Baseline Performance (Loopback via TUN IP):** ~110 Gbps
- **TUN Libraries:** [`tun-rs 2.5.1`](https://github.com/tun-rs/tun-rs)

## Test

Each test uses `iperf3` to send traffic from `10.0.1.1` (via `tun11`) to `10.0.2.1` (via `tun22`). All interfaces are
handled using a Rust-based TUN forwarder, either in async or sync mode, with optional channel buffering and offload.

```shell
 # Test all test cases
 ./scripts/bench.sh
 # Run the specified test case with parameters
 ./scripts/bench.sh "./target/release/tun-rs-async-gso-channel --thread 2"
```

## Benchmark Summary Table

| #   | Mode                                                        | Offload | Channel | Gbps | Retr | CPU Avg | CPU Max | Mem Avg | Mem Max |
|-----|-------------------------------------------------------------|---------|---------|------|------|---------|---------|---------|---------|
| 1   | Async                                                       | ❌       | ❌       | 8.84 | 326  | 87.61   | 131.00  | 3.71    | 3.71    |
| 2   | Async                                                       | ❌       | ✅       | 12.1 | 3513 | 126.87  | 190.00  | 10.04   | 11.44   |
| 3   | AsyncFramed                                                 | ❌       | ✅       | 12.0 | 3967 | 126.89  | 190.00  | 11.47   | 14.60   |
| 4   | Async                                                       | ✅       | ❌       | 35.7 | 0    | 64.89   | 97.70   | 7.44    | 7.44    |
| 5   | Async                                                       | ✅       | ✅       | 24.9 | 0    | 85.02   | 127.00  | 24.78   | 26.57   |
| 6   | Async + BytesPool                                           | ✅       | ✅       | 31.4 | 0    | 92.97   | 139.00  | 15.95   | 16.05   |
| 7   | AsyncFramed                                                 | ✅       | ✅       | 23.7 | 0    | 88.46   | 132.00  | 26.27   | 28.92   |
| 8   | Sync                                                        | ❌       | ❌       | 10.0 | 804  | 79.74   | 119.00  | 2.21    | 2.21    |
| 9   | Sync                                                        | ❌       | ✅       | 13.0 | 5585 | 136.90  | 205.00  | 3.97    | 4.23    |
| 10  | Sync                                                        | ✅       | ❌       | 36.4 | 0    | 57.90   | 86.90   | 6.80    | 6.80    |
| 11  | Sync                                                        | ✅       | ✅       | 29.5 | 0    | 90.38   | 135.00  | 14.87   | 15.00   |
| 12* | Sync (Concurrent)                                           | ✅       | ❌       | 70.6 | 2748 | 124.49  | 185.00  | 10.65   | 10.65   |
| 13  | [Golang](https://github.com/tun-rs/go_tun_test)             | ❌       | ❌       | 8.29 | 541  | 84.95   | 127.00  | 2.46    | 2.46    |
| 14  | [Golang](https://github.com/tun-rs/go_tun_test)             | ❌       | ✅       | 8.61 | 4436 | 158.77  | 239.00  | 7.85    | 8.58    |
| 15  | [Golang](https://github.com/tun-rs/go_tun_test)             | ✅       | ❌       | 28.8 | 0    | 64.14   | 96.20   | 4.15    | 4.15    |
| 16  | [Golang](https://github.com/tun-rs/go_tun_test)             | ✅       | ✅       | 24.0 | 0    | 109.78  | 164.00  | 40.09   | 42.20   |
| 17  | [Golang + BytesPool](https://github.com/tun-rs/go_tun_test) | ✅       | ✅       | 30.1 | 0    | 101.62  | 152.00  | 39.53   | 43.15   |

\* Test 12 uses dual-threaded concurrent I/O with GSO enabled (no channel), yielding peak throughput.

- **Channel**: Channel buffering
- **Gbps**: Bitrate
- **Retr**: Retransmissions
- **CPU Avg/Max**: usage in %
- **Mem Avg/Max**: usage in MB

![throughput_chart.png](flamegraph/canvas.png)  

### 1. Basic TUN Read/Write (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 39800 connected to 10.0.2.1 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  10.3 GBytes  8.84 Gbits/sec  326    626 KBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  10.3 GBytes  8.84 Gbits/sec  326             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  10.3 GBytes  8.84 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    87.61 %
Max CPU:    131.00 %
Avg Memory: 3.71 MB
Max Memory: 3.71 MB
```

![tun-rs-async-normal-flamegraph.svg](flamegraph/tun-rs-async-normal-flamegraph.svg)

### 2. Basic TUN with Channel Buffering (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 51256 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  14.1 GBytes  12.1 Gbits/sec  3513   2.00 MBytes                                                                                                                                                                                                                                                                                                                                                                                                   
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  14.1 GBytes  12.1 Gbits/sec  3513             sender                                                                                                                                                                                                                                                                                                                                                                                              
[  5]   0.00-10.01  sec  14.1 GBytes  12.1 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    126.87 %
Max CPU:    190.00 %
Avg Memory: 10.04 MB
Max Memory: 11.44 MB
```

![tun-rs-async-normal-channel-flamegraph.svg](flamegraph/tun-rs-async-normal-channel-flamegraph.svg)

### 3. DeviceFramed (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 33882 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  14.0 GBytes  12.0 Gbits/sec  3967   1.62 MBytes                                                                                                                                                                                                                                                                                                                                                                                                   
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  14.0 GBytes  12.0 Gbits/sec  3967             sender                                                                                                                                                                                                                                                                                                                                                                                              
[  5]   0.00-10.01  sec  14.0 GBytes  12.0 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    126.89 %
Max CPU:    190.00 %
Avg Memory: 11.47 MB
Max Memory: 14.60 MB
```

![tun-rs-async-framed-flamegraph.svg](flamegraph/tun-rs-async-framed-flamegraph.svg)

### 4. TUN with Offload Enabled (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 36108 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  41.6 GBytes  35.7 Gbits/sec    0   4.17 MBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  41.6 GBytes  35.7 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  41.6 GBytes  35.7 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    64.89 %
Max CPU:    97.70 %
Avg Memory: 7.44 MB
Max Memory: 7.44 MB
```

![tun-rs-async-gso-flamegraph.svg](flamegraph/tun-rs-async-gso-flamegraph.svg)

### 5. TUN with Offload + Channel Buffering (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 35300 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  29.0 GBytes  24.9 Gbits/sec    0   4.15 MBytes                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  29.0 GBytes  24.9 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  29.0 GBytes  24.9 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    85.02 %
Max CPU:    127.00 %
Avg Memory: 24.78 MB
Max Memory: 26.57 MB
```

![tun-rs-async-gso-channel-flamegraph.svg](flamegraph/tun-rs-async-gso-channel-flamegraph.svg)

### 6. TUN with Offload + Channel Buffering + Bytes Pool (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 37966 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  36.6 GBytes  31.4 Gbits/sec    0   4.19 MBytes                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  36.6 GBytes  31.4 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  36.6 GBytes  31.4 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    92.97 %
Max CPU:    139.00 %
Avg Memory: 15.95 MB
Max Memory: 16.05 MB
```

![tun-rs-async-gso-channel-pool-flamegraph.svg](flamegraph/tun-rs-async-gso-channel-pool-flamegraph.svg)

### 7. TUN with Offload + DeviceFramed (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 40948 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  27.6 GBytes  23.7 Gbits/sec    0   4.13 MBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  27.6 GBytes  23.7 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  27.6 GBytes  23.7 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    88.46 %
Max CPU:    132.00 %
Avg Memory: 26.27 MB
Max Memory: 28.92 MB
```

![tun-rs-async-gso-framed-flamegraph.svg](flamegraph/tun-rs-async-gso-framed-flamegraph.svg)

### 8. Basic TUN Read/Write (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 40824 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  11.7 GBytes  10.0 Gbits/sec  804    491 KBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  11.7 GBytes  10.0 Gbits/sec  804             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  11.7 GBytes  10.0 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    79.74 %
Max CPU:    119.00 %
Avg Memory: 2.21 MB
Max Memory: 2.21 MB
```

![tun-rs-async-gso-framed-flamegraph.svg](flamegraph/tun-rs-async-gso-framed-flamegraph.svg)

### 9. Basic TUN with Channel Buffering (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 54696 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  15.2 GBytes  13.0 Gbits/sec  5585   1.13 MBytes                                                                                                                                                                                                                                                                                                                                                                                                   
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  15.2 GBytes  13.0 Gbits/sec  5585             sender                                                                                                                                                                                                                                                                                                                                                                                              
[  5]   0.00-10.01  sec  15.2 GBytes  13.0 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    136.90 %
Max CPU:    205.00 %
Avg Memory: 3.97 MB
Max Memory: 4.23 MB
```

![tun-rs-sync-normal-channel-flamegraph.svg](flamegraph/tun-rs-sync-normal-channel-flamegraph.svg)

### 10. TUN with Offload Enabled (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 36196 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  42.4 GBytes  36.4 Gbits/sec    0   4.07 MBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  42.4 GBytes  36.4 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  42.4 GBytes  36.4 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    57.90 %
Max CPU:    86.90 %
Avg Memory: 6.80 MB
Max Memory: 6.80 MB
```

![tun-rs-sync-gso-flamegraph.svg](flamegraph/tun-rs-sync-gso-flamegraph.svg)

### 11. TUN with Offload + Channel Buffering (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 38904 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  34.4 GBytes  29.5 Gbits/sec    0   4.14 MBytes                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  34.4 GBytes  29.5 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  34.4 GBytes  29.5 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    90.38 %
Max CPU:    135.00 %
Avg Memory: 14.87 MB
Max Memory: 15.00 MB
```

![tun-rs-sync-gso-channel-flamegraph.svg](flamegraph/tun-rs-sync-gso-channel-flamegraph.svg)

### 12. TUN with Offload + Dual-Threaded Concurrent I/O (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 35190 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  82.3 GBytes  70.6 Gbits/sec  2748   3.98 MBytes                                                                                                                                                                                                                                                                                                                                                                                                   
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  82.3 GBytes  70.6 Gbits/sec  2748             sender                                                                                                                                                                                                                                                                                                                                                                                              
[  5]   0.00-10.01  sec  82.3 GBytes  70.6 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    124.49 %
Max CPU:    185.00 %
Avg Memory: 10.65 MB
Max Memory: 10.65 MB
```

![tun-rs-sync-gso-concurrent-flamegraph.svg](flamegraph/tun-rs-sync-gso-concurrent-flamegraph.svg)

### 13. Go TUN Implementation: Basic Read/Write (No Offload)

https://github.com/tun-rs/go_tun_test

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 58348 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.00  sec  9.65 GBytes  8.29 Gbits/sec  541    505 KBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.00  sec  9.65 GBytes  8.29 Gbits/sec  541             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.00  sec  9.65 GBytes  8.29 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    84.95 %
Max CPU:    127.00 %
Avg Memory: 2.46 MB
Max Memory: 2.46 MB
```

![go-tun-normal-flamegraph.svg](flamegraph/go-tun-normal-flamegraph.svg)

### 14. Go TUN Implementation:  Read/Write + Channel (No Offload)

https://github.com/tun-rs/go_tun_test

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 47252 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  10.0 GBytes  8.61 Gbits/sec  4436    634 KBytes                                                                                                                                                                                                                                                                                                   
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  10.0 GBytes  8.61 Gbits/sec  4436             sender                                                                                                                                                                                                                                                                                              
[  5]   0.00-10.01  sec  10.0 GBytes  8.60 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    158.77 %
Max CPU:    239.00 %
Avg Memory: 7.85 MB
Max Memory: 8.58 MB
```

![go-tun-normal-channel-flamegraph.svg](flamegraph/go-tun-normal-channel-flamegraph.svg)

### 15. Go TUN Implementation: With Offload (GSO/GRO Enabled)

https://github.com/tun-rs/go_tun_test

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 57976 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  33.6 GBytes  28.8 Gbits/sec    0   4.14 MBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  33.6 GBytes  28.8 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  33.6 GBytes  28.8 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    64.14 %
Max CPU:    96.20 %
Avg Memory: 4.15 MB
Max Memory: 4.15 MB
```

![go-tun-offload-flamegraph.svg](flamegraph/go-tun-offload-flamegraph.svg)

### 16. Go TUN Implementation: With Offload + Channel (GSO/GRO Enabled)

https://github.com/tun-rs/go_tun_test

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 50800 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.00  sec  27.9 GBytes  24.0 Gbits/sec    0   4.18 MBytes                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.00  sec  27.9 GBytes  24.0 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.00  sec  27.9 GBytes  24.0 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    109.78 %
Max CPU:    164.00 %
Avg Memory: 40.09 MB
Max Memory: 42.20 MB
```

![go-tun-offload-channel-flamegraph.svg](flamegraph/go-tun-offload-channel-flamegraph.svg)

### 17. Go TUN Implementation: With Offload + Channel + Bytes Pool (GSO/GRO Enabled)

https://github.com/tun-rs/go_tun_test

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 36524 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  35.1 GBytes  30.1 Gbits/sec    0   4.12 MBytes                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  35.1 GBytes  30.1 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  35.1 GBytes  30.1 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    101.62 %
Max CPU:    152.00 %
Avg Memory: 39.53 MB
Max Memory: 43.15 MB
```

![go-tun-offload-channel-pool-flamegraph.svg](flamegraph/go-tun-offload-channel-pool-flamegraph.svg)
