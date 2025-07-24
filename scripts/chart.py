import matplotlib.pyplot as plt
import numpy as np

labels = [
    'Async', 'Async+channel', 'AsyncFramed+channel', 'Async+offload',
    'Async+offload+channel', 'Async+offload+channel+BytesPool', 'AsyncFramed+offload+channel',
    'Sync', 'Sync+channel', 'Sync+offload', 'Sync+offload+channel', 'Sync+offload+Concurrent'
]
rust_gbps =  [8.84, 12.1, 12.0, 35.7, 24.9, 31.4, 23.7, 10.0, 13.0, 36.4, 29.5, 70.6]
go_gbps   =  [8.29, 8.61, 0, 28.8, 24.0, 30.1, 0, 0, 0, 0, 0, 0]

x = np.arange(len(labels))
width = 0.35

fig, ax = plt.subplots(figsize=(14,6))
rects1 = ax.bar(x - width/2, rust_gbps, width, label='Rust', color='#2062ea')
rects2 = ax.bar(x + width/2, go_gbps, width, label='Golang', color='#3ec3fc')

ax.set_ylabel('Throughput (Gbps)')
ax.set_title('TUN implementation throughput (Gbps)', fontsize=15, weight='bold', loc='left')
ax.set_xticks(x)
ax.set_xticklabels(labels, rotation=22, ha="right", fontsize=10)
ax.legend()

def autolabel(rects):
    for rect in rects:
        height = rect.get_height()
        if height > 0:
            ax.annotate(f'{height}',
                        xy=(rect.get_x() + rect.get_width() / 2, height),
                        xytext=(0, 4),
                        textcoords="offset points",
                        ha='center', va='bottom', fontsize=10, color='black')

autolabel(rects1)
autolabel(rects2)

plt.tight_layout()
plt.savefig("/mnt/data/tun_throughput_compare.png", dpi=160)
plt.close()
