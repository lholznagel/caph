<template>
  <v-card class="mt-5">
    <v-card-title>
      Blueprints

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
        item-key="item_id"
        sort-by="name"
      >

        <template v-slot:item.type_id="{ item }">
          <c-item-icon :id="Number(item.type_id)" :type="'bp'"  v-if="item.quantity !== -2" />
          <c-item-icon :id="Number(item.type_id)" :type="'bpc'" v-if="item.quantity === -2" />
        </template>

        <template v-slot:item.runs="{ item }">
          {{ item.runs === -1 ? '∞' : item.runs }}
        </template>

        <template v-slot:item.type="{ item }">
          {{ item.quantity === -2 ? 'Copy' : 'Original' }}
        </template>

        <template v-slot:item.count="{ item }">
          <c-format-number :value="item.count" />
        </template>

        <template v-slot:item.blueprint_info="{ item }">
          <v-btn
            :to="{ name: 'Blueprint', params: { id: item.type_id, itemId: item.item_id } }"
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
    { text: 'Type', value: 'type' },
    { text: 'Count', value: 'count' },
    { text: 'Material Eff', value: 'material_efficiency' },
    { text: 'Time Eff', value: 'time_efficiency' },
    { text: 'Runs', value: 'runs' },
    { text: 'Info', value: 'blueprint_info', sortable: false, filterable: false, width: 32 }
  ];

  public async created() {
    this.$busy = true;
    const blueprints: IBlueprint[] = (await axios.get(`/api/character/blueprints`)).data;
    const itemName: IItem[] = [];
    for (const blueprint of blueprints) {
      itemName.push({
        name: (await IdNameCache.resolve(blueprint.type_id)),
        count: 1,
        ...blueprint
      });
    }

    this.items = this.groupItems(itemName);
    this.$busy = false;
  }

  // TODO: the server should do this like assets
  private groupItems(items: IItem[]): IItem[] {
    const map = new Map();
    for (const item of items) {
      // generate a unique key
      const name = `${item.type_id}_
                    ${item.time_efficiency}_
                    ${item.material_efficiency}_
                    ${item.runs}_
                    ${item.quantity === -2 ? '_copy' : '_orig'}`;

      if (map.get(name)) {
        const update = map.get(name);
        update.count += 1;
        map.set(name, update);
      } else {
        map.set(name, item);
      }
    }

    return Array.from(map.values());
  }
}

interface IBlueprint {
  item_id: number;
  material_efficiency: number;
  quantity: number;
  runs: number;
  time_efficiency: number;
  type_id: number;
}

interface IItem {
  quantity: number;
  type_id: number;
  name: string;
  // unique per item, used as key
  item_id: number;
  // custom value for the real count
  count: number;
  material_efficiency: number;
  time_efficiency: number;
  runs: number;
}
</script>
