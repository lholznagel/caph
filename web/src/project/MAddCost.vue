<template>
  <n-modal v-model:show="show" :on-update:show="close">
    <n-card
      style="width: 600px;"
      size="huge"
      :bordered="false"
      :title="isEdit ? 'Edit cost' : 'Add cost'"
    >
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

        <n-text>Description</n-text>
        <n-input
          placeholder="Cost description"
          v-model:value="tracking.description"
        />

        <n-text>Cost by</n-text>
        <character-selector v-model:value="tracking.character" />
      </n-space>

      <template #action>
        <n-space justify="space-between">
          <n-button @click="close()" ghost>Close</n-button>

          <div>
            <n-button
              @click="remove(tracking.id)"
              ghost
              type="error"
              style="margin-right: 10px"
              v-if="isEdit"
            >Delete</n-button>

            <n-button
              @click="add"
              ghost
              type="primary"
              v-if="!isEdit"
            >Add</n-button>

            <n-button
              @click="edit"
              ghost
              type="primary"
              v-if="isEdit"
            >Save</n-button>
          </div>
        </n-space>
      </template>
    </n-card>
  </n-modal>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber, NModal, NSpace,
NText } from 'naive-ui';
import { ProjectService, IProjectCostTracking } from '@/project/service';
import { TrackingId } from '@/utils';

import CharacterSelector from '@/components/CharacterSelector.vue';

class Props {
  pid = prop({
    type:     String,
    required: true
  });

  config = prop({
    type:     Object,
    required: true,
  });

  show = prop({
    type:     Boolean,
    required: true,
  });

  isEdit = prop({
    type:     Boolean,
    required: false,
  });
}

@Options({
  components: {
    NButton,
    NCard,
    NDynamicInput,
    NInput,
    NInputNumber,
    NModal,
    NSpace,
    NText,

    CharacterSelector
  }
})
export default class ModalAddCost extends Vue.with(Props) {
  public show_modal: boolean = this.show;

  public costs: { cost: number  | undefined }[] = [{ cost: undefined }];

  public tracking: IProjectCostTracking = <IProjectCostTracking>{};

  public async mounted() {
    this.$watch('config', () => {
      this.costs = [{ cost: this.config.amount }];
      Object.assign(this.tracking, this.config);
    })
  }

  public default_cost() {
    return { cost: undefined };
  }

  public async add() {
    this.tracking.amount = <number>this.costs
      .map(x => x.cost)
      .reduce((prev, curr) => (prev ? prev : 0) + (curr ? curr : 0), 0);

    await ProjectService.tracking_add(this.pid, this.tracking);
    this.close();
  }

  public async edit() {
    this.tracking.amount = <number>this.costs
      .map(x => x.cost)
      .reduce((prev, curr) => (prev ? prev : 0) + (curr ? curr : 0), 0);

    await ProjectService.tracking_edit(this.pid, this.tracking);
    this.close();
  }

  public async remove(id: TrackingId) {
    await ProjectService.tracking_remove(this.pid, id);
    this.close();
  }

  public close() {
    this.costs = [];
    this.costs.push(this.default_cost());

    this.tracking = <IProjectCostTracking>{};

    this.$emit('update:show', false);
  }
}
</script>
