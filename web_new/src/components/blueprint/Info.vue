<template>
  <n-table>
    <tbody>
      <tr>
        <td>Blueprint Location</td>
        <td><location v-if="cbp.location_id" :lid="cbp.location_id" /></td>
      </tr>
      <tr>
        <td>Produces</td>
        <td>
          <name-by-id
            v-if="bp.manufacture.products"
            :id="bp.manufacture.products[0].mid"
          />
        </td>
      </tr>
      <tr>
        <td>Production quantity</td>
        <td v-if="bp.manufacture.products">
          {{ bp.manufacture.products[0].quantity }}
        </td>
      </tr>
      <tr>
        <td>Material Efficiency</td>
        <td>{{ cbp.material_efficiency }}</td>
      </tr>
      <tr>
        <td>Time efficiency</td>
        <td>{{ cbp.time_efficiency }}</td>
      </tr>
      <tr>
        <td>Runs</td>
        <td>{{ cbp.runs === -1 ? '∞' : cbp.runs }}</td>
      </tr>
      <tr>
        <td>Type</td>
        <td>
          <n-tag
            :type="cbp.quantity === -2 ? 'warning' : 'info'"
          >
            {{ cbp.quantity === -2 ? 'Copy' : 'Original' }}
          </n-tag>
        </td>
      </tr>
      <tr>
        <td>Owner</td>
        <td><owner v-if="cbp.user_id" :ids="[cbp.user_id]" /></td>
      </tr>
    </tbody>
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

class Props {
  // IBlueprint object
  bp = prop({
    type: Object,
    required: true,
  });
  // ICharacterBlueprint object
  cbp = prop({
    type: Object,
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
export default class BlueprintItemInfo extends Vue.with(Props) {  }
</script>
