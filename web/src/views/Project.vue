<template>
  <n-card>
    <template #header>
      Project "{{ project.name }}"
    </template>

    <n-tabs line>
      <n-tab-pane name="Stored Materials">
        <project-stored-materials :pid="$route.params.id" />
      </n-tab-pane>

      <n-tab-pane name="Blueprints">
        <project-blueprints :pid="$route.params.id" />
      </n-tab-pane>

      <n-tab-pane name="Manufacture">
        <project-manufacture :pid="$route.params.id" />
      </n-tab-pane>

      <n-tab-pane name="Planetary Interaction">
        <label>A</label>
      </n-tab-pane>

      <n-tab-pane name="Invention">
        <label>A</label>
      </n-tab-pane>

      <n-tab-pane name="Cost">
        <project-cost :pid="$route.params.id" />
      </n-tab-pane>

      <n-tab-pane name="Raw Materials">
        <blueprint-raw-material :bpids="project.blueprints" />
      </n-tab-pane>

      <n-tab-pane name="Tree">
        <project-tree :pid="$route.params.id" />
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>

<script lang="ts">
import { NButton, NCard, NDataTable, NInput, NSkeleton, NTabs, NTabPane } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

import { IProject, ProjectService } from '@/services/project';

import BlueprintRawMaterial from '@/components/blueprint/RawMaterial.vue';
import ProjectCost from '@/components/project/Cost.vue';
import ProjectMaterial from '@/components/project/Material.vue';
import ProjectBlueprints from '@/components/project/Blueprints.vue';
import ProjectStoredMaterials from '@/components/project/Stored.vue';
import ProjectTree from '@/components/project/Tree.vue';
import ProjectManufacture from '@/components/project/Manufacture.vue';

@Options({
  components: {
    NButton,
    NCard,
    NDataTable,
    NInput,
    NSkeleton,
    NTabs,
    NTabPane,

    BlueprintRawMaterial,
    ProjectBlueprints,
    ProjectCost,
    ProjectManufacture,
    ProjectMaterial,
    ProjectStoredMaterials,
    ProjectTree,
  }
})
export default class ProjectOverview extends Vue {
  public project: IProject | {  } = {  };

  public async created() {
    const id: string = <string>this.$route.params.id;
    this.project = await ProjectService.project(id);
  }
}
</script>
