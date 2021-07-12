<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy && materials.length > 0">
      <thead>
        <tr>
          <th width="48px"></th>
          <th>Resource</th>
          <th>Amount</th>
          <th>Market value</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="material in materials" :key="material.mid">
          <td><item-icon :id="material.mid" type="icon" /></td>
          <td><name-by-id :id="material.mid" /></td>
          <td>
            <format-number :value="material.amount_eff" />
            (<format-number :value="material.amount_orig" />)
          </td>
          <td><format-number :value="material.cost" /> ISK</td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { BlueprintService, IBlueprintMaterialCost } from '@/services/blueprint';
import { NAlert, NCard, NSkeleton, NTable } from 'naive-ui';
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
    NSkeleton,
    NTable,

    FormatNumber,
    ItemIcon,
    NameById,
  }
})
export default class BlueprintMaterial extends Vue.with(Props) {
  public busy: boolean = false;

  public materials: IBlueprintMaterialCost[] = [];

  public async created() {
    await this.loadMaterial();

    this.$watch('runs', async () => {
      await this.loadMaterial();
    });
  }

  public async loadMaterial() {
    this.busy = true;
    this.materials = await BlueprintService.manufactureMaterial(
      this.iid,
      this.runs,
    );
    this.busy = false;
  }
}
</script>
