<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Industry Jobs
        </h1>
      </template>
    </n-page-header>

    <loading
      description="Loading industry jobs"
      :busy=busy
    />

    <n-text>Active jobs: {{ jobs.length }}</n-text>

    <n-card content-style="padding: 0" v-if="!busy">
      <n-table v-if="!busy">
        <thead>
          <tr>
            <th width="40px"></th>
            <th width="200px">Activity</th>
            <th width="500px">Name</th>
            <th width="100px">Runs</th>
            <th width="500px">Remaining</th>
            <th width="300px">End date (EVE-Time)</th>
            <th width="150px" style="text-align: right">Cost</th>
            <th width="40px"></th>
            <th width="500px">Owner</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="j in jobs" :key="j.job_id">
            <tr>
              <td>
                <eve-icon :id="j.blueprint_type_id" type="bp" />
              </td>
              <td>
                {{ j.activity_id }}
              </td>
              <td>
                <item :tid="j.blueprint_type_id">
                  <template v-slot="{ item }">
                    {{ rename(item.name) }}
                  </template>
                </item>
              </td>
              <td>
                {{ j.runs }}
              </td>
              <td>
                <n-tag v-if="j.remaining === 0" type="success">Done</n-tag>
                <format-number v-if="j.remaining > 0" :value="j.remaining || 0" time />
              </td>
              <td>
                <n-button
                  text
                  tag="a"
                  target="_blank"
                  type="primary"
                  :href="'https://time.nakamura-labs.com/?#' + new Date(j.end_date).valueOf() / 1000"
                >
                  {{ format_end_date(j.end_date) }}
                </n-button>
              </td>
              <td style="text-align: right">
                <format-number
                  :value="j.cost"
                />
                ISK
              </td>
              <td>
                <eve-icon v-if="!j.corporation_id" :id="j.installer_id" character />
                <eve-icon v-if="j.corporation_id" :id="j.corporation_id" corporation />
              </td>
              <td>
                <character-info v-if="!j.corporation_id" :id="j.installer_id">
                  <template v-slot="{ info }">
                    {{ info.name }}
                  </template>
                </character-info>

                <div v-if="j.corporation_id">
                  <corporation-info :id="j.corporation_id">
                    <template v-slot="{ info }">
                      {{ info.name }}
                    </template>
                  </corporation-info>
                  <character-info :id="j.installer_id">
                    <template v-slot="{ info }">
                      {{ info.name }}
                    </template>
                  </character-info>
                </div>
              </td>
            </tr>
          </template>
        </tbody>
      </n-table>
    </n-card>
  </div>
</template>

<script lang="ts">
import { IndustryService, IIndustryJob } from '@/services/industry';
import { events } from '@/main';
import { NButton, NCard, NCountdown, NPageHeader, NSpace, NTable, NTag, NText } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { ROUTE_CHANGE } from '@/event_bus';
import { format_date } from '@/utils';

import CharacterInfo from '@/components/CharacterInfo.vue';
import CorporationInfo from '@/components/CorporationInfo.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Item from '@/components/Item.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loading from '@/components/Loading.vue';

@Options({
  components: {
    NButton,
    NCard,
    NCountdown,
    NPageHeader,
    NSpace,
    NTable,
    NTag,
    NText,

    CharacterInfo,
    CorporationInfo,
    FormatNumber,
    Item,
    EveIcon,
    Loading,
  }
})
export default class IndustryJobs extends Vue {
  public busy: boolean = false;

  public jobs: IIndustryJob[] = [];
  public selected: number = 0;

  public async created() {
    events.$emit(
      ROUTE_CHANGE,
      this.$route.name
    );

    this.busy = true;
    await this.load();
    this.busy = false;

    this.timerRefresh();
  }

  public format_end_date(input: string): string {
    return format_date(new Date(input).valueOf());
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

  public rename(name: string): string {
    if (name) {
      return name
        .replace(' Blueprint', '')
        .replace(' Reaction Formula', '');
    } else {
      return '';
    }
  }

  private async load() {
    let character = await IndustryService.character_jobs();
    let corporation = await IndustryService.corporation_jobs();
    this.jobs = [...character, ...corporation];
    this.jobs = this.jobs.sort(
      (a: IIndustryJob, b: IIndustryJob) => a.end_date.localeCompare(b.end_date)
    );
  }
}
</script>
