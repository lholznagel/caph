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
            v-if="project.has_blueprint(filter.key)"
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

              <n-table v-if="project.required_blueprints(filter.key).length > 0">
                <thead>
                  <tr>
                    <th width="34"></th>
                    <th width="500">Name</th>
                    <th width="100">Iters</th>
                    <th width="100">Runs</th>
                    <th width="100">Material</th>
                    <th width="100">Time</th>
                    <th width="200">Status</th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="entry in project.required_blueprints(filter.key)"
                    :key="entry.type_id()"
                  >
                    <td>
                      <item-icon
                        :id="entry.type_id()"
                        :type="entry.typ()"
                        :width="32"
                      />
                    </td>
                    <td>{{ entry.name() }}</td>
                    <td>{{ entry.iters() }}</td>
                    <td>{{ entry.runs() ? entry.runs() : '-' }}</td>
                    <td>{{ entry.material_eff() ? entry.material_eff() : '-' }}</td>
                    <td>{{ entry.time_eff() ? entry.time_eff() : '-' }}</td>
                    <td>
                      <n-tag v-if="entry.is_stored()" type="success">Stored</n-tag>
                      <n-tag v-if="!entry.is_stored()" type="error">Missing</n-tag>
                    </td>
                  </tr>
                </tbody>
              </n-table>
            </n-card>

            <p-export
              v-model:show="show_export"
              :data="blueprints(filter.key)"
              :data-fields="['name']"
              :data-fields-csv="['type_id', 'name']"
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
import { NButton, NCard, NSpace, NTable, NTabs, NTabPane, NTag } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import ItemIcon from '@/components/ItemIcon.vue';

import PExport from '@/project/MExport.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';
import { IBlueprint, Project, ProjectId, ProjectService2 } from './service';

@Options({
  components: {
    NButton,
    NCard,
    NSpace,
    NTable,
    NTabPane,
    NTabs,
    NTag,

    ItemIcon,

    PExport,
    PHeader,
    WProject,
  }
})
export default class ProjectBlueprintView extends Vue {
  public show_export: boolean = false;

  public project: Project = <Project>{  };

  public filters: { label: string, key: string }[] = [{
    label: 'All',
    key:   'ALL'
  }, {
    label: 'Blueprint',
    key:   'BLUEPRINTS'
  }, {
    label: 'Reaction',
    key:   'REACTIONS'
  }];

  public async created() {
    this.project = await ProjectService2.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();

    events.$emit(
      PROJECT_ROUTE,
      `projects_blueprint`
    );
  }

  public blueprints(filter: 'ALL' | 'BLUEPRINTS' | 'REACTIONS'): IBlueprint[] {
    return this.project
      .required_blueprints(filter)
      .map(x => x.blueprint);
  }
}
</script>
