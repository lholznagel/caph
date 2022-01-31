<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <p-header />

      <n-tabs
        type="line"
        v-if="!busy"
      >
        <n-tab-pane name="Overview">
          <n-card content-style="padding: 0">
            <n-space justify="end" style="margin: 10px">
              <n-button
                @click="
                  $router.push({ name: 'projects_budget_edit', params: {
                    pid: $route.params.pid,
                    bid: selected_budget || '',
                  } })
                "
                :disabled="!selected_budget"
              >
                Edit Entry
              </n-button>

              <n-button
                @click="show_confirm = true"
                :disabled="!selected_budget"
              >
                Delete Entry
              </n-button>

              <n-button
                @click="$router.push({
                  name: 'projects_budget_add',
                  params: { pid: $route.params.pid }
                })"
                type="info"
              >
                Add cost
              </n-button>
            </n-space>

            <n-table v-if="!busy && budget_entries.length > 0">
              <tbody>
                <tr>
                  <th width="10px"></th>
                  <th width="100" style="text-align: right">Amount</th>
                  <th width="100">Category</th>
                  <th width="500">Description</th>
                  <th width="100">Created at</th>
                  <th width="32"></th>
                </tr>
                <tr v-for="entry in budget_entries" :key="entry.budget">
                  <td>
                    <n-checkbox
                      :checked="selected_budget"
                      :checked-value="entry.budget"
                      @update:checked="handle_budget_select"
                      unchecked-value="undefined"
                      name="selected_budget"
                    >
                    </n-checkbox>
                  </td>
                  <td style="text-align: right">
                    <format-number
                      :value="entry.amount"
                      :type="entry.amount > 0 ? 'success' : 'error'"
                    />
                    ISK</td>
                  <td>{{ category(entry.category) }}</td>
                  <td>{{ entry.description }}</td>
                  <td>{{ format_date(entry.created_at) }}</td>
                  <td><character :id="entry.character" /></td>
                </tr>
              </tbody>
            </n-table>

            <n-empty
              v-if="!busy && budget_entries.length === 0"
              description="No cost added yet"
            />

            <confirm-dialog
              v-model:show="show_confirm"
              :confirm="confirm_delete"
              resource="entry"
            >
              Are you sure you want to delete the entry?
              This is not be undone.<br>
              Please type in 'delete' to confirm.
            </confirm-dialog>
          </n-card>
        </n-tab-pane>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NCheckbox, NEmpty, NSpace, NTable, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import Character from '@/components/Character.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import FormatNumber from '@/components/FormatNumber.vue';

import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';
import { IBudgetEntry, ProjectService2 } from './service';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NEmpty,
    NSpace,
    NTable,
    NTabPane,
    NTabs,

    Character,
    ConfirmDialog,
    FormatNumber,

    PHeader,
    WProject,
  }
})
export default class ProjectBudgetView extends Vue {
  public show_confirm: boolean = false;
  public selected_budget: string | undefined = '';

  public budget_entries: IBudgetEntry[] = [];
  public balance: number = 0;

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_budget`
    );

    await this.load();
  }

  public async load() {
    this.budget_entries = (await ProjectService2.budget_entries(<string>this.$route.params.pid)).reverse();
    this.balance = this.budget_entries
      .map(x => x.amount)
      .reduce((acc, curr) => acc + curr, 0);
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

  public handle_budget_select(bid: string) {
    if (bid === 'undefined') {
      this.selected_budget = undefined;
      return;
    }

    this.selected_budget = bid;
  }

  public async confirm_delete() {
    if (!this.selected_budget) { return; }

    await ProjectService2.budget_remove_entry(
      <string>this.$route.params.pid,
      this.selected_budget
    );
    await this.load();

    this.selected_budget = undefined;
    this.show_confirm = false;
  }
}
</script>
