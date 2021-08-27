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

import { CharacterService, ICharacterBlueprint } from '@/services/character';
import { CorporationService, ICorporationBlueprint } from '@/services/corporation';
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

    const charBlueprints = await CharacterService.blueprints();
    //const corpBlueprints = await CorporationService.blueprints();
    const corpBlueprints = [];

    const blueprints = await this.joinCharCorpBp(charBlueprints, corpBlueprints);
    this.entries = await this.groupItems(blueprints);
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

  private async groupItems(items: IBlueprint[]): Promise<IBlueprint[]> {
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
        update.count += item.count;

        if (update.owners.indexOf(item.owners[0]) === -1) {
          update.owners.push(item.owners[0]);
        }

        map.set(name, update);
      } else {
        map.set(name, item);
      }
    }

    return Array
      .from(map.values())
      .filter(x => x.name)
      .sort((a: IBlueprint, b: IBlueprint) => a.name.localeCompare(b.name))
  }

  private async joinCharCorpBp(
    character: ICharacterBlueprint[],
    corporation: ICorporationBlueprint[]
  ): Promise<IBlueprint[]> {
    const blueprints: IBlueprint[] = [];

    for (let x of character) {
      blueprints.push({
        quantity:            x.quantity,
        type_id:             x.type_id,
        name:                (await NameService.resolve(x.type_id)),
        // TODO: replace with uuid
        id:                  x.item_id.toString(),
        count:               1,
        material_efficiency: x.material_efficiency,
        time_efficiency:     x.time_efficiency,
        runs:                x.runs,
        owners:              [x.user_id],
      });
    }

    for (let x of corporation) {
      blueprints.push({
        quantity:            x.quantity,
        type_id:             x.type_id,
        name:                (await NameService.resolve(x.type_id)),
        // TODO: replace with uuid
        id:                  x.id || '',
        count:               1,
        material_efficiency: x.material_efficiency,
        time_efficiency:     x.time_efficiency,
        runs:                x.runs,
        owners:              [x.corp_id],
      });
    }

    return blueprints;
  }
}

interface IBlueprint {
  quantity:            number;
  type_id:             number;
  name:                string;
  // unique per item, used as key
  id:                  string;
  // custom value for the real count
  count:               number;
  material_efficiency: number;
  time_efficiency:     number;
  runs:                number;
  owners:              number[];
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
    render(row: IBlueprint) {
      return h(
        NameById,
        { id: row.type_id },
      )
    }
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
                { onClick: () => openDetails(row.type_id, row.id) },
                { default: () => 'Details' }
              ),
              h(
                NButton,
                { onClick: () => newProject(row.type_id, row.id) },
                { default: () => 'New Project' }
              )]
          }
        }
      )
    }
  }];
}
</script>
