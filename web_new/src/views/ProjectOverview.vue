<template>
  <n-card title="Projects">
    <template #header-extra>
      <n-button @click="newProject">New project</n-button>
    </template>

    <n-skeleton text v-if="busy" :repeat="5" />

    <n-data-table
      ref="table"
      :columns="columns"
      :data="entries"
      v-if="!busy"
    />
  </n-card>
</template>

<script lang="ts">
import { NButton, NCard, NDataTable, NInput, NSkeleton } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';

import { IProject, ProjectService } from '@/services/project';

@Options({
  components: {
    NButton,
    NCard,
    NDataTable,
    NInput,
    NSkeleton
  }
})
export default class ProjectOverview extends Vue {
  public busy: boolean = false;

  public columns: any = [];
  public entries: IProject[] = [];

  public async created() {
    this.busy = true;

    this.columns = createColumns(this.deleteProject, this.openDetails);
    this.entries = await ProjectService.projects();

    this.busy = false;
  }

  public openDetails(id: string) {
    this.$router.push({
      name: 'project',
      params: { id }
    });
  }

  public async deleteProject(id: string) {
    await ProjectService.delete(id);
    this.entries = this.entries.filter(x => x.id !== id);
  }

  public newProject() {
    this.$router.push({
      name: 'project_new',
    });
  }

  public handleNameFilter(val: string) {
    const ref: any = this.$refs.table;
    ref.filter({
      name: [val]
    });
  }
}

const createColumns = (deleteProject: any, openDetails: any) => {
  return [{
    title: 'Name',
    key:   'name',
    width: 1600,
    filter(value: string, row: IProject) {
      return ~row.name.toLowerCase().indexOf(value.toLowerCase());
    }
  }, {
    title: '',
    key:   'details',
    render(row: IProject) {
      return h(
        NButton,
        { onClick: () => openDetails(row.id) },
        { default: () => 'Details' }
      )
    }
  }, {
    title: '',
    key:   'delete',
    render(row: IProject) {
      return h(
        NButton,
        {
          onClick: () => deleteProject(row.id),
          ghost:   true,
          type:    'error'
        },
        { default: () => 'Delete' }
      )
    }
  }];
}
</script>
