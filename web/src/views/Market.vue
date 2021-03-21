<template>
  <v-card class="mt-5">
    <v-card-title>
      Market

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
        item-key="img"
        sort-by="name"
      >

        <template v-slot:item.img="{ item }">
          <c-item-icon :id="item.itemId" />
        </template>

        <template v-slot:item.marketInfoLink="{ item }">
          <v-btn
            :to="{ name: 'MarketInfo', params: { id: item.itemId } }"
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
import { IdNameCache, IIdName } from '../services/resolve_names';

@Component
export default class MyItems extends Vue {
  public $busy = false;

  public items: IEntry[] = [];
  public search: string  = '';
  public headers         = [
    { text: '', value: 'img', sortable: false, filterable: false, width: 32 },
    { text: 'Name', value: 'name' },
    { text: 'Info', value: 'marketInfoLink', sortable: false, filterable: false, width: 32 },
  ];

  public async created() {
    this.$busy = true;
    const ids = (await axios.get('/api/market/items')).data;
    this.items = (await IdNameCache.resolve_bulk(ids)).map(x => {
      return {
        itemId: x.id,
        name: x.name,
      };
    });
    this.$busy = false;
  }
}

interface IEntry {
  itemId: number;
  name: string;
}
</script>
