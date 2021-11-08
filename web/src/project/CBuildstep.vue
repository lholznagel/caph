<template>
  <n-table>
    <thead>
      <tr>
        <th width="34"></th>
        <th width="500">Name</th>
        <th width="100">Runs</th>
        <th width="200">Time</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="entry in buildsteps" :key="entry.type_id">
        <td>
          <item-icon
            type="icon"
            :id="entry.type_id"
            :width="32"
          />
        </td>
        <td>{{ entry.name }}</td>
        <td>{{ entry.run }}</td>
        <td>{{ entry.time }}</td>
      </tr>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NSkeleton, NTable } from 'naive-ui';
import { IProjectBuildstep, ProjectService } from '@/project/service';

import ItemIcon from '@/components/ItemIcon.vue';

class Props {
  // ProjectId
  pid = prop({
    type:     String,
    required: true,
  });

  activity = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NSkeleton,
    NTable,

    ItemIcon,
  }
})
export default class ProjectBuildstep extends Vue.with(Props) {
  public buildsteps: IProjectBuildstep[] = [];

  public async created() {
    this.buildsteps = await ProjectService.buildsteps(this.pid, this.activity);
  }
}
</script>
