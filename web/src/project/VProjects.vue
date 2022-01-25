<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Projects
        </h1>
      </template>
    </n-page-header>

    <n-card content-style="padding: 0">
      <n-skeleton text :repeat="5" v-if="busy" />

      <n-space justify="end" style="margin: 10px;" v-if="!busy">
        <n-button
          :disabled="!selected_project"
          @click="
            $router.push({ name: 'projects_overview', params: {
              pid: selected_project || ''
            } })
          "
        >
          View project
        </n-button>

        <n-button
        :disabled="!selected_project"
        @click="show_confirm = true"
        >
          Delete project
        </n-button>

        <n-button
          @click="$router.push({ name: 'projects_new' })"
          type="info"
        >
          New project
        </n-button>
      </n-space>

      <n-table v-if="!busy && projects.length > 0">
        <thead>
          <tr>
            <th width="10px"></th>
            <th width="700px">Name</th>
            <th width="700px">Status</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="project in projects" :key="project.project">
            <td>
              <n-checkbox
                :checked="selected_project"
                :checked-value="project.project"
                @update:checked="handle_project_select"
                unchecked-value="undefined"
                name="select_project"
              >
              </n-checkbox>
            </td>
            <td>
              <n-button text type="info">
                <router-link
                  :to="
                    {
                      name: 'projects_overview',
                      params: { pid: project.project }
                    }
                  "
                  style="color: inherit; text-decoration: none"
                >
                  {{ project.name }}
                </router-link>
              </n-button>
            </td>
            <td>
              <n-tag v-if="project.status === 'ABORTED'">Aborted</n-tag>
              <n-tag v-else-if="project.status === 'PAUSED'">PAUSED</n-tag>
              <n-tag v-else-if="project.status === 'DONE'">Done</n-tag>
              <n-tag v-else>In Progress</n-tag>
            </td>
          </tr>
        </tbody>
      </n-table>

      <n-empty
        v-if="!busy && projects.length === 0"
        description="No projects yet" />

      <confirm-dialog
        v-model:show="show_confirm"
        :confirm="confirm_delete"
        :resource="project_name()"
      >
        Are you sure you want to delete {{ project_name() }}?
        This action will delete everything that is stored about this project.
        This is not recoverable.
        Please type in 'delete' to confirm.
      </confirm-dialog>
    </n-card>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NAlert, NButton, NCard, NCheckbox, NEmpty, NPageHeader, NSkeleton,
NSpace, NTable, NTag } from 'naive-ui';
import { ProjectService, IInfo } from '@/project/service';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import ConfirmDialog from '@/components/ConfirmDialog.vue';

@Options({
  components: {
    NAlert,
    NButton,
    NCard,
    NCheckbox,
    NEmpty,
    NPageHeader,
    NSkeleton,
    NSpace,
    NTable,
    NTag,

    ConfirmDialog,
  }
})
export default class ProjectsView extends Vue {
  public busy: boolean         = false;
  public show_confirm: boolean = false;

  public selected_project: string | undefined = '';

  public projects: IInfo[] = [];

  public async created() {
    this.busy = true;
    await this.load();
    this.busy = false;

    events.$emit(
      PROJECT_ROUTE,
      undefined
    );
  }

  public async confirm_delete() {
    if (!this.selected_project) { return; }
    await ProjectService.remove(this.selected_project);
    await this.load();
    this.show_confirm = false;
  }

  public handle_project_select(pid: string) {
    if (pid === 'undefined') {
      this.selected_project = undefined;
      return;
    }

    this.selected_project = pid;
  }

  public project_name(): string {
    let info = this.projects
      .find(x => x.project === this.selected_project) || <IInfo>{ name: '' };
    return info.name;
  }

  private async load() {
    this.projects = await ProjectService.get_all();
  }
}
</script>
