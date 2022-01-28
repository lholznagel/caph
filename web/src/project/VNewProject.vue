<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          New project
        </h1>
      </template>
    </n-page-header>

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
        <resolve v-model="config.products" />
      </n-space>

      <template #action>
        <n-space justify="end">
          <n-button
            @click="$router.back()"
            quaternary
          >Cancel</n-button>

          <n-button
            :disabled="
              !config.name ||
              !config.products ||
              !config.products[0] ||
              !config.products[0].type_id"
            @click="create_project"
            type="info"
          >Create</n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber,
NPageHeader, NSelect, NSpace, NText, SelectOption } from 'naive-ui';

import {IConfig, ProjectService2} from '@/project/service';
import {AssetService} from '@/services/asset';

import AssetSelector from '@/components/AssetSelector.vue';
import Resolve from '@/components/Resolve.vue';

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NInput,
    NInputNumber,
    NPageHeader,
    NSelect,
    NSpace,
    NText,

    AssetSelector,
    Resolve
  }
})
export default class ProjectNew extends Vue {
  public buildable_items: SelectOption[]   = [];

  public config: IConfig = <IConfig>{  };

  public async created() {
    // TODO: replace with caching version
    this.buildable_items = (await AssetService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });
  }

  public async create_project() {
    // The selector adds it as an object, we want it as an array
    let pid = await ProjectService2.create(this.config);
    this.$router.push({
      name: 'projects_overview',
      params: {
        pid
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
