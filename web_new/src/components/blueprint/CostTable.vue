<template>
  <n-table v-if="bp_cost">
    <thead>
      <tr>
        <th>Type</th>
        <th>Cost</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>Material cost (ME0)</td>
        <td><format-number :value="bp_cost.material_total_cost" /> ISK</td>
      </tr>
      <tr>
        <td>System cost index</td>
        <td>
          <format-number :value="bp_cost.system_cost_index" /> ISK
          ({{ bp_cost.system_cost_index_perc * 100 }}%)
        </td>
      </tr>
      <tr>
        <td>Facility Bonus</td>
        <td>
          - <format-number :value="bp_cost.facility_bonus" /> ISK
          ({{ bp_cost.facility_bonus_perc }}%)
        </td>
      </tr>
      <tr>
        <td>Facility Tax</td>
        <td>
          <format-number :value="bp_cost.facility_tax" /> ISK
          ({{ bp_cost.facility_tax_perc }}%)
        </td>
      </tr>
      <tr>
        <td>Production cost</td>
        <td><format-number :value="bp_cost.production_cost" /> ISK</td>
      </tr>
      <tr>
        <td>Total cost</td>
        <td><format-number :value="bp_cost.total_cost" /> ISK</td>
      </tr>
      <tr>
        <td>Sell value</td>
        <td><format-number :value="bp_cost.sell_price" /> ISK</td>
      </tr>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { BlueprintService, IBlueprintCost } from '@/services/blueprint';
import { NAlert, NCard, NStatistic, NTable } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // Blueprint item id object
  iid = prop({
    type: Number,
    required: true,
  });
  // System id where things n stuff are produced
  sid = prop({
    type: Number,
    required: true
  });
  // Number of runs
  runs = prop({
    type: Number,
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
export default class BlueprintCost extends Vue.with(Props) {
  public bp_cost: IBlueprintCost | null = null;

  public async created() {
    await this.loadBlueprint();

    this.$watch('sid', async () => {
      await this.loadBlueprint();
    });
    this.$watch('runs', async () => {
      await this.loadBlueprint();
    });
  }

  public async loadBlueprint() {
    this.bp_cost = await BlueprintService.manufactureCost(
      this.iid,
      this.sid,
      this.runs,
    );
  }
}
</script>
