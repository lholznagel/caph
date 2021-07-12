<template>
  <div>
    <n-space style="margin-bottom: 25px">
      <n-progress
      type="circle"
      :percentage="material.stored / (material.quantity / 100)"
        v-for="material in product.materials" :key="material.mid"
      >
        <item-icon :id="material.mid" type="icon" :width="64" />
      </n-progress>
    </n-space>

    <n-table>
      <thead>
        <tr>
          <th width="48px"></th>
          <th>Name</th>
          <th></th>
          <th>Required</th>
          <th>Stored</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="material in product.materials" :key="material.mid">
          <td><item-icon :id="material.mid" type="icon" /></td>
          <td><name-by-id :id="material.mid" /></td>
          <td>
            <n-tag
              type="success"
              v-if="material.stored >= material.quantity"
            >
              All materials stored
            </n-tag>
          </td>
          <td><format-number :value="material.quantity" /></td>
          <td><format-number :value="material.stored" /></td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { NAlert, NCard, NStatistic, NTable, NTag, NProgress, NSpace, } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // IProjectCost Object
  product = prop({
    type: Object,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NCard,
    NProgress,
    NStatistic,
    NTable,
    NTag,
    NSpace,

    FormatNumber,
    ItemIcon,
    NameById,
  }
})
export default class ProjectRequiredProduct extends Vue.with(Props) {  }
</script>
