<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header v-if="!busy" />

      <n-tabs
        type="line"
        v-if="!busy"
      >
        <n-tab-pane name="Stored Products">
          <n-card content-style="padding: 0" v-if="!busy">
            <p-material
              :materials="project.stored_products()"
              :pid="$route.params.pid"
            />
          </n-card>
        </n-tab-pane>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NSkeleton, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import PMaterial from '@/project/CMaterial.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NCard,
    NSkeleton,
    NTabPane,
    NTabs,

    PMaterial,
    PHeader,
    WProject,
  }
})
export default class ProjectOverviewView extends Vue {
  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_overview`
    );
  }
}
</script>
