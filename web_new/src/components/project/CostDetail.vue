<template>
  <n-table>
    <tbody>
      <tr>
        <td>Material cost (ME0)</td>
        <td><format-number :value="cost.material_total_cost" /> ISK</td>
      </tr>
      <tr>
        <td>System cost index</td>
        <td>
          <format-number :value="cost.system_cost_index" /> ISK
          ({{ cost.system_cost_index_perc * 100 }}%)
        </td>
      </tr>
      <tr>
        <td>Facility Bonus</td>
        <td>
          - <format-number :value="cost.facility_bonus" /> ISK
          ({{ cost.facility_bonus_perc }}%)
        </td>
      </tr>
      <tr>
        <td>Facility Tax</td>
        <td>
          <format-number :value="cost.facility_tax" /> ISK
          ({{ cost.facility_tax_perc }}%)
        </td>
      </tr>
      <tr>
        <td>Production cost</td>
        <td><format-number :value="cost.production_cost" /> ISK</td>
      </tr>
      <tr>
        <td>Total cost</td>
        <td><format-number :value="cost.total_cost" /> ISK</td>
      </tr>
      <tr>
        <td>Sell value</td>
        <td><format-number :value="cost.sell_price" /> ISK</td>
      </tr>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { NAlert, NCard, NStatistic, NTable } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // IProjectCost Object
  cost = prop({
    type: Object,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NCard,
    NStatistic,
    NTable,

    FormatNumber,
    ItemIcon,
    NameById,
  }
})
export default class ProjectCostTable extends Vue.with(Props) {  }
</script>
