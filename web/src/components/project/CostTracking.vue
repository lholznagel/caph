<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-page-header title="Cost tracking" v-if="!busy">
      <n-grid :cols="5">
        <n-grid-item>
          <n-statistic label="Current balance">
            <template #default>
              <format-number
                :type="balance > 0 ? 'success' : 'error'"
                :value="balance"
              />
            </template>
          </n-statistic>
        </n-grid-item>
      </n-grid>

      <template #extra>
        <n-button v-if="!busy" @click="show_add_modal = true" ghost type="primary">
          Add cost
        </n-button>
      </template>
    </n-page-header>

    <n-table v-if="!busy" style="margin-top: 10px">
      <tbody>
        <tr>
          <th width="150">Amount</th>
          <th width="500">Description</th>
          <th width="150">Created at</th>
          <th width="32">Creator</th>
        </tr>
        <tr v-for="tracking in trackings" :key="tracking.id">
          <td style="text-align: right">
            <format-number
              :value="tracking.amount"
              :type="tracking.amount > 0 ? 'success' : 'error'"
            />
            ISK</td>
          <td>{{ tracking.description }}</td>
          <td>{{ format_date(tracking.created_at) }}</td>
          <td><owner :id="tracking.creator" /></td>
        </tr>
      </tbody>
    </n-table>

    <n-modal v-model:show="show_add_modal">
      <n-card style="width: 600px;" title="Add cost" :bordered="false" size="huge">
        <n-space vertical>
          <label>Amount</label>
          <n-input-number
            placeholder="Amount"
            :show-button="false"
            v-model:value="tracking.amount"
          >
            <template #suffix>ISK</template>
          </n-input-number>

          <label>Description</label>
          <n-input
            placeholder="Cost description"
            v-model:value="tracking.description"
          />
        </n-space>

        <template #action>
          <n-space justify="space-between">
            <n-button @click="show_add_modal = false" ghost>Close</n-button>
            <n-button @click="add" ghost type="primary">Add</n-button>
          </n-space>
        </template>
      </n-card>
    </n-modal>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NInput, NInputNumber, NModal, NSkeleton, NSpace,
NTable, NPageHeader, NStatistic, NGrid, NGridItem } from 'naive-ui';
import { ProjectService, IProjectCostTracking } from '@/project/service';

import FormatNumber from '@/components/FormatNumber.vue';
import Owner from '@/components/Owner.vue';

class Props {
  // Project id
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NButton,
    NCard,
    NInput,
    NInputNumber,
    NModal,
    NSkeleton,
    NSpace,
    NTable,

    NPageHeader,
    NStatistic,
    NGrid,
    NGridItem,

    FormatNumber,
    Owner
  }
})
export default class ProjectCostTracking extends Vue.with(Props) {
  public busy: boolean = false;

  public show_add_modal: boolean = false;

  public balance: number = 0;

  public trackings: IProjectCostTracking[] = [];
  public tracking: IProjectCostTracking    = <IProjectCostTracking>{};

  public async created() {
    this.busy = true;

    await this.load();

    this.busy = false;
  }

  public async load() {
    this.trackings = (await ProjectService.trackings(this.pid)).reverse();

    this.balance = this.trackings
      .map(x => x.amount)
      .reduce((acc, curr) => acc + curr, 0);
  }

  public async add() {
    await ProjectService.tracking_add(this.pid, this.tracking);
    this.show_add_modal = false;
    this.tracking = <IProjectCostTracking>{};
    await this.load();
  }

  public format_date(date_ms: number | undefined): string {
    if (!date_ms) { return ''; }

    const date = new Date(date_ms);

    const preZero = (val: number): string => val >= 10 ? `${val}` : `0${val}`;
    const day = preZero(date.getUTCDate());
    const month = preZero(date.getMonth() + 1);
    const year = date.getFullYear();
    const hours = preZero(date.getUTCHours());
    const minutes = preZero(date.getUTCMinutes());

    return `${day}.${month}.${year} ${hours}:${minutes}`;
  }
}
</script>
