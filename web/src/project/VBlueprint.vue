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
              <n-table v-if="project.required_blueprints().length > 0">
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
          </n-tab-pane>
        </template>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NTable, NTabs, NTabPane, NTag } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import ItemIcon from '@/components/ItemIcon.vue';

import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NCard,
    NTable,
    NTabPane,
    NTabs,
    NTag,

    ItemIcon,

    PHeader,
    WProject,
  }
})
export default class ProjectBlueprintView extends Vue {
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
    events.$emit(
      PROJECT_ROUTE,
      `projects_blueprint`
    );
  }
}
</script>
