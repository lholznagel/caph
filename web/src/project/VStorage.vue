<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <p-header />

      <n-tabs v-if="!busy && entries.length > 0" type="line">
        <template
          v-for="filter in filters"
          :key="filter.key"
        >
          <n-tab-pane
            v-if="filtered(filter.key).length > 0"
            :name="filter.label"
          >
            <n-card content-style="padding: 0">
              <n-space justify="end" style="margin: 10px">
                <n-button
                  @click="show_export = true"
                >
                  Export
                </n-button>
              </n-space>

              <n-table>
                <thead>
                  <tr>
                    <th width="32px"></th>
                    <th width="400px">Name</th>
                    <th width="200px">Quantity</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="entry in filtered(filter.key)"
                    :key="entry.type_id"
                  >
                    <td><eve-icon :id="entry.type_id" type="icon" /></td>
                    <td>{{ entry.name }}</td>
                    <td><format-number :value="entry.quantity" /></td>
                  </tr>
                </tbody>
              </n-table>
            </n-card>

            <p-export
              v-model:show="show_export"
              :data="filtered(filter.key)"
              :pid="$route.params.pid"
            />
          </n-tab-pane>
        </template>
      </n-tabs>

      <n-card
        content-style="padding: 0"
        v-if="!busy && entries.length === 0"
      >
        <n-space justify="end" style="margin: 10px">
          <n-button
            @click="$router.push({
              name: 'projects_storage_add',
              params: { pid: $route.params.pid }
            })"
            type="info"
          >
            Add material
          </n-button>
        </n-space>
        <n-empty description="Nothing stored" />
      </n-card>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NEmpty, NSpace, NTable, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { ItemGroup } from '@/utils';
import { Service, IStorageEntry} from '@/project/service';
import { ProjectId } from '@/project/project';

import FormatNumber from '@/components/FormatNumber.vue';
import EveIcon from '@/components/EveIcon.vue';

import PExport from '@/project/MExport.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/wrapper/Project.vue';

@Options({
  components: {
    NButton,
    NCard,
    NEmpty,
    NSpace,
    NTable,
    NTabs,
    NTabPane,

    FormatNumber,
    EveIcon,

    PExport,
    PHeader,
    WProject,
  }
})
export default class ProjectStorageView extends Vue {
  public show_export: boolean = false;
  public entries: IStorageEntry[] = [];

  public filters: { label: string, key: ItemGroup[] }[] = [{
    label: 'All',
    key:   [ItemGroup.All]
  }, {
    label: 'Blueprints',
    key:   [ItemGroup.Blueprints]
  }, {
    label: 'Minerals',
    key:   [ItemGroup.Minerals]
  }, {
    label: 'Ice',
    key:   [ItemGroup.Ice]
  }, {
    label: 'Moon',
    key:   [ItemGroup.Moon]
  }, {
    label: 'Gas',
    key:   [ItemGroup.Gas]
  }, {
    label: 'Salvage',
    key:   [ItemGroup.Salvage]
  }, {
    label: 'PI0',
    key:   [ItemGroup.PI0Solid, ItemGroup.PI0Liquid, ItemGroup.PI0Organic]
  }, {
    label: 'PI1',
    key:   [ItemGroup.PI1]
  }, {
    label: 'PI2',
    key:   [ItemGroup.PI2]
  }, {
    label: 'PI3',
    key:   [ItemGroup.PI3]
  }, {
    label: 'PI4',
    key:   [ItemGroup.PI4]
  }, {
    label: 'Misc',
    key:   [ItemGroup.NotCovered]
  }];

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );

    this.entries = await Service.stored(<ProjectId>this.$route.params.pid);
  }

  public filtered(filter: ItemGroup[]): IStorageEntry[] {
    if (filter.length === 0 || filter[0] === ItemGroup.All) {
      return this.entries;
    } else if (filter.length > 0 && filter[0] === ItemGroup.Blueprints) {
      return this.entries.filter(x => x.name.indexOf('Blueprint') > -1)
    } else if(filter.length > 0 && filter[0] === ItemGroup.NotCovered) {
      let all = this.all_groups();
      return this.entries.filter(x => all.indexOf(x.group_id) === -1);
    } else {
      return this.entries.filter(x => filter.indexOf(x.group_id) > -1);
    }
  }

  private all_groups(): ItemGroup[] {
    return [
      ItemGroup.Blueprints,
      ItemGroup.Minerals,
      ItemGroup.Ice,
      ItemGroup.Moon,
      ItemGroup.Gas,
      ItemGroup.Salvage,
      ItemGroup.PI0Solid,
      ItemGroup.PI0Liquid,
      ItemGroup.PI0Organic,
      ItemGroup.PI1,
      ItemGroup.PI2,
      ItemGroup.PI3,
      ItemGroup.PI4,
    ];
  }
}
</script>
