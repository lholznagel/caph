<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Budget entry
        </h1>
      </template>
    </n-page-header>

    <n-card>
      <n-space vertical>
        <n-text>Amount</n-text>
        <n-dynamic-input
          v-model:value="costs"
          :on-create="default_cost"
          :min="1"
          #="{ value }"
        >
          <div style="width: 100%;">
            <div style="display: flex; align-items: center;">
              <n-input-number
                placeholder="Amount"
                style="width: 100%"
                :show-button="false"
                v-model:value="value.cost"
              >
                <template #suffix>ISK</template>
              </n-input-number>
            </div>
          </div>
        </n-dynamic-input>

        <n-text>Category</n-text>
        <n-select v-model:value="entry.category" :options="categories" />

        <n-text>Cost by</n-text>
        <character-selector v-model:value="entry.character" />

        <n-text>Description</n-text>
        <n-input
          placeholder="Cost description"
          v-model:value="entry.description"
        />
      </n-space>

      <template #action>
        <n-space justify="end">
          <n-button
            @click="$router.back()"
            quaternary
          >Cancel</n-button>

          <n-button
            :disabled="
              costs.length == 0 ||
              !entry.category  ||
              !entry.character ||
              !entry.description"
            @click="edit"
            type="info"
          >
            Edit
          </n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber, NPageHeader, NSelect, NSpace, NText } from 'naive-ui';
import { Service, IBudgetEntry, BUDGET_CATEGORIES } from '@/project/service';

import CharacterSelector from '@/components/CharacterSelector.vue';

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NInput,
    NInputNumber,
    NPageHeader,
    NSelect,
    NSpace,
    NText,

    CharacterSelector
  }
})
export default class AddBudgetView extends Vue {
  public categories = BUDGET_CATEGORIES;

  public costs: { cost: number  | undefined }[] = [{ cost: undefined }];

  public entry: IBudgetEntry = <IBudgetEntry>{};

  public async created() {
    this.entry = await Service.budget_entry(
      <string>this.$route.params.pid,
      <string>this.$route.params.bid
    );

    this.costs = [{ cost: this.entry.amount }];
  }

  public default_cost() {
    return { cost: undefined };
  }

  public async edit() {
    this.entry.amount = <number>this.costs
      .map(x => x.cost)
      .reduce((prev, curr) => (prev ? prev : 0) + (curr ? curr : 0), 0);

    Service.budget_edit_entry(
      <string>this.$route.params.pid,
      <string>this.$route.params.bid,
      this.entry
    );

    this.$router.push({
      name: 'projects_budget',
      params: {
        pid: this.$route.params.pid
      }
    });
  }
}
</script>
