<template>
  <n-table>
    <thead>
      <tr>
        <th width="34"></th>
        <th width="500">Name</th>
        <th width="100">Runs</th>
        <th width="200">Status</th>
        <th>Location</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="entry in project.required_blueprints" :key="entry.type_id">
        <td>
          <item-icon
            :id="entry.type_id"
            :type="blueprint_type(entry)"
            :width="32"
          />
        </td>
        <td>{{ entry.name }}</td>
        <td v-if="entry.runs">{{ entry.runs }}</td>
        <td v-if="!entry.runs"></td>
        <td>
          <n-tag v-if="entry.container_id" type="success">Stored</n-tag>
          <n-tag v-if="!entry.container_id" type="error">Missing</n-tag>
        </td>
        <td v-if="entry.container_id">
          <asset-name :tid="entry.container_id" />
          (<station-name :id="entry.location_id" />)
        </td>
        <td v-if="!entry.container_id"></td>
      </tr>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NSkeleton, NTable, NTag } from 'naive-ui';
import { IProject, ProjectService } from '@/project/service';

import AssetName from '@/components/AssetName.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import StationName from '@/components/StationName.vue';
import { IProjectBlueprint } from './service';

class Props {
  // ProjectId
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NSkeleton,
    NTable,
    NTag,

    AssetName,
    ItemIcon,
    StationName,
  }
})
export default class ProjectRequiredBlueprints extends Vue.with(Props) {
  public project: IProject = <IProject>{  };

  public show_export: boolean = false;

  public async created() {
    this.project = await ProjectService.by_id(this.pid);
  }

  public blueprint_type(entry: IProjectBlueprint): string {
    if (entry.original === null || entry.original) {
      return 'bp';
    } else {
      return 'bpc';
    }
  }
}
</script>
