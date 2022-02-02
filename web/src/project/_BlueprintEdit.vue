<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <p-header />

      <n-card>
        <n-space vertical>
          <n-text>Blueprint type</n-text>
          <n-select
            v-model:value="original"
            placeholder="Blueprint type"
            :options="bp_select"
          />

          <n-text>Stored quantity</n-text>
          <n-input-number
            v-model:value="entry.quantity"
            placeholder="Number of blueprints stored"
          />

          <n-text
            v-show="original >= 0"
          >
            Runs
          </n-text>
          <n-input-number
            v-show="original >= 0"
            v-model:value="entry.runs"
            placeholder="Blueprint copy runs"
          />

          <n-text>Material efficiency</n-text>
          <n-input-number
            v-model:value="entry.me"
            placeholder="Material efficiency. Between 0 and 10"
            :disabled="original === -2"
          />

          <n-text>Time efficiency</n-text>
          <n-input-number
            v-model:value="entry.te"
            placeholder="Time efficiency. Between 0 and 20"
            :disabled="original === -2"
          />
        </n-space>

        <template #action>
          <n-space justify="end">
            <n-button
              @click="$router.back()"
              quaternary
            >Cancel</n-button>

            <n-button
              :disabled="!entry.quantity"
              @click="save"
              type="info"
            >Save</n-button>
          </n-space>
        </template>
      </n-card>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NDynamicInput, NInput, NInputNumber, NPageHeader, NSelect, NSpace, NText, SelectOption } from 'naive-ui';
import { Service, IStorageEntry } from '@/project/service';
import { ProjectId } from '@/project/project';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

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

    PHeader,
    WProject
  }
})
export default class AddStorage extends Vue {
  public entry: IStorageEntry = <IStorageEntry>{  };
  public original: number | undefined = -1;
  public stored: IStorageEntry[] = [];

  public bp_select = [{
    label: 'Blueprint Original',
    value: -1
  }, {
    label: 'Blueprint Copy',
    value: 0
  }, {
    label: 'Reaction',
    value: -2
  }];

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );
    this.stored = await Service.stored(<ProjectId>this.$route.params.pid);

    let response = await Service.storage_by_id(
      <string>this.$route.params.pid,
      parseInt(<string>this.$route.params.bid) || 0
    );
    if (response) {
      this.entry = response;
      if (response.runs && response.runs > -1) {
        this.original = -2;
      }
    } else {
      this.entry.type_id = parseInt(<string>this.$route.params.bid);
    }
  }

  public async save() {
    if (this.original) {
      this.entry.runs = -1;
    }
    await Service.set_storage(<ProjectId>this.$route.params.pid, [this.entry]);
    this.$router.back();
  }

  public stored_quantity(type_id: number): number {
    let stored = this.stored.find(x => x.type_id === type_id);
    return stored ? stored.quantity : 0;
  }
}
</script>
