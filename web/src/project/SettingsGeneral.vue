<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-card title="Project name">
        <n-input v-model:value="project.info.name" placeholder="Name" />
      </n-card>

      <n-card
        style="margin-top: 10px"
        title="Products"
      >
        <settings-products
          :project="project.info"
        />
      </n-card>

      <n-space style="margin-top: 10px" justify="end">
        <n-button
          :disabled="
            !project.info.name ||
            !project.info.products ||
            !project.info.products[0] ||
            !project.info.products[0].type_id"
          @click="edit_project"
          type="info"
        >
          Save project
        </n-button>
      </n-space>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber, NSpace,
NSelect, NTable, SelectOption, NText } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { Project } from './project';
import { Service } from '@/project/service';
import { ProjectId } from '@/project/project';
import { ItemService } from '@/services/item';

import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/wrapper/Project.vue';

import SettingsProducts from '@/project/components/SettingsProducts.vue';

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NInput,
    NInputNumber,
    NSelect,
    NSpace,
    NTable,
    NText,

    SettingsProducts,

    PHeader,
    WProject
  }
})
export default class ProjectSettingsGeneral extends Vue {
  public buildable_items: SelectOption[] = [];

  public project: Project = <Project>{  };

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_settings_general`
    );

    this.buildable_items = (await ItemService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });

    this.project = await Service.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();
  }

  public async edit_project() {
    // The selector adds it as an object, we want it as an array
    await Service.edit(
      <string>this.$route.params.pid,
      {
        name: this.project.info.name,
        products: this.project.info.products,
      }
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
