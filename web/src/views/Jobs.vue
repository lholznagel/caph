<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Industry Jobs
        </h1>
      </template>
    </n-page-header>

    <n-card>
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
            <td>{{ job.name }}</td>
            <td>{{ getActivityName(job.activity_id) }}</td>
            <td><station-name :id="job.station_id" /></td>
            <td>
                <n-button
                  text
                  tag="a"
                  target="_blank"
                  type="primary"
                  :href="'https://time.nakamura-labs.com/?#' + new Date(job.end_date).valueOf() / 1000"
                >
                  {{ job.end_date }}
                </n-button>
              </td>
            <td>
              <n-tag v-if="job.remaining === 0" type="success">Done</n-tag>
              <format-number v-if="job.remaining > 0" :value="job.remaining || 0" is-time />
            </td>
            <td><owner :id="job.installer_id" /></td>
          </tr>
        </tbody>
      </n-table>
    </n-card>
  </div>
</template>

<script lang="ts">
import { IIndustryJob, IndustryService } from '@/services/industry';
import { NButton, NCard, NPageHeader, NSkeleton, NTable, NTag } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import StationName from '@/components/StationName.vue';
import Owner from '@/components/Owner.vue';

@Options({
  components: {
    NButton,
    NCard,
    NPageHeader,
    NSkeleton,
    NTable,
    NTag,

    FormatNumber,
    ItemIcon,
    StationName,
    Owner,
  }
})
export default class IndustryJobs extends Vue {
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

