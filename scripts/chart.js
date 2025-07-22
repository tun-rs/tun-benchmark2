const spec = {
  type: 'bar',
  data: [
    {
      id: 'barData',
      values: [
        { type: 'Rust', mode: 'Async', value: 8.84 },
        { type: 'Golang', mode: 'Async', value: 8.29 },
        { type: 'Rust', mode: 'Async+Channel', value: 12.1},
        { type: 'Golang', mode: 'Async+Channel', value: null },
        { type: 'Rust', mode: 'AsyncFramed+Channel', value: 12.0},
        { type: 'Golang', mode: 'AsyncFramed+Channel', value: null },
        { type: 'Rust', mode: 'Async+Offload', value: 35.7},
        { type: 'Golang', mode: 'Async+Offload', value: 28.8 },
        { type: 'Rust', mode: 'Async+channel+offload', value: 20.7},
        { type: 'Golang', mode: 'Async+channel+offload', value: null },
        { type: 'Rust', mode: 'AsyncFramed+channel+offload', value: 23.7},
        { type: 'Rust', mode: 'Sync', value: 10.0},
        { type: 'Rust', mode: 'Sync+Channel', value: 13.0},
        { type: 'Rust', mode: 'Sync+Offload', value: 36.4},
        { type: 'Rust', mode: 'Sync+channel+Offload', value: 33.7},
        { type: 'Rust', mode: 'Sync+Offload+Cocurrent', value: 70.6},
      ]
    }
  ],
  xField: ['mode', 'type'],
  yField: 'value',
  title:{
    visible:true,
    text: 'TUN implementation throughput(Gbps)',
  },
  seriesField: 'type',
    axes: [
    {
      orient: 'bottom',
      sampling: false,
      label: {
        autoRotate: true,
        autoRotateAngle: [0, 60]
      }
    }
  ],
  legends: {
    visible: true,
    orient: 'bottom',
    position: 'middle'
  }
};

const vchart = new VChart(spec, { dom: CONTAINER_ID });
vchart.renderSync();

// Just for the convenience of console debugging, DO NOT COPY!
window['vchart'] = vchart;
