<template>
  <n-table v-if="materials.length > 0">
    <thead>
      <tr>
        <th width="48px"></th>
        <th>Resource</th>
        <th>Amount</th>
        <th>Group</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="material in materials" :key="material.mid">
        <td><item-icon :id="material.mid" type="icon" /></td>
        <td><name-by-id :id="material.mid" /></td>
        <td><format-number :value="material.quantity" /></td>
        <td>{{ material.group || '' }}</td>
      </tr>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { NTable } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // Array of Object
  // { mid: number, group: number, quantity: number }
  materials = prop({
    type: Array,
    required: true,
  });
}

@Options({
  components: {
    NTable,

    FormatNumber,
    ItemIcon,
    NameById,
  }
})
export default class BlueprintMaterialTable extends Vue.with(Props) {  }
</script>
