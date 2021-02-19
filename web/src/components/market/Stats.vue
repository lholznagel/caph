<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <v-simple-table v-if="!busy">
      <template v-slot:default>
        <tbody>
          <tr>
            <th>Count orders</th>
            <td>
              <c-format-number :value="stats.order_count" />
            </td>
          </tr>
          <tr>
            <th>Total volume</th>
            <td>
              <c-format-number :value="stats.total_volume" />
            </td>
          </tr>
          <tr>
            <th>Highest {{ isBuyOrder ? 'buy' : 'sell' }} price</th>
            <td>
              <c-format-number :value="stats.highest_price" />
            </td>
          </tr>
          <tr>
            <th>Average {{ isBuyOrder ? 'buy' : 'sell' }} price</th>
            <td>
              <c-format-number :value="stats.average_price" />
            </td>
          </tr>
          <tr>
            <th>Lowest {{ isBuyOrder ? 'buy' : 'sell' }} price</th>
            <td>
              <c-format-number :value="stats.lowest_price" />
            </td>
          </tr>
        </tbody>
      </template>
    </v-simple-table>
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class ItemInfo extends Vue {
  @Prop(Number)
  public id!: number;
  @Prop(Boolean)
  public isBuyOrder!: boolean;

  public busy: boolean = false;
  public stats: IMarketStats = DEFAULT_MARKET_STATS;

  public async created() {
    this.busy = true;

    if (this.isBuyOrder) {
      this.stats = (await axios.get(`/api/market/${this.id}/stats/buy`)).data;
    } else {
      this.stats = (await axios.get(`/api/market/${this.id}/stats/sell`)).data;
    }

    this.busy = false;
  }
}

const DEFAULT_MARKET_STATS: IMarketStats = {
  average_price: 0,
  highest_price: 0,
  lowest_price: 0,
  order_count: 0,
  total_volume: 0
};

interface IMarketStats {
  average_price: number;
  highest_price: number;
  lowest_price: number;
  order_count: number;
  total_volume: number;
}
</script>
