<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-alert v-if="!busy" type="info">
      The following table shows an approximation of the cost to build the project.
      For buying and selling items, the tool uses adjusted values provided by CCP that they use to calculate the cost of producing.
    </n-alert>

    <n-table v-if="!busy">
      <tbody>
        <tr>
          <td width="175"></td>
          <td width="150">Material cost (ME0)</td>
          <td width="5">+</td>
          <td width="100" style="text-align: right"><format-number :value="cost.material_total_cost" /></td>
          <td width="10"></td>
          <td width="10">ISK</td>
          <td width="175"></td>
        </tr>
        <tr>
          <td></td>
          <td>System cost index</td>
          <td>+</td>
          <td style="text-align: right"><format-number :value="cost.system_cost_index" /></td>
          <td>({{ cost.system_cost_index_perc * 100 }}%)</td>
          <td>ISK</td>
          <td></td>
        </tr>
        <tr>
          <td></td>
          <td>Facility Bonus</td>
          <td>-</td>
          <td style="text-align: right"><format-number :value="cost.facility_bonus" /></td>
          <td>({{ cost.facility_bonus_perc }}%)</td>
          <td>ISK</td>
          <td></td>
        </tr>
        <tr>
          <td></td>
          <td>Facility Tax</td>
          <td>+</td>
          <td style="text-align: right"><format-number :value="cost.facility_tax" /></td>
          <td>({{ cost.facility_tax_perc }}%)</td>
          <td>ISK</td>
          <td></td>
        </tr>
        <tr>
          <td></td>
          <td>Production cost</td>
          <td>+</td>
          <td style="text-align: right"><format-number :value="cost.production_cost" /></td>
          <td></td>
          <td>ISK</td>
          <td></td>
        </tr>
        <tr>
          <td></td>
          <td>Total cost</td>
          <td>=</td>
          <td style="text-align: right"><format-number :value="cost.total_cost" /></td>
          <td></td>
          <td>ISK</td>
          <td></td>
        </tr>
        <tr>
          <td></td>
          <td>Sell value</td>
          <td></td>
          <td style="text-align: right"><format-number :value="cost.sell_price" /></td>
          <td></td>
          <td>ISK</td>
          <td></td>
        </tr>
        <tr>
          <td></td>
          <td>Margin</td>
          <td></td>
          <td style="text-align: right"><format-number :value="cost.sell_price - cost.total_cost" /></td>
          <td></td>
          <td>ISK</td>
          <td></td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NAlert, NList, NListItem, NSkeleton, NTable, NThing } from 'naive-ui';
import { ProjectService, IProjectCost } from '@/services/project';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

class Props {
  // Project id
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NList,
    NListItem,
    NSkeleton,
    NTable,
    NThing,

    FormatNumber,
    ItemIcon,
  }
})
export default class ProjectCostApproximation extends Vue.with(Props) {
  public busy: boolean = false;

  public cost: IProjectCost = <IProjectCost>{};

  public async created() {
    this.busy = true;

    this.cost = await ProjectService.project_cost(this.pid);

    this.busy = false;
  }
}
</script>
