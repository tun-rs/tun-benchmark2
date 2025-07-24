const spec = {
  type: 'bar',
  data: [
    {
      id: 'barData',
      values: [
        { type: 'Rust', mode: 'Async', value: 8.84 },
        { type: 'Golang', mode: 'Async', value: 8.29 },
        { type: 'Rust', mode: 'Async+channel', value: 12.1},
        { type: 'Golang', mode: 'Async+channel', value: 8.61 },
        { type: 'Rust', mode: 'AsyncFramed+channel', value: 12.0},
        { type: 'Golang', mode: 'AsyncFramed+channel', value: null },
        { type: 'Rust', mode: 'Async+offload', value: 35.7},
        { type: 'Golang', mode: 'Async+offload', value: 28.8 },
        { type: 'Rust', mode: 'Async+channel+offload', value: 24.9},
        { type: 'Golang', mode: 'Async+channel+offload', value: 24.0 },
        { type: 'Rust', mode: 'AsyncFramed+channel+offload', value: 23.7},
        { type: 'Rust', mode: 'Async+channel+offload+pool', value: 31.4},
        { type: 'Golang', mode: 'Async+channel+offload+pool', value: 30.1 },
        { type: 'Rust', mode: 'Sync', value: 10.0},
        { type: 'Rust', mode: 'Sync+channel', value: 13.0},
        { type: 'Rust', mode: 'Sync+offload', value: 36.4},
        { type: 'Rust', mode: 'Sync+channel+offload', value: 29.5},
        { type: 'Rust', mode: 'Sync+offload+concurrent', value: 70.6},
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
  label: {
    visible: true,
    style:{
      fontSize:10
    }
  },
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
