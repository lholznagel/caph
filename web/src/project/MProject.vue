<template>
  <n-modal
    v-model:show="show_modal"
  >
    <n-card
      :title="isEdit ? 'Edit project' : 'New project'"
      style="width: 600px"
      :bordered="false"
    >
      <n-space vertical>
        <n-text>Project name</n-text>
        <n-input v-model:value="config.name" placeholder="Name" />

        <n-text>Containers</n-text>
        <n-dynamic-input
          v-model:value="containers"
          :on-create="add_container"
          :min="1"
          #="{ value }"
        >
          <div style="width: 100%;">
            <div style="display: flex; align-items: center;">
              <asset-selector v-model:value="value.item_id" :filter="{ group: 448 }" />
            </div>
          </div>
        </n-dynamic-input>

        <n-text>Items to produce</n-text>
        <n-dynamic-input
          v-model:value="config.products"
          :on-create="add_item_to_produce"
          :min="1"
          #="{ value }"
        >
          <div style="width: 100%;">
            <div style="display: flex; align-items: center;">
              <n-input-number
                v-model:value="value.count"
                style="margin-right: 5px"
              />
              <n-select
                :options="buildable_items"
                v-model:value="value.type_id"
                placeholder="Select Item"
                filterable
              />
            </div>
          </div>
        </n-dynamic-input>
        <resolve v-model="config.products" />
      </n-space>

      <template #action>
        <n-space justify="space-between">
          <n-button @click="close">Close</n-button>

          <n-button
            v-if="!isEdit"
            @click="create_project"
            type="primary"
            ghost
          >Create</n-button>

          <n-button
            v-if="isEdit"
            @click="edit_project"
            type="primary"
            ghost
          >Edit</n-button>
        </n-space>
      </template>
    </n-card>
  </n-modal>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber,
NModal, NSelect, NSpace, NText, SelectOption } from 'naive-ui';

import {ProjectService} from './service';
import {AssetService} from '@/services/asset';

import AssetSelector from '@/components/AssetSelector.vue';
import Resolve from '@/components/Resolve.vue';
import {ItemId} from '@/utils';

class Props {
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
    NSelect,
    NSpace,
    NText,

    AssetSelector,
    Resolve
  }
})
export default class ProjectSettings extends Vue.with(Props) {
  public containers: { item_id: ItemId }[] = [];
  public buildable_items: SelectOption[]   = [];

  public show_modal: boolean = this.show;
  public form_ref: any = {};

  public async created() {
    // TODO: replace with caching version
    this.buildable_items = (await AssetService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });

    this.$watch('show_modal', () => this.$emit('update:show', this.show_modal));
    this.$watch('show', () => this.show_modal = this.show);
    this.$watch('config', () => {
      if (this.config.containers && this.config.containers.length > 0) {
        this.containers = this.config
          .containers
          .map((x: any) => { return { item_id: x }; });
      }
    });
  }

  public async create_project() {
    // The selector adds it as an object, we want it as an array
    this.config.containers = this.containers.map(x => x.item_id);
    await ProjectService.create(this.config);
    this.close();
  }

  public async edit_project() {
    // The selector adds it as an object, we want it as an array
    this.config.containers = this.containers.map(x => x.item_id);
    await ProjectService.edit(this.config.id, this.config);
    this.close();
  }

  public close() {
    this.$emit('update:show', false);
  }


  public add_item_to_produce() {
    return {
      type_id: undefined,
      count:   1,
    }
  }

  public add_container() {
    return {
      item_id: undefined,
    }
  }
}
</script>
