<template>
  <n-modal
    v-model:show="show"
    :on-update:show="close"
  >
    <n-card
      title="Export material data"
      style="width: 600px"
      :bordered="false"
    >
      <item-export
        :csv-name="pid"
        :format-list="dataFields"
        :format-csv="dataFieldsCsv"
        :items="data"
      />

      <template #action>
        <n-space justify="end">
          <n-button
            @click="close"
            quaternary
          >
            Close
          </n-button>
        </n-space>
      </template>
    </n-card>
  </n-modal>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NModal, NSpace } from 'naive-ui';

import ItemExport from '@/components/ItemExport.vue';

class Props {
  data = prop({
    type:     Object,
    required: true,
  });

  dataFields = prop({
    type:     Array,
    required: false,
    default:  () => ['name', 'quantity']
  });

  dataFieldsCsv = prop({
    type:     Array,
    required: false,
    default:  () => ['type_id', 'name', 'quantity']
  });

  pid = prop({
    type:     String,
    required: true,
  });

  show = prop({
    type:     Boolean,
    required: true,
  });
}

@Options({
  components: {
    NButton,
    NCard,
    NModal,
    NSpace,

    ItemExport,
  }
})
export default class ProjectExport extends Vue.with(Props) {
  public close() {
    this.$emit('update:show', false);
  }
}
</script>
