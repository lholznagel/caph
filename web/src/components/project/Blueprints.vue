<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy">
      <thead>
        <tr>
          <th width="34"></th>
          <th width="500">Name</th>
          <th width="50">Runs</th>
          <th width="200">Status</th>
          <th>Location</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in entries" :key="entry.type_id">
          <td>
            <item-icon
              :id="entry.type_id"
              :type="entry.original ? 'bpo' : 'bpc'"
              :width="32"
            />
          </td>
          <td>{{ entry.name }}</td>
          <td v-if="entry.info">{{ entry.info.runs }}</td>
          <td v-if="!entry.info">Not stored</td>
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
import { AssetService, IAsset } from '@/services/asset';
import { ProjectService } from '@/services/project';

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

  public entries: IBlueprintInfo[] = [];

  public async created() {
    this.busy = true;

    let project = await ProjectService.project(this.pid);
    let blueprints = await ProjectService.template_required_blueprints(project.template);
    let stored_materials = await ProjectService.project_stored_materials(this.pid);

    for (let blueprint of blueprints) {
      let entry: IBlueprintInfo = <IBlueprintInfo>{
        name:    blueprint.name,
        type_id: blueprint.type_id
      };

      let material = stored_materials
        .find(x => x.type_id === blueprint.type_id);

      if (material) {
        entry.info         = await AssetService.by_id(<number>material.item_id);
        entry.status       = 'STORED';
        entry.container_id = material.container_id;
        entry.location_id  = <number>material.location_id;
      } else {
        entry.status = 'MISSING';
      }
      this.entries.push(entry);
    }

    this.busy = false;
  }
}

interface IBlueprintInfo {
  info:         IAsset;
  status:       string;
  container_id: number;
  location_id:  number;
  type_id:      number;
  name:         string;
}
</script>
