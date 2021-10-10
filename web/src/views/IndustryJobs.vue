<template>
  <n-card title="Industry Jobs">
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy">
      <thead>
        <tr>
          <th width="48px"></th>
          <th>Blueprint</th>
          <th>Activity</th>
          <th>Location</th>
          <th>End (EVE-Time)</th>
          <th>Remaining</th>
          <th>Owner</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="job in jobs" :key="job.job_id">
          <td><item-icon :id="job.blueprint_type_id" type="bp" /></td>
          <td><name-by-id :id="job.blueprint_type_id" /></td>
          <td>{{ getActivityName(job.activity_id) }}</td>
          <td><location :lid="job.facility_id" /></td>
          <td>{{ job.end_date }}</td>
          <td><format-number :value="job.remaining || 0" is-time /></td>
          <td><owner :ids="[job.installer_id]" /></td>
        </tr>
      </tbody>
    </n-table>
  </n-card>
</template>

<script lang="ts">
import { IIndustryJob, IndustryService } from '@/services/industry';
import { NButton, NCard, NSkeleton, NTable } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import Location from '@/components/Location.vue';
import Owner from '@/components/Owner.vue';
import NameById from '@/components/NameById.vue';

@Options({
  components: {
    NButton,
    NCard,
    NSkeleton,
    NTable,

    FormatNumber,
    ItemIcon,
    Location,
    NameById,
    Owner,
  }
})
export default class Blueprint extends Vue {
  public busy: boolean = false;

  public jobs: IIndustryJob[] = [];

  public async created() {
    this.busy = true;
    this.jobs = (await IndustryService.jobs())
      .sort((a: IIndustryJob, b: IIndustryJob) => {
        const date_a: any = new Date(a.end_date);
        const date_b: any = new Date(b.end_date);

        return date_a - date_b;
      });
    this.busy = false;

    this.timerRefresh();
  }

  public timerRefresh() {
    setInterval(() => {
      for (let i = 0; i < this.jobs.length; i++) {
        // Typescript hates calculating dates
        const start: any = new Date();
        const end: any   = new Date(this.jobs[i].end_date);

        const remaining = Math.floor((end - start) / 1000);
        this.jobs[i].remaining = remaining > 0 ? remaining : 0;
      }
    }, 1000);
  }

  // https://sde.hoboleaks.space/tq/industryactivities.json
  public getActivityName(id: number): string {
    if (id === 1) {
      return 'Manufacturing';
    } else if (id === 3) {
      return 'Time Efficiency';
    } else if (id === 4) {
      return 'Material Efficiency';
    } else if (id === 5) {
      return 'Copying';
    } else if (id === 8) {
      return 'Invention';
    } else if (id === 9) {
      return 'Reaction';
    } else {
      return 'Unknown';
    }
  }
}
</script>

