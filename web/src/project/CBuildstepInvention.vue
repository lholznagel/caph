<template>
  <n-table>
    <thead>
      <tr>
        <th width="24"></th>
        <th width="34"></th>
        <th width="500">Name</th>
        <th width="100">Probability</th>
        <th width="200">Time per run</th>
        <th width="200">Total time</th>
      </tr>
    </thead>
    <tbody>
      <template v-for="entry in buildsteps" :key="entry.type_id">
        <tr>
          <td>
            <n-icon size="22">
              <angle-right
                style="cursor: pointer"
                @click="entry.open = true"
                v-if="!entry.open"
              />
              <angle-down
                style="cursor: pointer"
                @click="entry.open = false"
                v-if="entry.open"
              />
            </n-icon>
          </td>
          <td>
            <item-icon
              type="bpc"
              :id="entry.type_id"
              :width="32"
            />
          </td>
          <td>{{ entry.name }}</td>
          <td>{{ entry.probability }}</td>
          <td><format-number :value="entry.time_per_run" time /></td>
          <td><format-number :value="entry.time_total" time /></td>
        </tr>
        <tr v-if="entry.open">
          <td colspan="6">
            <n-table>
              <thead>
                <th width="34"></th>
                <th>Name</th>
                <th>Qty.</th>
              </thead>
              <tbody>
                <tr v-for="material in entry.materials">
                  <td>
                    <item-icon
                      type="icon"
                      :id="material.type_id"
                      :width="32"
                    />
                  </td>
                  <td>{{ material.name }}</td>
                  <td>
                    <format-number :value="material.quantity" />
                  </td>
                </tr>
              </tbody>
            </n-table>
          </td>
        </tr>
      </template>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NIcon, NSkeleton, NTable } from 'naive-ui';
import { AngleDown, AngleRight } from '@vicons/fa';
import { ProjectId, ProjectService2, IBuildstepEntry } from '@/project/service';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

class Props {
  // ProjectId
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NIcon,
    NSkeleton,
    NTable,

    AngleDown,
    AngleRight,

    FormatNumber,
    ItemIcon,
  }
})
export default class ProjectBuildstepInvention extends Vue.with(Props) {
  public buildsteps: IBuildstepEntry[] = [];

  public async created() {
    let project = await ProjectService2.by_id(<ProjectId>this.$route.params.pid);
    await project.init();
    this.buildsteps = project.buildsteps.inventions || [];
  }
}
</script>
