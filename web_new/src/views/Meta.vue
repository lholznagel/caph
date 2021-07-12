<template>
  <n-card title="Meta">
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy">
      <thead>
        <tr>
          <th width="600px">Item</th>
          <th width="200px">Type</th>
          <th width="600px">Source</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in entries" :key="entry.type_id">
          <td><name-by-id :id="entry.type_id" /></td>
          <td>{{ entry.mtype }}</td>
          <td></td>
          <td><n-button type="error" ghost>Del</n-button></td>
        </tr>
      </tbody>
      <tfoot>
        <tr>
          <td>
            <n-select
              :options="items"
              v-model:value="meta.type_id"
              filterable
            /></td>
          <td><n-select :options="materialTypes" v-model:value="meta.mtype" /></td>
          <td>
            <n-select
              :options="items"
              v-model:value="meta.source"
              filterable
              multiple
            /></td>
          <td><n-button type="success" ghost @click="addEntry">Add</n-button></td>
        </tr>
      </tfoot>
    </n-table>

    {{ meta }}
  </n-card>
</template>

<script lang="ts">
import { NButton, NCard, NSelect, NSkeleton, NTable } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { ItemService } from '@/services/item';

import NameById from '@/components/NameById.vue';
import { IName, NameService } from '@/services/name';

@Options({
  components: {
    NButton,
    NCard,
    NSelect,
    NSkeleton,
    NTable,

    NameById,
  }
})
export default class CharacterBlueprint extends Vue {
  public busy: boolean = false;

  public entries: IItemMeta[] = [];
  public items: IItemSelect[] = [];

  public meta: any = {  };

  public materialTypes = [
    { label: 'Asteroid', value: 'ASTEROID' },
    { label: 'Ice',      value: 'ICE'      },
    { label: 'Moon',     value: 'MOON'     },
    { label: 'Pi 1',     value: 'PI1'      },
    { label: 'Pi 2',     value: 'PI2'      },
    { label: 'Pi 3',     value: 'PI3'      },
    { label: 'Pi 4',     value: 'PI4'      },
    { label: 'Reaction', value: 'REACTION' },
    { label: 'Salvage',  value: 'SALVAGE'  },
  ];

  public async created() {
    this.busy = true;

    const ids = await ItemService.keys();
    let names = await NameService.resolve_bulk(ids);

    for (let id of ids) {
      const name = (names.find((x: IName) => x.id === id) || { name: 'Unknown' }).name;
      if (name &&
        (
          name.indexOf('SKIN') > -1 ||
          name.indexOf('Men\'s') > -1 ||
          name.indexOf('Women\'s') > -1
        )
      ) {
        continue
      }

      this.items.push({
        label: (names.find((x: IName) => x.id === id) || { name: 'Unknown' }).name,
        value: id
      });
    }

    this.busy = false;
  }

  public addEntry() {
    this.entries.push(this.meta);
    Object.assign(this.meta, {});
  }
}

interface IItemSelect {
  label: string;
  value: number;
}

interface IItemMeta {
  type_id: number;
  mtype:   string;
  sources: IMaterialSource[];
}

interface IMaterialSource {
  type_id:  number;
  quantity: number;
}
</script>

