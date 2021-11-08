<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Projects
        </h1>
      </template>

      <template #extra>
        <n-button
          @click="show_modal = true"
          type="primary"
          ghost
        >
          New project
        </n-button>
      </template>
    </n-page-header>

    <n-card content-style="padding: 0">
      <n-skeleton text :repeat="5" v-if="busy" />

      <n-table v-if="!busy && projects.length > 0">
        <thead>
          <tr>
            <th width="200px">Name</th>
            <th width="100px">Status</th>
            <th width="600px"></th>
            <th width="50px"></th>
            <th width="50px"></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="project in projects" :key="project.id">
            <td>{{ project.name }}</td>
            <td>
              <n-tag v-if="project.status === 'HALTED'">Halted</n-tag>
              <n-tag v-else-if="project.status === 'DONE'">Done</n-tag>
              <n-tag v-else>In Progress</n-tag>
            </td>
            <td></td>
            <td>
              <n-button @click="edit(project.id)">Edit</n-button>
            </td>
            <td>
              <n-button @click="remove(project.id)" type="error" ghost>Delete</n-button>
            </td>
          </tr>
        </tbody>
      </n-table>

      <n-empty
        v-if="!busy && projects.length === 0"
        description="No porjects yet"
      >
        <template #extra>
          <n-button @click="show_modal = true">New project</n-button>
        </template>
      </n-empty>

      <p-project v-model:show="show_modal" :config="project" :is-edit="is_edit" />

      <n-modal
        v-model:show="show_confirm"
        preset="dialog"
        title="Dialog"
        type="warning"
        content="Are you sure you want to delete the project?"
        positive-text="Submit"
        @positive-click="confirm_delete"
        @negative-click="show_confirm = false"
        negative-text="Cancel"
      />
    </n-card>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NEmpty, NDynamicInput, NInput, NInputNumber, NModal,
NPageHeader, NSkeleton, NSpace, NTable, NTag } from 'naive-ui';
import { ProjectService, IProject, ProjectId } from '@/project/service';
import { events } from '@/main';
import { PROJECT_CHANGE } from '@/event_bus';

import PProject from '@/project/MProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NEmpty,
    NInput,
    NInputNumber,
    NModal,
    NPageHeader,
    NSkeleton,
    NSpace,
    NTable,
    NTag,

    PProject
  }
})
export default class ProjectTemplates extends Vue {
  public busy: boolean         = false;
  public show_modal: boolean   = false;

  public projects: IProject[] = [];

  public project: IProject = <IProject>{  };
  public is_edit: boolean  = false;

  public show_confirm: boolean = false;
  public delete_id: ProjectId = <ProjectId>'';

  public async created() {
    this.busy = true;
    await this.load_projects();
    this.busy = false;

    // When the modal closes refresh
    this.$watch('show_modal', () => this.show_modal ? {} : this.load_projects());
  }

  public edit(id: ProjectId | undefined) {
    if (!id) { return; }

    let project = this.projects.find(x => x.id === id);
    if (!project) { return; }
    this.project = project;
    this.show_modal = true;
    this.is_edit = true;
  }

  public async remove(id: ProjectId | undefined) {
    if (!id) { return; }

    this.delete_id = id;
    this.show_confirm = true;
  }

  public async confirm_delete() {
    if (!this.delete_id) { return; }
    await ProjectService.remove(this.delete_id);
    await this.load_projects();
  }

  private async load_projects() {
    this.is_edit = false;
    this.project = <IProject>{};

    this.projects = await ProjectService.get_all();

    events.$emit(PROJECT_CHANGE, {});
  }
}
</script>
