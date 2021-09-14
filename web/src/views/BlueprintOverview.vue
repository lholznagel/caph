<template>
  <n-card title="Blueprints">
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
import { NButton, NButtonGroup, NCard, NDataTable, NInput, NSkeleton, NTag } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';

import FilterText, { IFilterOption } from '@/components/Filter.vue';
import FilterElement from '@/components/FilterElement.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';
import Owner from '@/components/Owner.vue';

import { AssetService, IBlueprint } from '@/services/asset';

@Options({
  components: {
    NButton,
    NButtonGroup,
    NCard,
    NDataTable,
    NInput,
    NSkeleton,
    NTag,

    FilterText,
    FilterElement,
    ItemIcon,
    NameById,
    Owner,
  }
})
export default class CharacterBlueprint extends Vue {
  public busy: boolean = false;
  public entries: IBlueprint[] = [];

  public filters = {};
  public filterName: string = '';

  public columns: any = [];
  public pagination: any = {
    pageSize: 10
  };

  public filterOptions: { [key: string]: IFilterOption } = {
    name: {
      label: 'Name',
      element: 'TEXT'
    },
    owner: {
      label: 'Owner',
      element: 'OWNER',
      options: ['2117441999', '692480993']
    }
  };

  public async created() {
    this.busy = true;
    this.columns = createColumns(this.openDetails, this.newProject);

    this.entries = await AssetService.blueprints();
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

  public openDetails(bid: number, iid: number) {
    this.$router.push({
      name: 'blueprint',
      params: { bid, iid }
    });
  }

  public newProject(bid: number, iid: number) {
    this.$router.push({
      name: 'project_new_blueprint',
      params: { bid, iid }
    });
  }
}

const createColumns = (openDetails: any, newProject: any) => {
  return [{
    title: '',
    key:   'type_id',
    width: 48,
    render(row: IBlueprint) {
      return h(
        ItemIcon,
        {
          id: row.type_id,
          type: row.quantity === -2 ? 'bpc' : 'bp',
          width: 32
        }
      )
    }
  }, {
    title: 'Name',
    key:   'name',
    width: 450,
    filter(value: string, row: IBlueprint) {
      return ~row.name.toLowerCase().indexOf(value.toLowerCase());
    },
  }, {
    title: 'Type',
    key:   'type',
    width: 100,
    render(row: IBlueprint) {
      return h(
        NTag,
        { type: row.quantity === -2 ? 'warning' : 'info' },
        { default: () => row.quantity === -2 ? 'Copy' : 'Original' }
      )
    }
  }, {
    title: 'Count',
    key:   'count',
    sortOrder: false,
    sorter(a: IBlueprint, b: IBlueprint) {
      return a.count - b.count;
    }
  }, {
    title: 'Material Eff',
    key:   'material_efficiency',
  }, {
    title: 'Time Eff',
    key:   'time_efficiency',
  }, {
    title: 'Owners',
    key:   'owner',
    width: 200,
    render(row: IBlueprint) {
      return h(
        Owner,
        { ids: row.owners, withText: false }
      )
    }
  }, {
    title: '',
    key:   'btn',
    render(row: IBlueprint) {
      return h(
        NButtonGroup,
        {  },
        {
          default: () => {
            return [
              h(
                NButton,
                { onClick: () => openDetails(row.type_id, row.item_id) },
                { default: () => 'Details' }
              ),
              h(
                NButton,
                { onClick: () => newProject(row.type_id, row.item_id) },
                { default: () => 'New Project' }
              )]
          }
        }
      )
    }
  }];
}
</script>
