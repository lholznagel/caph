<template>
  <n-card title="Assets">
    <n-skeleton text v-if="busy" :repeat="5" />

    <div v-if="!busy">
      <n-table v-if="entries.length > 0">
        <thead>
          <tr>
            <th width="48px"></th>
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="material in entries" :key="material.type_id">
            <td><item-icon :id="material.type_id" type="icon" /></td>
            <td><name-by-id :id="material.type_id" /></td>
          </tr>
        </tbody>
      </n-table>
      <!--n-input
        type="input"
        placeholder="#WithFilter"
        style="margin-bottom: 5px"
        clearable
        v-model="filterName"
        @input="handleNameFilter"
      />

      <n-data-table
        ref="table"
        :columns="columns"
        :data="entries"
        :pagination="pagination"
        @update:sorter="handleSortChange"
      /-->
    </div>
  </n-card>
</template>

<script lang="ts">
import { NButton, NButtonGroup, NCard, NDataTable, NInput, NSkeleton, NTag,
NTable } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';

import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';
import Owner from '@/components/Owner.vue';

import { CharacterService, ICharacterBlueprint } from '@/services/character';
import { NameService } from '@/services/name';

@Options({
  components: {
    NButton,
    NButtonGroup,
    NCard,
    NDataTable,
    NInput,
    NSkeleton,
    NTag,
    NTable,

    ItemIcon,
    NameById,
    Owner,
  }
})
export default class CharacterAsset extends Vue {
  public busy: boolean = false;
  public entries: IEntry[] = [];

  public filterName: string = '';

  public columns: any = [];
  public pagination: any = {
    pageSize: 10
  };


  public async created() {
    this.busy = true;

    this.columns = createColumns();

    this.entries = await CharacterService.assets();

    this.busy = false;
  }

  public handleNameFilter(val: string) {
    this.pagination.page = 1;
    const ref: any = this.$refs.table;
    ref.filter({
      name: [val]
    });
  }

  public handleSortChange(sorter: any) {
    this.columns.forEach((column: any) => {
      if (column.sortOrder === undefined) { return; }

      if (!sorter) {
        column.sortOrder = false;
      }
      if (column.key === sorter.columnKey) {
        column.sortOrder = sorter.order;
      } else {
        column.sortOrder = false;
      }
    })
  }
}

interface IEntry {
  quantity:            number;
  type_id:             number;
  name:                string;
  // unique per item, used as key
  item_id:             number;
  // custom value for the real count
  count:               number;
  material_efficiency: number;
  time_efficiency:     number;
  runs:                number;
  owners:              number[];
}

const createColumns = () => {
  return [{
    title: '',
    key:   'type_id',
    width: 48,
    render(row: IEntry) {
      return h(
        ItemIcon,
        {
          id: row.type_id,
          type: row.name.indexOf('Blueprint') === -1 ? 'icon' : 'bp',
          width: 32
        }
      )
    }
  }, {
    title: 'Name',
    key:   'name',
    width: 450,
    filter(value: string, row: IEntry) {
      return ~row.name.toLowerCase().indexOf(value.toLowerCase());
    },
    render(row: IEntry) {
      return h(
        NameById,
        { id: row.type_id },
      )
    }
  }];
}
</script>
