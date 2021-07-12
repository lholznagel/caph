<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy && materials.length > 0">
      <thead>
        <tr>
          <th width="48px"></th>
          <th>Resource</th>
          <th>Amount</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="material in materials" :key="material.mid">
          <td><item-icon :id="material.mid" type="icon" /></td>
          <td><name-by-id :id="material.mid" /></td>
          <td><format-number :value="material.quantity" /></td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { NAlert, NCard, NSkeleton, NTable } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { ProjectService } from '@/services/project';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // Project id
  pid = prop({
    type: String,
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
export default class ProjectMaterial extends Vue.with(Props) {
  public busy: boolean = false;

  public materials: any[] = [];

  public async created() {
    this.busy = true;
    this.materials = (await ProjectService.materials(this.pid))
      .sort((a: any, b: any) => a.mid - b.mid);
    this.busy = false;
  }
}
</script>
