<template>
  <n-card title="Assets">
    <n-skeleton text v-if="busy" :repeat="5" />

    <div v-if="!busy">
      <filter-text
        :filters="filters"
        :options="filterOptions"
      />
      <filter-element
        style="margin-top: 5px"
        :filters="filters"
        :options="filterOptions"
      />

      <n-data-table
        style="margin-top: 5px"
        ref="table"
        :columns="columns"
        :data="entries"
        :pagination="pagination"
        @update:sorter="handleSortChange"
      />
    </div>
  </n-card>
</template>

<script lang="ts">
import { NButton, NButtonGroup, NCard, NDataTable, NInput, NSkeleton, NSpace,
NTag, NTable } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';

import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';
import Owner from '@/components/Owner.vue';

import { AssetService, IAsset } from '@/services/asset';
import { CharacterService } from '@/services/character';
import FilterText, { IFilterOption } from '@/components/Filter.vue';
import FilterElement from '@/components/FilterElement.vue';

@Options({
  components: {
    NButton,
    NButtonGroup,
    NCard,
    NDataTable,
    NInput,
    NSkeleton,
    NSpace,
    NTable,
    NTag,

    FilterText,
    FilterElement,
    ItemIcon,
    NameById,
    Owner,
  }
})
export default class CharacterAsset extends Vue {
  public busy: boolean = false;
  public entries: IAsset[] = [];

  public filters = {};
  public filterName: string = '';

  public columns: any = [];
  public pagination: any = {
    pageSize: 10
  };

  public filterOptions: { [key: string]: IFilterOption } = {};

  public async created() {
    this.busy = true;

    this.columns = createColumns();
    this.entries = await AssetService.assets(this.filters);

    let character_opts = (await CharacterService.ids()).map(x => x.toString());
    this.filterOptions = {
      name: {
        label: 'Name',
        element: 'TEXT'
      },
      owner: {
        label: 'Owner',
        element: 'OWNER',
        options: character_opts
      }
    };

    this.$watch(() => this.filters, async () => {
      this.entries = await AssetService.assets(this.filters);
    }, { deep: true });

    this.busy = false;
  }

  public mounted() {
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

const createColumns = () => {
  return [{
    title: '',
    key:   'type_id',
    width: 48,
    render(row: IAsset) {
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
    filter(value: string, row: IAsset) {
      return ~row.name.toLowerCase().indexOf(value.toLowerCase());
    },
    render(row: IAsset) {
      return h(
        NameById,
        { id: row.type_id },
      )
    }
  }, {
    title: 'Count',
    key:   'quantity',
    sortOrder: false,
    sorter(a: IAsset, b: IAsset) {
      return a.quantity - b.quantity;
    }
  }, {
    title: 'Owners',
    key:   'owner',
    width: 200,
    render(row: IAsset) {
      return h(
        NSpace,
        {},
        {
          default: () => row.owners
            .map(x => h(
              Owner,
              { id: x, withText: false }
            ))
        })
    }
  }];
}
</script>
