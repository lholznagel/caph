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

                <n-button
                  @click="$router.push({
                    name: 'projects_storage_add_material',
                    params: { pid: $route.params.pid }
                  })"
                  type="info"
                >
                  Add materials
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
                    :key="entry.ptype_id"
                  >
                    <td><item-icon :id="entry.ptype_id" type="icon" /></td>
                    <td>{{ entry.name }}</td>
                    <td><format-number :value="entry.products" /></td>
                    <td><format-number :value="stored_products(entry.ptype_id)" /></td>
                    <td>
                      <format-number
                        :value="missing_materials(entry.ptype_id, entry.products)"
                      />
                    </td>
                    <td>
                      <n-progress
                        type="line"
                        :percentage="calc_progress(entry.ptype_id, entry.products)"
                        :indicator-placement="'inside'"
                        :status="calc_progress(entry.ptype_id, entry.products) >= 100 ? 'success' : 'default'"
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
import { NButton, NCard, NProgress, NSpace, NTable, NTabs, NTabPane, NScrollbar } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { ItemGroup } from '@/utils';
import { Service, IStorageEntry } from '@/project/service';
import { ProjectId } from '@/project/project';

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
    NScrollbar,

    FormatNumber,
    ItemIcon,

    PExport,
    PHeader,
    WProject,
  }
})
export default class ProjectMaterialView extends Vue {
  public show_export: boolean = false;
  public stored: IStorageEntry[] = [];

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
      this.$route.name
    );

    this.stored = await Service.stored(<ProjectId>this.$route.params.pid);
  }

  public stored_products(type_id: number): number {
    let stored = this.stored.find(x => x.type_id === type_id);
    return stored ? stored.quantity : 0;
  }

  public calc_progress(type_id: number, quantity: number): number {
    let stored = this.stored_products(type_id);
    return Math.ceil(stored / (quantity / 100) * 100) / 100 || 0;
  }

  public missing_materials(type_id: number, quantity: number): number {
    let stored = this.stored_products(type_id);
    if (stored > quantity) {
      return 0;
    } else {
      return quantity - stored;
    }
  }
}
</script>
