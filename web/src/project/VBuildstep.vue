<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <p-header />

      <n-tabs v-if="!busy" type="line">
        <n-tab-pane name="Manufacture / Reaction">
          <n-card content-style="padding: 0">
            <p-buildstep
              :pid="$route.params.pid"
            />
          </n-card>
        </n-tab-pane>

        <n-tab-pane name="Invention">
          <n-card content-style="padding: 0">
            <p-invention
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
import { NCard, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import PBuildstep from '@/project/CBuildstep.vue';
import PInvention from '@/project/CBuildstepInvention.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NCard,
    NTabPane,
    NTabs,

    PBuildstep,
    PInvention,
    PHeader,
    WProject,
  }
})
export default class ProjectBuildstepView extends Vue {
  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_buildstep`
    );
  }
}
</script>
