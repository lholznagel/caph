<template>
  <n-card title="Projects">
    <template #header-extra>
      <n-button @click="new_project">New project</n-button>
    </template>

    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy && projects.length > 0">
      <thead>
        <tr>
          <th>Name</th>
          <th width="75px"></th>
          <th width="75px"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="project in projects" :key="project.id">
          <td>{{ project.name }}</td>
          <td>
            <n-button @click="open(project.id)">Open</n-button>
          </td>
          <td>
            <n-button @click="remove(project.id)" type="error" ghost>Delete</n-button>
          </td>
        </tr>
      </tbody>
    </n-table>

    <n-empty
      v-if="!busy && projects.length === 0"
      description="No projects yet"
    >
      <template #extra>
        <n-button @click="new_project">New project</n-button>
      </template>
    </n-empty>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NEmpty, NSkeleton, NSpace, NTable } from 'naive-ui';
import { ProjectService, IProject } from '@/services/project';

@Options({
  components: {
    NButton,
    NCard,
    NEmpty,
    NSkeleton,
    NSpace,
    NTable
  }
})
export default class ProjectNew extends Vue {
  public busy: boolean = false;

  public projects: IProject[] = [];

  public async created() {
    this.busy = true;

    this.projects = await ProjectService.projects();

    this.busy = false;
  }

  public new_project() {
    this.$router.push({
      name: 'project_new',
    });
  }

  public open(id: string) {
    this.$router.push({
      name:   'project',
      params: { pid: id }
    });
  }

  public async remove(id: string) {
    await ProjectService.remove(id);
    this.projects = this.projects = await ProjectService.projects();
  }
}
</script>
