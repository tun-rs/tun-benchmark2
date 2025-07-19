# TUN Benchmark Tool

This is a benchmarking tool for measuring the performance of TUN interface implementations on Linux using different
configurations and libraries.

## Environment

- **OS:** Ubuntu 20.04.6 LTS(6.14.0-24-generic)
- **CPU:** i7-13700K
- **Memory:** DDR5 32GB（2×16GB, 4800 MT/s）
- **Benchmark Tool:** iperf3
- **Baseline Performance (Loopback via TUN IP):** ~110 Gbps
- **TUN Libraries:** [`tun-rs`](https://github.com/tun-rs/tun-rs)

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

| #   | Mode        | Offload | Channel | Gbps | Retr  | CPU Avg | CPU Max | Mem Avg | Mem Max |
|-----|-------------|---------|---------|------|-------|---------|---------|---------|---------|
| 1   | Async       | ❌       | ❌       | 8.84 | 326   | 87.61   | 131.00  | 3.71    | 3.71    |
| 2   | Async       | ❌       | ✅       | 12.1 | 3513  | 126.87  | 190.00  | 10.04   | 11.44   |
| 3   | AsyncFramed | ❌       | ✅       | 12.0 | 3967  | 126.89  | 190.00  | 11.47   | 14.60   |
| 4   | Async       | ✅       | ❌       | 35.7 | 0     | 65.79   | 98.50   | 20.62   | 20.62   |
| 5   | Async       | ✅       | ✅       | 20.7 | 0     | 87.65   | 131.00  | 293.40  | 329.55  |
| 6   | AsyncFramed | ✅       | ✅       | 23.7 | 0     | 88.46   | 132.00  | 26.27   | 28.92   |
| 7   | Sync        | ❌       | ❌       | 10.0 | 804   | 79.74   | 119.00  | 2.21    | 2.21    |
| 8   | Sync        | ❌       | ✅       | 13.0 | 5585  | 136.90  | 205.00  | 3.97    | 4.23    |
| 9   | Sync        | ✅       | ❌       | 36.4 | 0     | 58.02   | 86.80   | 20.43   | 20.43   |
| 10  | Sync        | ✅       | ✅       | 33.7 | 0     | 95.27   | 143.00  | 111.30  | 140.10  |
| 11* | Sync        | ✅       | ❌       | 70.6 | 2748  | 121.64  | 181.00  | 38.33   | 38.33   |


\* Test 11 uses dual-threaded concurrent I/O with GSO enabled (no channel), yielding peak throughput.

- **Channel**: Channel buffering
- **Gbps**: Bitrate
- **Retr**: Retransmissions
- **CPU Avg/Max**: usage in %
- **Mem Avg/Max**: usage in MB

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
Avg CPU:    65.79 %
Max CPU:    98.50 %
Avg Memory: 20.62 MB
Max Memory: 20.62 MB
```

![tun-rs-async-gso-flamegraph.svg](flamegraph/tun-rs-async-gso-flamegraph.svg)

### 5. TUN with Offload + Channel Buffering (Async)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 41218 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  24.1 GBytes  20.7 Gbits/sec    0   4.11 MBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  24.1 GBytes  20.7 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  24.1 GBytes  20.7 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    87.65 %
Max CPU:    131.00 %
Avg Memory: 293.40 MB
Max Memory: 329.55 MB
```

![tun-rs-async-gso-channel-flamegraph.svg](flamegraph/tun-rs-async-gso-channel-flamegraph.svg)

### 6. TUN with Offload + DeviceFramed (Async)

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

### 7. Basic TUN Read/Write (Sync)

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

### 8. Basic TUN with Channel Buffering (Sync)

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

### 9. TUN with Offload Enabled (Sync)

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
Avg CPU:    58.02 %
Max CPU:    86.80 %
Avg Memory: 20.43 MB
Max Memory: 20.43 MB
```

![tun-rs-sync-gso-flamegraph.svg](flamegraph/tun-rs-sync-gso-flamegraph.svg)

### 10. TUN with Offload + Channel Buffering (Sync)

```text
Connecting to host 10.0.2.1, port 5201
[  5] local 10.0.1.1 port 38892 connected to 10.0.2.1 port 5201                                                                                                                                                                                                                                                                                                                                                                                                            
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd                                                                                                                                                                                                                                                                                                                                                                                                           
[  5]   0.00-10.01  sec  39.2 GBytes  33.7 Gbits/sec    0   4.17 MBytes                                                                                                                                                                                                                                                                                                                                                                                                    
- - - - - - - - - - - - - - - - - - - - - - - - -                                                                                                                                                                                                                                                                                                                                                                                                                          
[ ID] Interval           Transfer     Bitrate         Retr                                                                                                                                                                                                                                                                                                                                                                                                                 
[  5]   0.00-10.01  sec  39.2 GBytes  33.7 Gbits/sec    0             sender                                                                                                                                                                                                                                                                                                                                                                                               
[  5]   0.00-10.01  sec  39.2 GBytes  33.7 Gbits/sec                  receiver                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
iperf Done.                                                                                                                                                                                                                                                                                                                                                                                                                                                                
=== Monitor Summary ===
Avg CPU:    95.27 %
Max CPU:    143.00 %
Avg Memory: 111.30 MB
Max Memory: 140.10 MB
```

![tun-rs-sync-gso-channel-flamegraph.svg](flamegraph/tun-rs-sync-gso-channel-flamegraph.svg)

### 11. TUN with Offload + Dual-Threaded Concurrent I/O (Sync)

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
Avg CPU:    121.64 %
Max CPU:    181.00 %
Avg Memory: 38.33 MB
Max Memory: 38.33 MB
```

![tun-rs-sync-gso-concurrent-flamegraph.svg](flamegraph/tun-rs-sync-gso-concurrent-flamegraph.svg)
