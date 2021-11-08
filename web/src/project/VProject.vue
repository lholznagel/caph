<template>
  <div>
    <n-page-header v-if="!busy">
      <template #title>
        <h1>
          Project "{{ project.info.name }}"
        </h1>
      </template>

      <template #extra>
        <n-button
          @click="refresh"
          :loading="busy"
          :disabled="busy"
        >
          Refresh
        </n-button>
      </template>
    </n-page-header>

    <n-tabs line v-if="!busy">
      <n-tab-pane name="Overview">
        <n-card content-style="padding: 0">
          <n-tabs
            type="line"
            :tabs-padding="20"
          >
            <n-tab-pane name="Stored Products">
              <p-material :materials="stored_materials" />
            </n-tab-pane>
          </n-tabs>
        </n-card>
      </n-tab-pane>

      <n-tab-pane name="Market">
        <n-card content-style="padding: 0">
          <n-tabs
            type="line"
            :tabs-padding="20"
          >
            <n-tab-pane name="Material buy">
              <n-space vertical>
                <n-space justify="end">
                  <system-selector
                    v-model:value="sid"
                  />
                </n-space>

                <p-market :pid="$route.params.pid" :sid="sid" />
              </n-space>
            </n-tab-pane>

            <n-tab-pane name="Material sell">
              <n-space vertical>
                <n-space justify="end">
                  <system-selector
                    v-model:value="sid"
                  />
                </n-space>

                <p-market :pid="$route.params.pid" :sid="sid" is-sell />
              </n-space>
            </n-tab-pane>
          </n-tabs>
        </n-card>
      </n-tab-pane>

      <n-tab-pane name="Budget">
        <n-card content-style="padding: 0">
          <n-tabs
            type="line"
            :tabs-padding="20"
          >
            <n-tab-pane name="Overview">
              <n-space vertical>
                <n-space justify="end">
                  <n-button
                    @click="show_add_budget = true;" ghost type="primary">
                    Add cost
                  </n-button>
                </n-space>

                <p-budget :pid="$route.params.pid" />
                <p-add-cost
                  v-model:show="show_add_budget"
                  :pid="$route.params.pid"
                  :config="{}"
                  :is-edit="false"
                />
              </n-space>
            </n-tab-pane>
          </n-tabs>
        </n-card>
      </n-tab-pane>

      <n-tab-pane name="Material">
        <n-card content-style="padding: 0">
          <n-tabs
            type="line"
            :tabs-padding="20"
          >
            <n-tab-pane name="Raw">
              <p-raw :pid="$route.params.pid" />
            </n-tab-pane>

            <n-tab-pane name="Blueprints">
              <p-blueprint :pid="$route.params.pid" />
            </n-tab-pane>
          </n-tabs>
        </n-card>
      </n-tab-pane>

      <n-tab-pane name="Buildsteps">
        <n-card content-style="padding: 0">
          <n-tabs
            type="line"
            :tabs-padding="20"
          >
            <n-tab-pane name="Manufacturing">
              <p-buildstep :pid="$route.params.pid" :activity="'manufacture'" />
            </n-tab-pane>

            <n-tab-pane name="Reactions">
              <p-buildstep :pid="$route.params.pid" :activity="'reaction'" />
            </n-tab-pane>

            <n-tab-pane name="Inventions">
            </n-tab-pane>
          </n-tabs>
        </n-card>
      </n-tab-pane>

      <n-tab-pane name="Settings">
        <n-card content-style="padding: 0">
          <n-tabs
            type="line"
            :tabs-padding="20"
          >
            <n-tab-pane name="General">
            </n-tab-pane>
          </n-tabs>
        </n-card>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NPageHeader, NSpace, NSkeleton, NTabs, NTabPane } from 'naive-ui';
import { Project, ProjectId, ProjectService } from '@/project/service';
import { IMaterial } from '@/project/CMaterial.vue';
import { SystemId } from '@/utils';
import { events } from '@/main';
import { BUDGET_CHANGE } from '@/event_bus';

import PAddCost from '@/project/MAddCost.vue';
import PBlueprint from '@/project/CBlueprint.vue';
import PBudget from '@/project/CBudget.vue';
import PBuildstep from '@/project/CBuildstep.vue';
import PMarket from '@/project/CMarket.vue';
import PMaterial from '@/project/CMaterial.vue';
import PName from '@/project/CName.vue';
import PRaw from '@/project/CRaw.vue';

import ProjectCostApproximation from '@/components/project/CostApproximation.vue';
import SystemSelector from '@/components/SystemSelector.vue';


@Options({
  components: {
    NButton,
    NCard,
    NPageHeader,
    NSkeleton,
    NSpace,
    NTabPane,
    NTabs,

    PAddCost,
    PBlueprint,
    PBudget,
    PBuildstep,
    PMarket,
    PMaterial,
    PName,
    PRaw,

    ProjectCostApproximation,
    SystemSelector
  }
})
export default class ProjectDetails extends Vue {
  public busy: boolean = false;
  public sid: SystemId = 30000142; // Jita

  public show_add_budget = false;

  public project: Project = <Project>{  };

  public stored_materials: IMaterial[] = [];

  public async created() {
    this.busy = true;
    this.project = await ProjectService.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();

    this.calc_stored_materials();
    this.$watch('show_add_budget', () => events.$emit(BUDGET_CHANGE, {}));
    this.busy = false;
  }

  public calc_stored_materials() {
    for (let product of this.project.info.products) {
      let material = this.project
        .stored
        .find(x => x.type_id === product.type_id);

      if (!material) {
        this.stored_materials.push({
          type_id:  product.type_id,
          name:     product.name,
          quantity: product.count,
          stored:   0,
          bonus:    0,
        });
      } else {
        this.stored_materials.push({
          type_id:  product.type_id,
          name:     product.name,
          quantity: product.count,
          stored:   material.quantity,
          bonus:    material.material,
        });
      }
    }
  }

  public refresh() {
    this.busy = true;
    ProjectService.refresh(<string>this.$route.params.pid);
    this.busy = false;
  }
}
</script>
