<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <v-simple-table v-if="!busy && marketData.length > 0">
      <template v-slot:default>
        <thead>
          <tr>
            <th class="text-left">Volume</th>
            <th class="text-left">Price</th>
            <th class="text-left">Security</th>
            <th class="text-left">System</th>
            <th class="text-left">Region</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in marketData" :key="item.order_id">
            <td><c-format-number :value="item.volume_remain || 0" /></td>
            <td><c-format-number :value="item.price || 0" /></td>
            <td>{{ item.security }}</td>
            <td><c-name-by-id :id="item.system_id"/></td>
            <td><c-name-by-id :id="item.region_id"/></td>
          </tr>
        </tbody>
      </template>
    </v-simple-table>

    <v-alert color="blue-grey" type="info" v-if="!busy && marketData.length === 0">
      No orders
    </v-alert>
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class MarketOrders extends Vue {
  @Prop(String)
  public id!: string;
  @Prop(String)
  public type!: string; // low, high
  @Prop(Boolean)
  public isBuyOrder!: boolean;

  public busy: boolean = false;
  public marketData: IMarketResult[] = [];

  public async created() {
    this.busy = true;

    this.marketData = (await axios.post(`/api/market/${this.id}/orders`, {
      is_buy_order: this.isBuyOrder,
      count: 5,
      sort: this.type === 'high' ? 'DESC' : 'ASC'
    })).data;

    this.busy = false;
  }
}

interface IMarketResult {
  is_buy_order: boolean;
  price: number;
  region_id: number;
  order_id: number;
  security: number;
  system_id: number;
  timestamp: number;
  volume_remain: number;
}
</script>
