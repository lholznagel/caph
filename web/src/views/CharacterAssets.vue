<template>
  <v-card class="mt-5">
    <v-card-title>
      All items

      <v-spacer></v-spacer>

      <v-text-field
        v-model="search"
        append-icon="mdi-magnify"
        label="#WithFilter"
        clearable
        hide-details
        single-line
      ></v-text-field>
    </v-card-title>

    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="items"
        :search="search"
        :loading="$busy"
        item-key="type_id"
        sort-by="name"
      >

        <template v-slot:item.type_id="{ item }">
          <c-item-icon :id="item.type_id" />
        </template>

        <template v-slot:item.quantity="{ item }">
          <c-format-number :value="item.quantity" />
        </template>

        <template v-slot:item.marketInfoLink="{ item }">
          <v-btn
            :to="{ name: 'MarketInfo', params: { id: item.type_id } }"
            color="primary"
            icon
          >
            <v-icon>mdi-open-in-new</v-icon>
          </v-btn>
        </template>


      </v-data-table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue } from 'vue-property-decorator';
import { IdNameCache } from '../services/resolve_names';

@Component
export default class MyItems extends Vue {
  public $busy = false;

  public items: IItem[] = [];
  public search: string = '';
  public headers        = [
    { text: '', value: 'type_id', sortable: false, filterable: false, width: 32 },
    { text: 'Name', value: 'name' },
    { text: 'Quantity', value: 'quantity', width: 128 },
    { text: 'Info', value: 'marketInfoLink', sortable: false, filterable: false, width: 32 }
  ];

  public async created() {
    this.$busy = true;
    const assets: IAsset[] = (await axios.get(`/api/character/assets`)).data;
    for (const asset of assets) {
      this.items.push({
        name: (await IdNameCache.resolve(asset.type_id)),
        ...asset
      });
    }

    this.$busy = false;
  }
}

interface IAsset {
  type_id: number;
  quantity: number;
}

interface IItem {
  quantity: number;
  type_id: number;
  name: string;
}
</script>
