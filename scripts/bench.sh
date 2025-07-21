#!/bin/bash

cargo build --bin=tun-rs-async-normal --release
cargo build --bin=tun-rs-async-normal-channel --release
cargo build --bin=tun-rs-async-framed --release
cargo build --bin=tun-rs-async-gso --release
cargo build --bin=tun-rs-async-gso-channel --release
cargo build --bin=tun-rs-async-gso-framed --release

cargo build --bin=tun-rs-sync-normal --release
cargo build --bin=tun-rs-sync-normal-channel --release
cargo build --bin=tun-rs-sync-gso --release
cargo build --bin=tun-rs-sync-gso-concurrent --release
cargo build --bin=tun-rs-sync-gso-channel --release

TUN_IFACE0="tun11"
TUN_IFACE1="tun22"

IP0=10.0.1.1
IP1=10.0.2.1
TEST_DURATION=10

GREEN='\033[1;32m'
BLUE='\033[1;34m'
RED='\033[1;31m'
NC='\033[0m'

print_green() { echo -e "${GREEN}$1${NC}"; }
print_blue() { echo -e "${BLUE}$1${NC}"; }
print_red() { echo -e "${RED}$1${NC}"; }

# Create namespace and move TUN_IFACE1 into it
setup_ns1() {
    print_blue "[1] Setting up network namespace: ns1"
    sudo ip netns add ns1
    sudo ip link set $TUN_IFACE1 netns ns1
    sudo ip netns exec ns1 ip addr add 10.0.2.1/24 dev $TUN_IFACE1
    sudo ip netns exec ns1 ip link set $TUN_IFACE1 up
    sudo ip netns exec ns1 ip route add 10.0.0.0/16 dev $TUN_IFACE1
    sudo ip route add $IP1/32 dev $TUN_IFACE0
}

# Delete the namespace
cleanup_ns1() {
    print_blue "[6] Cleaning up network namespace"
    sudo ip netns delete ns1
}
MONITOR_LOG=""
MONITOR_PID=""

start_monitor() {
    local pid="$1"
    MONITOR_LOG="/tmp/monitor_${pid}.log"
    MONITOR_PID_FILE="/tmp/monitor_${pid}.pid"
    MONITOR_PID=""

    cat /dev/null > "$MONITOR_LOG"

    (
        COUNT=0
        MAX_COUNT=10

        while [ "$COUNT" -lt "$MAX_COUNT" ]; do
            if ! ps -p "$pid" > /dev/null; then
                break
            fi
            ts=$(date +%s)
            read -r cpu mem < <(ps -p "$pid" -o %cpu,rss --no-headers)
            echo "$ts $cpu $mem" >> "$MONITOR_LOG"
            COUNT=$((COUNT + 1))
            sleep 1
        done
    ) &

    MONITOR_PID=$!
    echo "$MONITOR_PID" > "$MONITOR_PID_FILE"
}

get_monitor_result() {
    if [ -z "$MONITOR_LOG" ] || [ ! -f "$MONITOR_LOG" ]; then
        echo "Monitor log not found."
        return
    fi

    if [ -f "$MONITOR_PID_FILE" ]; then
        MONITOR_PID=$(cat "$MONITOR_PID_FILE")
        wait "$MONITOR_PID" 2>/dev/null
        rm -f "$MONITOR_PID_FILE"
    fi

    local count
    count=$(wc -l < "$MONITOR_LOG")
    if [ "$count" -eq 0 ]; then
        echo "No monitor data collected."
        return
    fi

    CPU_AVG=$(awk '{sum+=$2} END{printf "%.2f", sum/NR}' "$MONITOR_LOG")
    CPU_MAX=$(awk 'BEGIN{max=0} {if($2>max) max=$2} END{printf "%.2f", max}' "$MONITOR_LOG")
    MEM_AVG_KB=$(awk '{sum+=$3} END{printf "%.2f", sum/NR}' "$MONITOR_LOG")
    MEM_MAX_KB=$(awk 'BEGIN{max=0} {if($3>max) max=$3} END{printf "%.2f", max}' "$MONITOR_LOG")

    MEM_AVG_MB=$(echo "scale=2; $MEM_AVG_KB / 1024" | bc)
    MEM_MAX_MB=$(echo "scale=2; $MEM_MAX_KB / 1024" | bc)

    echo "=== Monitor Summary ==="
    echo "Avg CPU:    $CPU_AVG %"
    echo "Max CPU:    $CPU_MAX %"
    echo "Avg Memory: $MEM_AVG_MB MB"
    echo "Max Memory: $MEM_MAX_MB MB"

    rm -f "$MONITOR_LOG"
}

# Run benchmark using iperf3
run_benchmark() {
    local prog_name="$1"
    print_blue "[2] Starting forwarder: $prog_name"
    $prog_name --iface1 $TUN_IFACE0 --ip1 $IP0 --iface2 $TUN_IFACE1 --ip2 $IP1 &
    FORWARDER_PID=$!
    sleep 2
    print_green "Forwarder PID: $FORWARDER_PID"

    setup_ns1
    sleep 1

    print_blue "[3] Starting iperf3 server in ns1"
    sudo ip netns exec ns1 iperf3 -s -B $IP1 > /dev/null 2>&1 &
    SERVER_PID=$!
    sleep 1
    print_green "iperf3 server PID: $SERVER_PID"

    print_blue "[4] Running iperf3 client test..."
    start_monitor "$FORWARDER_PID"
    sudo perf record -F 99 -p $FORWARDER_PID -g -- sleep 10 &
    TEST_OUTPUT=$(iperf3 -c $IP1 -t $TEST_DURATION -i 0)
    #echo
    #reset  # Reset colors without clearing screen
    print_green "[5] [$prog_name] iperf3 client test complete. Output below:"
    echo
    print_green "$TEST_OUTPUT"
    get_monitor_result
    kill $SERVER_PID
    kill $FORWARDER_PID
    cleanup_ns1
    sleep 1
    sudo perf script > out.perf
    stackcollapse-perf.pl out.perf > out.folded
    prog_path=${prog_name%% *}
    name=$(basename "$prog_path")
    flamegraph.pl out.folded > $name-flamegraph.svg
}

print_green ">>> Running TUN benchmark test..."

ALL_BENCHMARKS="
./target/release/tun-rs-async-normal
./target/release/tun-rs-async-normal-channel
./target/release/tun-rs-async-gso
./target/release/tun-rs-async-gso-channel
./target/release/tun-rs-async-gso-framed
./target/release/tun-rs-sync-normal
./target/release/tun-rs-sync-normal-channel
./target/release/tun-rs-sync-gso
./target/release/tun-rs-sync-gso-channel
./target/release/tun-rs-sync-gso-concurrent
"

if [ "$#" -eq 0 ]; then
    for bench in $ALL_BENCHMARKS; do
        run_benchmark "$bench"
        sleep 1
    done
else
    for bench in "$@"; do
        run_benchmark "$bench"
        sleep 1
    done
fi
