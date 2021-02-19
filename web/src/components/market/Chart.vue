<template>
  <apexchart height="400" type="line" :options="options" :series="series"></apexchart>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class MyItems extends Vue {
  public options = {
    chart: {
      id: 'vuechart-example'
    },
    xaxis: {
      categories: [1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998]
    },
    markers: {
      size: 1,
    }
  };
  public series = [{
    name: 'series-1',
    data: [30, 40, 45, 50, 49, 60, 70, 91]
  }]
}
</script>
