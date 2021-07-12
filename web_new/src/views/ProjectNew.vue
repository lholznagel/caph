<template>
  <n-card title="New project">
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-space vertical v-if="!busy">
      <n-form
        :model="project"
      >
        <n-form-item label="Project name" path="name">
          <n-input v-model:value="project.name" placeholder="Name of the project" />
        </n-form-item>

        <n-form-item label="Production system" path="system">
          <n-select
            :options="systemOptions"
            v-model:value="project.system"
            placeholder="Select system"
          />
        </n-form-item>

        <n-form-item label="Project chest" path="chest">
          <n-select
            :options="projectChestOptions"
            v-model:value="project.chest"
            placeholder="Select project chest"
          />
        </n-form-item>

        <n-form-item label="Add blueprints">
          <n-dynamic-input
            v-model:value="project.blueprints"
            :on-create="addBlueprint"
            :min="1"
            #="{ value }"
          >
            <div style="width: 100%;">
              <div style="display: flex; align-items: center;">
                <n-select
                  :options="blueprintOptions"
                  v-model:value="value.bpid"
                  filterable
                  placeholder="Select Blueprint"
                />
                <n-input-number style="margin-left: 5px" v-model:value="value.runs" />
              </div>
            </div>
          </n-dynamic-input>
        </n-form-item>

        <n-button @click="createProject">Create Project</n-button>
      </n-form>
    </n-space>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NForm, NFormItem, NInput, NInputNumber,
  NSelect, NSkeleton, NSpace } from 'naive-ui';

import { BlueprintService, IBlueprint, IBlueprintOption } from '@/services/blueprint';
import { ProjectService, IProjectChestOption } from '@/services/project';
import { IndustryService } from '@/services/industry';

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NForm,
    NFormItem,
    NInput,
    NInputNumber,
    NSelect,
    NSkeleton,
    NSpace,
  }
})
export default class ProjectNewBlueprint extends Vue {
  public busy: boolean = false;

  public bid: any = undefined;
  public iid: any = undefined;

  public bp:  {} | IBlueprint = {};
  public cbp: any = {};

  public project: any = {
    blueprints: [{
      bpid: undefined,
      runs: 1
    }],
    chest: 1036590986685,
    system: 30003978,
  };
  public blueprintOptions: IBlueprintOption[]       = [];
  public projectChestOptions: IProjectChestOption[] = [];
  public systemOptions: any[]                       = [];

  public addBlueprint() {
    return {
      bp:   undefined,
      runs: 1,
    }
  }

  public async created() {
    this.busy = true;

    this.blueprintOptions    = await BlueprintService.blueprints();
    this.projectChestOptions = await ProjectService.projectChests();
    this.systemOptions       = await IndustryService.stations();

    this.busy = false;
  }

  public async createProject() {
    await ProjectService.projectNew(this.project);
  }
}
</script>
