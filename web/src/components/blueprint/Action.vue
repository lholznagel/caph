<template>
  <n-card title="Info">
    <n-space>
      <n-statistic label="Time"><format-number is-time :value="action.time" /></n-statistic>
      <n-statistic
        label="Probability"
        style="margin-left: 100px"
        v-if="action.products && action.products[0].probability"
      >{{ action.products[0].probability }}</n-statistic>
    </n-space>
  </n-card>

  <n-card title="Materials">
    <n-alert type="info" v-if="!action.materials">
      No materials required
    </n-alert>

    <n-table :bordered="false" single-line v-if="action.materials">
      <tbody>
        <tr v-for="material in action.materials" :key="material.mid">
          <td style="width: 64px"><item-icon :id="material.mid" type="icon" /></td>
          <td style="width: 500px"><name-by-id :id="material.mid" /></td>
          <td><format-number :value="Number(material.quantity)" /></td>
        </tr>
      </tbody>
    </n-table>
  </n-card>

  <n-card title="Products">
    <n-alert type="info" v-if="!action.products">
      No products
    </n-alert>

    <n-table :bordered="false" single-line v-if="action.products">
      <tbody>
        <tr v-for="product in action.products" :key="product.mid">
          <td style="width: 64px"><item-icon :id="product.mid" type="bpc" /></td>
          <td style="width: 500px"><name-by-id :id="product.mid" /></td>
          <td><format-number :value="Number(product.quantity)" /></td>
        </tr>
      </tbody>
    </n-table>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NAlert, NCard, NSpace, NStatistic, NTable } from 'naive-ui';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // IBlueprintAction object
  action = prop({
    type: Object,
    required: false,
  });
}

@Options({
  components: {
    NAlert,
    NCard,
    NSpace,
    NStatistic,
    NTable,

    FormatNumber,
    ItemIcon,
    NameById,
  }
})
export default class BlueprintAction extends Vue.with(Props) {  }
</script>
