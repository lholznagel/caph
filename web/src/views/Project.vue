<template>
  <n-card>
    <template #header>
      Project "{{ (project && project.name) || '' }}"
    </template>

    <n-skeleton text :repeat="5" v-if="busy" />

    <n-tabs line v-if="!busy">
      <n-tab-pane name="Overview">
        <project-overview :pid="$route.params.pid" />
      </n-tab-pane>

      <n-tab-pane name="Required Blueprints">
        <project-required-blueprints :pid="$route.params.pid" />
      </n-tab-pane>

      <n-tab-pane name="Raw Materials">
        <project-raw :pid="$route.params.pid" :pids="pids" />
      </n-tab-pane>

      <n-tab-pane name="Buildsteps">
        <project-buildstep :pid="$route.params.pid" />
      </n-tab-pane>

      <n-tab-pane name="Cost">
        TODO
      </n-tab-pane>

      <n-tab-pane name="Invention">
        TODO
      </n-tab-pane>

      <n-tab-pane name="Tree">
        <project-tree :pid="$route.params.pid" />
      </n-tab-pane>

      <n-tab-pane name="Settings">
        <project-settings />
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NSkeleton, NTabs, NTabPane, NTimeline, NTimelineItem } from 'naive-ui';
import { ProjectService, IProject } from '@/services/project';

import ProjectBuildstep from '@/components/project/Buildstep.vue';
import ProjectSettings from '@/views/ProjectSettings.vue';
import ProjectOverview from '@/components/project/Overview.vue';
import ProjectRaw from '@/components/project/Raw.vue';
import ProjectRequiredBlueprints from '@/components/project/Blueprints.vue';
import ProjectTree from '@/components/project/Tree.vue';

@Options({
  components: {
    NCard,
    NSkeleton,
    NTabs,
    NTabPane,

    NTimeline,
    NTimelineItem,

    ProjectBuildstep,
    ProjectOverview,
    ProjectRaw,
    ProjectRequiredBlueprints,
    ProjectSettings,
    ProjectTree,
  }
})
export default class Project extends Vue {
  public busy: boolean = false;
  public project: IProject | null = null;

  public pids: number[] = [];

  public async created() {
    this.busy = true;

    const pid: string = <string>this.$route.params.pid;
    this.project = await ProjectService.project(pid);
    this.pids = (await ProjectService.products(pid)).map(x => <number>x.type_id);

    this.busy = false;
  }
}
</script>
