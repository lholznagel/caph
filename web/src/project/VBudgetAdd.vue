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
        <n-select v-model:value="budget.category" :options="categories" />

        <n-text>Cost by</n-text>
        <character-selector v-model:value="budget.character" />

        <n-text>Description</n-text>
        <n-input
          placeholder="Cost description"
          v-model:value="budget.description"
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
              !budget.category  ||
              !budget.character ||
              !budget.description"
            @click="add"
            type="info"
          >
            Add
          </n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber, NPageHeader, NSelect, NSpace, NText } from 'naive-ui';
import { ProjectService2, IBudgetEntry,BUDGET_CATEGORIES } from '@/project/service';

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

  public budget: IBudgetEntry = <IBudgetEntry>{};

  public default_cost() {
    return { cost: undefined };
  }

  public async add() {
    this.budget.amount = <number>this.costs
      .map(x => x.cost)
      .reduce((prev, curr) => (prev ? prev : 0) + (curr ? curr : 0), 0);

    await ProjectService2.budget_add_entry(<string>this.$route.params.pid, this.budget);
    this.$router.push({
      name: 'projects_budget',
      params: {
        pid: this.$route.params.pid
      }
    });
  }
}
</script>
