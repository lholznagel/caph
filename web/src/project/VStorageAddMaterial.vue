<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <p-header />

      <n-card v-if="!busy">
        <n-space vertical>
          <div>
            <h3 style="margin-bottom: 0">Mode</h3>
            <n-text>
              When the mode is "set" the currently stored materials are
              overwrite with the new materials. In "add" mode it will add the
              given materials.
            </n-text>
            set, add, sub
            <n-select :options="modes" v-model:value="mode" />
          </div>

          <div>
            <h3 style="margin-bottom: 0">Materials</h3>

            <n-text style="margin-bottom: 5px;">
              Materials that are either set, added or subtracted.
            </n-text>

            <n-dynamic-input
              v-model:value="materials"
              :on-create="add_material"
              :min="1"
              #="{ value }"
            >
              <div style="width: 100%;">
                <div style="display: flex; align-items: center;">
                  <n-input-number
                    v-model:value="value.quantity"
                    style="margin-right: 5px;"
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

            <resolve style="margin-top: 10px" v-model="resolved_items" />
          </div>
        </n-space>

        <template #action>
          <n-space justify="end">
            <n-button
              @click="$router.back()"
              quaternary
            >Cancel</n-button>

            <n-button
              :disabled="materials.length === 0"
              @click="add"
              type="info"
            >
              Add
            </n-button>
          </n-space>
        </template>
      </n-card>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber, NPageHeader, NSelect, NSpace, NText, NTable } from 'naive-ui';
import { IModify, Service } from '@/project/service';
import { ProjectId } from '@/project/project';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import { ItemService } from '@/services/item';

import Resolve from '@/components/Resolve.vue';

import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/wrapper/Project.vue';

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
    NTable,
    NText,

    Resolve,

    PHeader,
    WProject
  }
})
export default class AddStorage extends Vue {
  public buildable_items: { value: number; label: string}[] = [];
  public resolved_items: IModify[] = [];

  public mode: string = 'SET';
  public modes = [{
    label: 'Set',
    value: 'SET'
  }, {
    label: 'Add',
    value: 'ADD'
  }];

  public materials: IModify[] = [];

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );

    // TODO: replace with caching version
    // Get all available materials and filter out blueprints
    this.buildable_items = (await ItemService.components())
      .filter(x => x.name.indexOf('Blueprint') === -1)
      .map(x => {
        return {
          label: x.name,
          value: x.type_id
        }
      });

    this.$watch('resolved_items', () => {
      if (this.resolved_items.length === 0) { return; }

      this.resolved_items.forEach(x => this.materials.push(x));
      this.resolved_items = [];
    }, {
      deep: true
    });
  }

  public add_material() {
    return {
      type_id:  undefined,
      quantity: 1,
    }
  }

  public async add() {
    await Service.modify(<ProjectId>this.$route.params.pid, this.materials, this.mode);
    this.$router.back();
  }
}
</script>
