<template>
  <n-table>
    <thead>
      <tr>
        <th width="48px"></th>
        <th width="200px">Name</th>
        <th width="150px">Required</th>
        <th width="150px">Stored</th>
        <th width="150px">Missing</th>
        <th>Progress</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="x in entries" :key="x.type_id">
        <td><item-icon :id="x.type_id" type="icon" /></td>
        <td>{{ x.name }}</td>
        <td><format-number :value="x.quantity" /></td>
        <td><format-number :value="x.stored || 0" /></td>
        <td><format-number :value="missing_materials(x.quantity, x.stored)" /></td>
        <td>
          <n-progress
            type="line"
            :percentage="calc_progress(x)"
            :indicator-placement="'inside'"
            :status="calc_progress(x) >= 100 ? 'success' : 'default'"
          />
        </td>
      </tr>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NProgress, NTable } from 'naive-ui';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import { TypeId } from '@/utils';

class Props {
  // IMaterial
  materials = prop({
    type:     Array,
    required: true,
  });
}

@Options({
  components: {
    NProgress,
    NTable,

    FormatNumber,
    ItemIcon,
  }
})
export default class ProjectMaterial extends Vue.with(Props) {
  // Prevent that the type is declared as "undefined"
  public entries: IMaterial[] = <IMaterial[]>this.materials;

  public calc_progress(x: IMaterial): number {
    return Math.ceil(x.stored / (x.quantity / 100) * 100) / 100 || 0;
  }

  public missing_materials(quantity: number, stored: number): number {
    if (stored > quantity) {
      return 0;
    } else {
      return quantity - stored;
    }
  }
}

export interface IMaterial {
  type_id:  TypeId;
  name:     string;
  quantity: number;
  stored:   number;
  bonus:    number;
}
</script>
