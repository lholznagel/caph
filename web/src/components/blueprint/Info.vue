<template>
  <n-table>
    <tbody>
      <tr>
        <td>Blueprint Location</td>
        <td><!--location v-if="cbp.location_id" :lid="cbp.location_id" /--></td>
      </tr>
      <tr v-if="bp_product && bp_product[0]">
        <td>Produces</td>
        <td>{{ bp_product[0].name }}</td>
      </tr>
      <tr v-if="bp_product && bp_product[0]">
        <td>Production quantity</td>
        <td>
          {{ bp_product[0].quantity }}
        </td>
      </tr>
      <tr>
        <td>Material Efficiency</td>
        <td>{{ bp.material_efficiency }}</td>
      </tr>
      <tr>
        <td>Time efficiency</td>
        <td>{{ bp.time_efficiency }}</td>
      </tr>
      <tr>
        <td>Runs</td>
        <td>{{ bp.runs === -1 ? '∞' : bp.runs }}</td>
      </tr>
      <tr>
        <td>Type</td>
        <td>
          <n-tag
            :type="bp.quantity === -2 ? 'warning' : 'info'"
          >
            {{ bp.quantity === -2 ? 'Copy' : 'Original' }}
          </n-tag>
        </td>
      </tr>
    </tbody>

    {{ bp_product[0] }}
  </n-table>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NAlert, NCard, NSpace, NStatistic, NTable, NTag } from 'naive-ui';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import Location from '@/components/Location.vue';
import NameById from '@/components/NameById.vue';
import Owner from '@/components/Owner.vue';
import { AssetService, IBlueprintMaterial } from '@/services/asset';

class Props {
  // IBlueprint object
  bp = prop({
    type:     Object,
    required: true,
  });

  bp_product = prop({
    type:     Object,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NCard,
    NSpace,
    NStatistic,
    NTable,
    NTag,

    FormatNumber,
    ItemIcon,
    Location,
    NameById,
    Owner,
  }
})
export default class BlueprintItemInfo extends Vue.with(Props) {
  public mounted() {
    console.log(this.bp_product)
  }
}
</script>
