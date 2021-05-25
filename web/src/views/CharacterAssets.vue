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
        :items="assets"
        :loading="busy"
        :search="search"
        :single-expand="true"
        item-key="type_id"
        sort-by="name"
        show-expand
      >

        <template v-slot:item.type_id="{ item }">
          <c-item-icon :id="item.type_id" :type="item.icon" />
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

        <template v-slot:expanded-item="{ headers, item }">
          <td :colspan="headers.length">
            <c-character-asset-location
              :asset="item"
              :assets="assets"
              :names="names" />
          </td>
        </template>
      </v-data-table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { CharacterService, IAsset, IAssetName } from '@/services/character';

@Component
export default class CharacterAssets extends Vue {
  public busy = false;

  public assets: IAsset[]    = [];
  public names: IAssetName[] = [];

  public search: string = '';
  public headers        = [
    { text: '', value: 'type_id', sortable: false, filterable: false, width: 32 },
    { text: 'Name', value: 'name' },
    { text: 'Quantity', value: 'quantity', width: 128 },
    { text: 'Info', value: 'marketInfoLink', sortable: false, filterable: false, width: 32 }
  ];

  public async created() {
    this.busy = true;

    this.assets = await CharacterService.assets();
    this.names  = await CharacterService.asset_names(this.assets);

    this.busy = false;
  }
}
</script>
