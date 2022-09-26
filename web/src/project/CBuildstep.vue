<template>
  <n-table>
    <thead>
      <tr>
        <th width="24"></th>
        <th width="34"></th>
        <th width="500">Name</th>
        <th width="100">Runs</th>
        <th width="100">Required</th>
        <th width="200">Time per run</th>
        <th width="200">Total time</th>
      </tr>
    </thead>
    <tbody>
      <template v-for="entry in buildsteps" :key="entry.ptype_id">
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
            <eve-icon
              type="icon"
              :id="entry.ptype_id"
              :width="32"
            />
          </td>
          <td>{{ entry.name }}</td>
          <td>{{ Math.ceil(entry.products / entry.products_per_run) }}</td>
          <td>{{ entry.products }}</td>
          <td><format-number :value="entry.time_per_run" time /></td>
          <td><format-number :value="entry.time" time /></td>
        </tr>
        <tr v-if="entry.open">
          <td></td>
          <td colspan="6" style="padding-top: 0; padding-right: 0; padding-bottom: 0">
            <n-table>
              <thead>
                <th width="34"></th>
                <th>Name</th>
                <th>Qty.</th>
              </thead>
              <tbody>
                <tr v-for="material in entry.components">
                  <td>
                    <eve-icon
                      type="icon"
                      :id="material.ptype_id"
                      :width="32"
                    />
                  </td>
                  <td>{{ material.name }}</td>
                  <td>
                    <format-number :value="material.products" />
                  </td>
                </tr>
              </tbody>
            </n-table>
          </td>
        </tr>

        <pre>{{ project.jobs }}</pre>
      </template>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NIcon, NSkeleton, NTable } from 'naive-ui';
import { AngleDown, AngleRight } from '@vicons/fa';
import { IBuildstepEntry, Service } from '@/project/service';
import { ProjectId } from '@/project/project';

import FormatNumber from '@/components/FormatNumber.vue';
import EveIcon from '@/components/EveIcon.vue';

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
    EveIcon,
  }
})
export default class ProjectBuildstep extends Vue.with(Props) {
  public buildsteps: IBuildstepEntry[] = [];

  public async created() {
    let project = await Service.by_id(<ProjectId>this.$route.params.pid);
    await project.init();

    this.buildsteps = <any>await Service.jobs(<ProjectId>this.pid);
    //this.buildsteps = project.buildsteps.manufacture || [];
  }
}
</script>
