<template>
  <div>
    <v-card>
      <v-card-title
        >Market info for "<c-name-by-id :id="Number($route.params.id)"
      />"</v-card-title>

      <v-row dense>
        <v-col cols="3">
          <v-card>
            <v-card-title>Item info</v-card-title>
            <c-item-info :id="Number($route.params.id)" />
          </v-card>
        </v-col>
        <v-col cols="3">
          <v-card elevation="5">
            <v-card-title>Reprocessed</v-card-title>
            <c-item-reprocessing :id="Number($route.params.id)" :quantity="quantity" />
          </v-card>
        </v-col>
        <v-col cols="3">
          <v-card elevation="5">
            <v-card-title>Buy stats</v-card-title>
            <c-market-stats :id="Number($route.params.id)" :is-buy-order="true" />
          </v-card>
        </v-col>
        <v-col cols="3">
          <v-card elevation="5">
            <v-card-title>Sell stats</v-card-title>
            <c-market-stats :id="Number($route.params.id)" :is-buy-order="false" />
          </v-card>
        </v-col>
      </v-row>
    </v-card>

    <v-card class="mt-5">
      <v-card-title>Buy / Sell</v-card-title>

      <v-expansion-panels accordion>
        <v-expansion-panel>
          <v-expansion-panel-header>Highest Buy</v-expansion-panel-header>
          <v-expansion-panel-content>
            <c-market-orders
              type="high"
              :id=$route.params.id
              :is-buy-order="true"
            />
          </v-expansion-panel-content>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-header>Highest Sell</v-expansion-panel-header>
          <v-expansion-panel-content>
            <c-market-orders
              type="high"
              :id="$route.params.id"
              :is-buy-order="false"
            />
          </v-expansion-panel-content>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-header>Lowest Buy</v-expansion-panel-header>
          <v-expansion-panel-content>
            <c-market-orders
              type="low"
              :id="$route.params.id"
              :is-buy-order="true"
            />
          </v-expansion-panel-content>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-header>Lowest Sell</v-expansion-panel-header>
          <v-expansion-panel-content>
            <c-market-orders
              type="low"
              :id="$route.params.id"
              :is-buy-order="false"
            />
          </v-expansion-panel-content>
        </v-expansion-panel>
      </v-expansion-panels>
    </v-card>
  </div>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class ItemInfo extends Vue {
  public quantity: number = 0;

  public async created() {
    this.quantity = (await axios.get(`/api/v1/items/my/${this.$route.params.id}`)).data.quantity;
  }
}
</script>
