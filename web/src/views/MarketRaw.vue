<template>
  <v-card>
    <v-card-title>Raw market data</v-card-title>

    <v-card-text>
      <pre>{{ orders | pretty }}</pre>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class ItemInfo extends Vue {
  public orders: IOrders[] = [];

  public async created() {
    this.orders = (await axios.get(`/api/v1/market/raw`)).data[0];
  }
}

export interface IOrders {
    item_id: number;
    entries: IBucketEntry[];
}

export interface IBucketEntry {
    timestamp: number;
    entries: IOrderEntry[];
}

export interface IOrderEntry {
    volume_remain: number;
    order_id: number;
}
</script>
