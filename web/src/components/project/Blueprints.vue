<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy && blueprints.length > 0">
      <thead>
        <tr>
          <th width="48px"></th>
          <th>Resource</th>
          <th></th>
          <th>Material Efficiency</th>
          <th>Time Efficiency</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="blueprint in blueprints" :key="blueprint.bpid">
          <td><item-icon :id="blueprint.bpid" type="bp" /></td>
          <td><name-by-id :id="blueprint.bpid" /></td>
          <td>
            <n-tag v-if="blueprint.stored">Stored</n-tag>
            <n-tag v-if="blueprint.invention">Invention stored</n-tag>

            <n-tag
              v-if="blueprint.product"
              type="success"
            >
              Product stored
            </n-tag>
            <n-tag
              v-if="blueprint.product_invention"
              type="success"
            >
              Invention product stored
            </n-tag>
            <n-tag
              v-if="
              blueprint.stored &&
                (blueprint.mat_eff === unknown ||
                 blueprint.mat_eff !== 10 ||
                 blueprint.time_eff === unknown ||
                 blueprint.time_eff !== 20)"
              type="warning"
            >
              Not fully researched
            </n-tag>
          </td>
          <td>{{ blueprint.mat_eff  ? blueprint.mat_eff  : '' }}</td>
          <td>{{ blueprint.time_eff ? blueprint.time_eff : '' }}</td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { NAlert, NCard, NSkeleton, NTable, NTag } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { IProjectBlueprint, ProjectService } from '@/services/project';

import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // Project id
  pid = prop({
    type: String,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NCard,
    NSkeleton,
    NTable,
    NTag,

    ItemIcon,
    NameById,
  }
})
export default class ProjectRequiredBlueprints extends Vue.with(Props) {
  public busy: boolean = false;

  public blueprints: IProjectBlueprint[] = [];

  public async created() {
    this.busy = true;
    this.blueprints = (await ProjectService.blueprints(this.pid));
    this.busy = false;
  }
}
</script>
