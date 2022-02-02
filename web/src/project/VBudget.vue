<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-card content-style="padding: 0">
        <p-table-header
          :reload="() => project.load_budget_entries()"
          :primary-action="{
            name: 'projects_budget_add',
            params: { pid: $route.params.pid }
          }"
          primary-content="Add cost"
        >
          <template #additional>
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
          </template>
        </p-table-header>

        <div style="margin: 10px">
          <filter-text
            v-model:entries="project.budget_entries"
            :filters="filters"
            :options="filterOptions"
            style="width: 100%"
          />

          <filter-element
            style="margin-top: 5px"
            :filters="filters"
            :options="filterOptions"
          />
        </div>

        <n-table>
          <tbody>
            <tr>
              <th width="10px"></th>
              <th width="100" style="text-align: right">Amount</th>
              <th width="100">Category</th>
              <th width="500">Description</th>
              <th width="100">Created at</th>
              <th width="32"></th>
            </tr>
            <tr v-for="entry in project.budget_entries" :key="entry.budget">
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
              <td><format-number :value="entry.created_at" date /></td>
              <td><character :id="entry.character" /></td>
            </tr>
          </tbody>
        </n-table>

        <n-empty
          description="No cost added yet"
          size="large"
          style="margin: 5%"
          v-if="!busy && project.budget_entries.length === 0"
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
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NCheckbox, NIcon, NEmpty, NSpace, NTable } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { Service } from '@/project/service';
import { h, ref, VNode } from '@vue/runtime-core';
import { Project, ProjectId } from './project';

import { Refresh } from '@vicons/tabler';

import Character from '@/components/Character.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import FormatNumber from '@/components/FormatNumber.vue';

import FilterElement from '@/components/FilterElement.vue';
import FilterText, { IFilterOption } from '@/components/Filter.vue';

import PHeader from '@/project/CHeader.vue';
import PTableHeader from '@/project/CTableHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NEmpty,
    NIcon,
    NSpace,
    NTable,

    Refresh,

    Character,
    ConfirmDialog,
    FilterElement,
    FilterText,
    FormatNumber,

    PHeader,
    PTableHeader,
    WProject,
  }
})
export default class ProjectBudgetView extends Vue {
  public busy: any = ref(true);

  public show_confirm: boolean = false;
  public selected_budget: string | undefined = '';

  public balance: number = 0;

  public filters: any = {};
  public filterOptions: { [key: string]: IFilterOption } = {};

  private project: Project = <Project>{  };

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );

    this.project = await Service.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();

    await this.filter();
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

  // Deletes the selected entry and reset the set variables.
  public async confirm_delete() {
    if (!this.selected_budget) { return; }

    await this.project.budget_remove_entry(this.selected_budget);

    this.selected_budget = undefined;
    this.show_confirm = false;
  }

  private async filter() {
    let categories: string[] = [];
    this.project.budget_entries.map(x => {
      if (categories.indexOf(x.category) === -1) {
        categories.push(x.category)
      }
    });
    categories.sort();

    let characters: number[] = [];
    this.project.budget_entries.map(x => {
      if (characters.indexOf(x.character) === -1) {
        characters.push(x.character)
      }
    });
    characters.sort();

    this.filterOptions = {
      category: {
        label:   'Category',
        options:  categories,
        template: (val: string): VNode => {
          return h(
            'span',
            {},
            { default: () => this.category(val) }
          )
        }
      },
      character: {
        label:   'Character',
        options:  characters,
        template: (val: string): VNode => {
          return h(
            Character,
            { id: Number(val), withText: true }
           )
        }
      }
    };
  }
}
</script>
