<template>
  <div v-if="routes.length > 0">{{ routes[0].systems.length - 1 }}</div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';
import { HOME_SYSTEM } from '../main';

@Component
export default class Route extends Vue {
  @Prop(Number)
  public destination!: number;

  public routes: IRoute[] = [];

  public async created() {
    this.routes = (await axios.get(`/api/v1/regions/route?origin=${HOME_SYSTEM}&destination=${this.destination}`))
      .data
      .filter(x => x.flag === 'secure');
  }
}

interface IRoute {
  origin: number;
  destination: number;
  systems: number[];
  flag: string;
}
</script>
