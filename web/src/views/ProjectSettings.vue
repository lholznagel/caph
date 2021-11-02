<template>
  <n-card :title="$route.params.pid ? 'Update project' : 'New project'">
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-form
      v-if="!busy"
      :model="project"
    >
      <n-form-item label="Project name" path="name">
        <n-input v-model:value="project.name" placeholder="Name of the project" />
      </n-form-item>

      <n-form-item label="Project containers">
        <n-dynamic-input
          v-model:value="project.containers"
          :on-create="add_container"
          :min="1"
          #="{ value }"
        >
          <div style="width: 100%;">
            <div style="display: flex; align-items: center;">
              <asset-selector v-model:value="value.item_id" :filter="{ group: 448 }" />
            </div>
          </div>
        </n-dynamic-input>
      </n-form-item>

      <n-form-item label="Items to produce">
        <n-dynamic-input
          v-model:value="project.products"
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
      </n-form-item>

      <n-form-item :show-label="false" v-if="!$route.params.pid">
        <n-input
          type="textarea"
          placeholder="Production list"
          v-model:value="production_list"
          @input="debounce(() => resolve_production_list())"
        />
      </n-form-item>
    </n-form>

    <template #action>
      <n-space justify="space-between">
        <n-button @click="back">Back</n-button>

        <n-button
          v-if="!$route.params.pid"
          @click="create_project"
          type="primary"
          ghost
        >Create Project</n-button>
        <n-button
          v-if="$route.params.pid"
          @click="update_project"
          type="primary"
          ghost
        >Update Project</n-button>
      </n-space>
    </template>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NForm, NFormItem, NInput, NInputNumber,
  NSelect, NSkeleton, NSpace, SelectOption } from 'naive-ui';
import { AssetService } from '@/services/asset';
import { ProjectService, IProject } from '@/services/project';

import AssetSelector from '@/components/AssetSelector.vue';
import StationSelector from '@/components/StationSelector.vue';

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

    AssetSelector,
    StationSelector,
  }
})
export default class ProjectSettings extends Vue {
  public busy: boolean = false;

  public production_list: string = '';

  public project: IProject = this.default_project();

  public buildable_items: SelectOption[] = [];

  public debounce_timeout: any = null;

  public async created() {
    this.busy = true;

    if (this.$route.params.pid) {
      this.project = await ProjectService.project(<string>this.$route.params.pid);
      this.project.containers = await ProjectService.containers(<string>this.$route.params.pid);
      this.project.products = await ProjectService.products(<string>this.$route.params.pid);
    }

    this.buildable_items = (await AssetService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });

    this.busy = false;
  }

  // Debounces the user input for 500 milliseconds
  // After the debounce the given function is executed
  public debounce(fnc: () => void): void {
    clearTimeout(this.debounce_timeout);
    this.debounce_timeout = setTimeout(() => { fnc() }, 500)
  }

  public async resolve_production_list() {
    if (!this.production_list) {
      return;
    }

    let map = new Map();
    let splitted = this.production_list
      .split('\n')
      .filter(x => x !== '');

    for (let split of splitted) {
      let count = 1;
      let name = split;

      let rgx_match = split.match(/ x([0-9]+)/);
      if (rgx_match) {
        count = Number(rgx_match[1]);
        name  = name.replace(/ x([0-9]+)/, '');
      }

      if (map.has(name)) {
        let entry = map.get(name);
        map.set(name, count += entry);
      } else {
        map.set(name, count);
      }
    }

    let ids: any[] = await AssetService.resolve_id_from_name_bulk(
      Array.from(map.keys()),
      { has_blueprint: true }
    );

    if (
      this.project.products.length === 1 &&
      !this.project.products[0].type_id) {

      this.project.products = [];
    }

    for (let id of ids) {
      let entry = map.get(id.name);
      this.project.products.push({
        type_id: id.type_id,
        count:   entry
      });
    }

    this.production_list = '';
  }

  public add_container() {
    return {
      item_id: undefined,
      count:   1,
    }
  }

  public add_item_to_produce() {
    return {
      type_id: undefined,
      count:   1,
    }
  }

  public async create_project() {
    await ProjectService.create(this.project);
    this.back();
  }

  public async update_project() {
    await ProjectService.update(<string>this.$route.params.pid, this.project);
  }

  public back() {
    this.$router.push({
      name: 'industry_projects',
    });
  }

  private default_project(): IProject {
    return {
      name: '',
      containers: [{
        item_id: null,
      }],
      products: [{
        count:   1,
        type_id: null
      }]
    }
  }
}
</script>
