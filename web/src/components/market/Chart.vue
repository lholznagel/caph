<template>
  <v-chart :init-options="init" :option="option" autoresize></v-chart>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class MyItems extends Vue {
  @Prop(String)
  public id!: string;
  @Prop(Boolean)
  public isBuyOrder!: boolean;

  public init = {
    renderer: 'svg',
  };
  public option = {};

  public async mounted() {
    const market = (await axios.get(`/api/market/${this.id}/historic?buy=${this.isBuyOrder}`)).data;
    const ts = market.map(x => x.ts);
    const series = [{
      name: 'Volume',
      data: market.map(x => [ x.ts, x.volume ]),
      type: 'bar'
    }, {
      name: 'Highest price',
      data: market.map(x => [x.ts, x.highest_price]),
      type: 'line',
      yAxisIndex: 1
    }, {
      name: 'Average price',
      data: market.map(x => [x.ts, x.average_price]),
      type: 'line',
      yAxisIndex: 1
    }, {
      name: 'Lowest price',
      data: market.map(x => [x.ts, x.lowest_price]),
      type: 'line',
      yAxisIndex: 1
    }];

    this.option = {
      legend: {
        textStyle: {
          color: '#fff'
        }
      },
      tooltip: {
        trigger: 'axis',
      },
      xAxis: {
        type: 'time',
        data: ts
      },
      yAxis: [{
        name: 'volume',
        type: 'value',
        scale: true,
      }, {
        name: 'price',
        type: 'value',
        scale: true,
      }],
      series
    }
  }
}
</script>

<style scoped>
.echarts {
  width: 100%;
  height: 400px;
}
</style>
