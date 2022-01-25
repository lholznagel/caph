<template>
  <div>
    <n-table v-if="!busy && budget_entries.length > 0">
      <tbody>
        <tr>
          <th width="100" style="text-align: right">Amount</th>
          <th width="100">Category</th>
          <th width="500">Description</th>
          <th width="100">Created at</th>
          <th width="32"></th>
          <th width="10"></th>
        </tr>
        <tr v-for="entry in budget_entries" :key="budget.budget">
          <td style="text-align: right">
            <format-number
              :value="entry.amount"
              :type="entry.amount > 0 ? 'success' : 'error'"
            />
            ISK</td>
          <td>{{ category(entry.category) }}</td>
          <td>{{ entry.description }}</td>
          <td>{{ format_date(entry.created_at) }}</td>
          <td><owner :id="entry.character" /></td>
          <td>
            <n-button
              @click="edit(entry.budget)"
              tertiary
              type="info"
            >
              Edit
            </n-button>
          </td>
        </tr>
      </tbody>
    </n-table>

    <n-empty
      v-if="!busy && budget_entries.length === 0"
      description="No cost added yet"
    >
      <template #extra>
        <n-button
          @click="show_modal = true"
          ghost
          type="primary"
        >Add cost</n-button>
      </template>
    </n-empty>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NEmpty, NInput, NInputNumber, NModal, NSkeleton, NSpace,
NTable, NPageHeader, NStatistic, NGrid, NGridItem } from 'naive-ui';
import { ProjectService, IBudgetEntry } from '@/project/service';
import { events } from '@/main';
import { BUDGET_CHANGE } from '@/event_bus';

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
    NEmpty,
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

  public show_modal: boolean = false;
  public is_edit: boolean    = false;

  public balance: number = 0;

  public budget_entries: IBudgetEntry[] = [];
  public budget: IBudgetEntry           = <IBudgetEntry>{};

  public async created() {
    this.busy = true;

    await this.load();

    this.busy = false;

    this.$watch('show_modal', () => this.show_modal ? {} : this.load());
    events.$on(BUDGET_CHANGE, () => this.load());
  }

  public async load() {
    this.budget_entries = (await ProjectService.budget_entries(this.pid)).reverse();

    this.balance = this.budget_entries
      .map(x => x.amount)
      .reduce((acc, curr) => acc + curr, 0);
  }

  public async edit(id: string) {
    this.budget = <IBudgetEntry>this.budget_entries.find(x => <string>x.budget === id);
    this.is_edit = true;
    this.show_modal = true;
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

  public category(c: string): string {
    let last = c.slice(1);
    last = last.toLowerCase();
    return c.slice(0, 1) + last;
  }
}
</script>
