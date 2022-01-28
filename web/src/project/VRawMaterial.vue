<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-tabs v-if="!busy" type="line">
        <template
          v-for="filter in filters"
          :key="filter.key"
        >
          <n-tab-pane
            v-if="project.required_materials(filter.key).length > 0"
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
                    <th width="48px"></th>
                    <th width="200px">Name</th>
                    <th width="150px">Required</th>
                    <th width="150px">Stored</th>
                    <th width="150px">Missing</th>
                    <th>Progress</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="entry in project.required_materials(filter.key)"
                    :key="entry.type_id"
                  >
                    <td><item-icon :id="entry.type_id" type="icon" /></td>
                    <td>{{ entry.name }}</td>
                    <td><format-number :value="entry.quantity" /></td>
                    <td><format-number :value="entry.stored || 0" /></td>
                    <td>
                      <format-number
                        :value="missing_materials(entry.quantity, entry.stored)"
                      />
                    </td>
                    <td>
                      <n-progress
                        type="line"
                        :percentage="calc_progress(entry)"
                        :indicator-placement="'inside'"
                        :status="calc_progress(entry) >= 100 ? 'success' : 'default'"
                      />
                    </td>
                  </tr>
                </tbody>
              </n-table>
            </n-card>

            <p-export
              v-model:show="show_export"
              :data="project.required_materials(filter.key)"
              :pid="$route.params.pid"
            />
          </n-tab-pane>
        </template>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NProgress, NSpace, NTable, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { ItemGroup } from '@/utils';
import { IRequiredMaterial } from './service';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

import PExport from '@/project/MExport.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NProgress,
    NSpace,
    NTable,
    NTabs,
    NTabPane,

    FormatNumber,
    ItemIcon,

    PExport,
    PHeader,
    WProject,
  }
})
export default class ProjectMaterialView extends Vue {
  public show_export: boolean = false;

  public filters: { label: string, key: ItemGroup[] }[] = [{
    label: 'All',
    key:   [ItemGroup.All]
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
  }];

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_raw_material`
    );
  }

  public calc_progress(x: IRequiredMaterial): number {
    return Math.ceil(x.stored / (x.quantity / 100) * 100) / 100 || 0;
  }

  public missing_materials(quantity: number, stored: number): number {
    if (stored > quantity) {
      return 0;
    } else {
      return quantity - stored;
    }
  }
}
</script>
