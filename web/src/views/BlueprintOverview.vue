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
import { NButton, NButtonGroup, NCard, NDataTable, NInput, NSkeleton, NSpace,
NTag } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h, VNode } from 'vue';

import FilterText, { IFilterOption } from '@/components/Filter.vue';
import FilterElement from '@/components/FilterElement.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';
import Owner from '@/components/Owner.vue';

import { AssetService, IAccountBlueprint } from '@/services/asset';
import { CharacterService } from '@/services/character';

@Options({
  components: {
    NButton,
    NButtonGroup,
    NCard,
    NDataTable,
    NInput,
    NSkeleton,
    NSpace,
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
  public entries: IAccountBlueprint[] = [];

  public filters = {};
  public filterName: string = '';

  public columns: any = [];
  public pagination: any = {
    pageSize: 10
  };

  public filterOptions: { [key: string]: IFilterOption } = {};

  public async created() {
    this.busy = true;
    this.columns = createColumns(this.openDetails, this.newProject);

    this.entries = await AssetService.all_blueprints(this.filters);

    let character_opts = (await CharacterService.ids()).map(x => x.toString());
    this.filterOptions = {
      name: {
        label: 'Name',
      },
      owner: {
        label: 'Owner',
        options: character_opts,
        template: (val: string): VNode => {
          return h(
            Owner,
            { id: Number(val), withText: true }
           )
        }
      },
      material_eff: {
        label: 'Material Efficiency',
      },
      time_eff: {
        label: 'Time Efficiency',
      }
    };

    this.$watch(() => this.filters, async () => {
      this.entries = await AssetService.all_blueprints(this.filters);
    }, { deep: true });

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

  public openDetails(tid: number, iid: number[]) {
    this.$router.push({
      name: 'blueprint',
      params: { tid, iid: iid[0] }
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
    render(row: IAccountBlueprint) {
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
    filter(value: string, row: IAccountBlueprint) {
      return ~row.name.toLowerCase().indexOf(value.toLowerCase());
    },
  }, {
    title: 'Type',
    key:   'type',
    width: 100,
    render(row: IAccountBlueprint) {
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
    sorter(a: IAccountBlueprint, b: IAccountBlueprint) {
      return a.count - b.count;
    }
  }, {
    title: 'Material Eff',
    key:   'material_efficiency',
  }, {
    title: 'Time Eff',
    key:   'time_efficiency',
  },  {
    title: 'Runs',
    key:   'runs',
  },{
    title: 'Owners',
    key:   'owner',
    width: 200,
    render(row: IAccountBlueprint) {
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
  }, {
    title: '',
    key:   'btn',
    render(row: IAccountBlueprint) {
      return h(
        NButtonGroup,
        {  },
        {
          default: () => {
            return [
              h(
                NButton,
                { onClick: () => openDetails(row.type_id, row.item_ids) },
                { default: () => 'Details' }
              ),
              h(
                NButton,
                { onClick: () => newProject(row.type_id, row.item_ids) },
                { default: () => 'New Project' }
              )]
          }
        }
      )
    }
  }];
}
</script>
