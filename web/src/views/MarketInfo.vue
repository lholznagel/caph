<template>
  <div>
    <v-card>
      <v-card-title
        >Market info for "<c-name-by-id :id="Number($route.params.id)"/>"</v-card-title>

      <v-row dense>
        <v-col cols="3" style="flex-direction:column">
          <v-card elevation="5" style="height: 100%">
            <v-card-title>Item info</v-card-title>
            <c-item-info :id="Number($route.params.id)" />
          </v-card>
        </v-col>
        <v-col cols="3">
          <v-card elevation="5" style="height: 100%">
            <v-card-title>Reprocessing</v-card-title>
            <c-item-reprocessing :id="Number($route.params.id)" />
          </v-card>
        </v-col>
        <v-col cols="3">
          <v-card elevation="5" style="height: 100%">
            <v-card-title>Buy stats</v-card-title>
            <c-market-stats :id="Number($route.params.id)" :is-buy-order="true" />
          </v-card>
        </v-col>
        <v-col cols="3">
          <v-card elevation="5" style="height: 100%">
            <v-card-title>Sell stats</v-card-title>
            <c-market-stats :id="Number($route.params.id)" :is-buy-order="false" />
          </v-card>
        </v-col>
      </v-row>
    </v-card>

    <v-card class="mt-5">
      <v-card-title>Latest</v-card-title>

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

    <v-card class="mt-5">
      <v-card-title>Historic Price</v-card-title>

      <v-expansion-panels accordion>
        <v-expansion-panel>
          <v-expansion-panel-header>Buy</v-expansion-panel-header>
          <v-expansion-panel-content>
            <c-market-charts></c-market-charts>
          </v-expansion-panel-content>
        </v-expansion-panel>
        <v-expansion-panel>
          <v-expansion-panel-header>Sell</v-expansion-panel-header>
          <v-expansion-panel-content>
            <c-market-charts></c-market-charts>
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
  // TODO: what?
  public quantity: number = 0;
}
</script>
