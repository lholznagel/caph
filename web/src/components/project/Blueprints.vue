<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy">
      <thead>
        <tr>
          <th width="34"></th>
          <th width="500">Name</th>
          <th width="200">Status</th>
          <th>Location</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in entries" :key="entry.type_id">
          <td>
            <item-icon
              type="bp"
              :id="entry.type_id"
              :width="32"
            />
          </td>
          <td>{{ entry.name }}</td>
          <td>
            <n-tag v-if="entry.status === 'STORED'" type="success">Stored</n-tag>
            <n-tag v-if="entry.status === 'MISSING'" type="error">Missing</n-tag>
          </td>
          <td v-if="entry.container_id">
            <asset-name :tid="entry.container_id" />
            (<station-name :id="entry.location_id" />)
          </td>
          <td v-if="!entry.container_id">
          </td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NSkeleton, NTable, NTag } from 'naive-ui';
import { ProjectService, IProjectRequiredBlueprint } from '@/services/project';

import AssetName from '@/components/AssetName.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import StationName from '@/components/StationName.vue';

class Props {
  // Project uuid
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
  public busy: boolean = false;

  public entries: IProjectRequiredBlueprint[] = [];

  public async created() {
    this.busy = true;

    this.entries = await ProjectService.required_blueprints(this.pid);
    let stored_materials = await ProjectService.stored_materials(this.pid);

    for (let entry of this.entries) {
      let material = stored_materials
        .find(x => x.type_id === entry.type_id);

      if (material) {
        entry.status       = 'STORED';
        entry.container_id = material.container_id;
        entry.location_id  = <number>material.location_id;
      } else {
        entry.status = 'MISSING';
      }
    }

    this.busy = false;
  }
}
</script>
