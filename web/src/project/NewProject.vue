<template>
  <div>
    <n-page-header>
      <template #title>
        <h1 style="margin-bottom: 0">
          New project
        </h1>
      </template>

      <p>
        Create a new project to track your build progress, required blueprints, required materials and budget.
        <br>
        Start by inserting a project name and then either select one or more items to produce or copy just copy a fitting / list of items and their quantity in the field.
      </p>
    </n-page-header>

    <n-card title="Project name">
      <n-input v-model:value="new_project.name" placeholder="Name" />
    </n-card>

    <n-card
      style="margin-top: 10px"
      title="Products"
    >
      <settings-products
        :project="new_project"
      />
    </n-card>

    <n-space style="margin-top: 10px" justify="end">
      <n-button
        @click="$router.back()"
        quaternary
      >Cancel</n-button>

      <n-button
        :disabled="
          !new_project.name ||
          !new_project.products ||
          !new_project.products[0] ||
          !new_project.products[0].type_id"
        @click="create_project"
        type="info"
      >
        Create project
      </n-button>
    </n-space>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NInput, NPageHeader, NSpace, NText } from 'naive-ui';

import { Service, INewProject } from '@/project/service';

import SettingsProducts from '@/project/components/SettingsProducts.vue';

@Options({
  components: {
    NButton,
    NCard,
    NInput,
    NPageHeader,
    NSpace,
    NText,

    SettingsProducts
  }
})
export default class ProjectNew extends Vue {
  public new_project: INewProject = <INewProject>{
    name: '',
    products: []
  };

  public async create_project() {
    // The selector adds it as an object, we want it as an array
    let pid = await Service.create(this.new_project);
    this.$router.push({
      name: 'projects_overview',
      params: {
        pid
      }
    });
  }
}
</script>
