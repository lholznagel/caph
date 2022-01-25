<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-tabs
        type="line"
        v-if="!busy"
      >
        <n-tab-pane name="General">
          <n-card>
            <n-space vertical>
              <n-text>Project name</n-text>
              <n-input v-model:value="config.name" placeholder="Name" />

              <n-text>Items to produce</n-text>
              <n-dynamic-input
                v-model:value="config.products"
                :on-create="add_item_to_produce"
                :min="1"
                #="{ value }"
              >
                <div style="width: 100%;">
                  <div style="display: flex; align-items: center;">
                    <n-input-number
                      v-model:value="value.count"
                      style="margin-right: 5px"
                    />
                    <n-select
                      :options="buildable_items"
                      v-model:value="value.type_id"
                      placeholder="Select Item"
                      filterable
                    />
                  </div>
                </div>
              </n-dynamic-input>
              <resolve v-model="project.products" />
            </n-space>

            <template #action>
              <n-space justify="end">
                <n-button
                  :disabled="
                    !config.name ||
                    !config.products ||
                    !config.products[0] ||
                    !config.products[0].type_id"
                  @click="edit_project"
                  type="info"
                >Save</n-button>
              </n-space>
            </template>
          </n-card>
        </n-tab-pane>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NDynamicInput, NInput, NInputNumber, NCard, NTabs, NSpace,
NSelect, NTabPane, SelectOption, NText } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { IConfig, ProjectService } from './service';
import { AssetService } from '@/services/asset';

import Resolve from '@/components/Resolve.vue';

import PMaterial from '@/project/CMaterial.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NInput,
    NInputNumber,
    NSpace,
    NSelect,
    NTabPane,
    NTabs,
    NText,

    Resolve,

    PMaterial,
    PHeader,
    WProject,
  }
})
export default class ProjectOverviewView extends Vue {
  public buildable_items: SelectOption[] = [];

  public config: IConfig = <IConfig>{  };

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_setting`
    );

    this.buildable_items = (await AssetService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });

    this.config = (
      await ProjectService.by_id(<string>this.$route.params.pid)
    ).info;
  }

  public async edit_project() {
    // The selector adds it as an object, we want it as an array
    await ProjectService.edit(
      <string>this.$route.params.pid,
      this.config
    );

    this.$router.push({
      name: 'projects_overview',
      params: {
        pid: this.$route.params.pid
      }
    });
  }

  public add_item_to_produce() {
    return {
      type_id: undefined,
      count:   1,
    }
  }
}
</script>
