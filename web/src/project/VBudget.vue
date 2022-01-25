<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <p-header />

      <n-tabs
        type="line"
        v-if="!busy"
      >
        <n-tab-pane name="Overview">
          <n-card content-style="padding: 0">
            <n-space justify="end" style="margin: 10px">
              <n-button>
                Edit Entry
              </n-button>

              <n-button>
                Delete Entry
              </n-button>

              <n-button
                @click="$router.push({
                  name: 'projects_add_budget',
                  params: { pid: $route.params.pid }
                })"
                type="info"
              >
                Add cost
              </n-button>
            </n-space>

            <p-budget :pid="$route.params.pid" />
          </n-card>
        </n-tab-pane>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NSpace, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import PBudget from '@/project/CBudget.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NSpace,
    NTabPane,
    NTabs,

    PBudget,
    PHeader,
    WProject,
  }
})
export default class ProjectBudgetView extends Vue {
  public show_add_budget: boolean = false;

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_budget`
    );
  }
}
</script>
