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
        :format-list="data_fields"
        :format-csv="data_fields_csv"
        :items="data"
      />

      <template #action>
        <n-button @click="close">Close</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NModal } from 'naive-ui';

import ItemExport from '@/components/ItemExport.vue';

class Props {
  data = prop({
    type:     Object,
    required: true,
  });

  data_fields = prop({
    type:     Array,
    required: false,
    default:  () => ['name', 'quantity']
  });

  data_fields_csv = prop({
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

    ItemExport,
  }
})
export default class ProjectExportMaterial extends Vue.with(Props) {
  public close() {
    this.$emit('update:show', false);
  }
}
</script>
