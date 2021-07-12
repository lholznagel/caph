<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-collapse>
      <n-collapse-item title="Asteroid" v-if="materials_asteroid.length > 0">
        <blueprint-raw-material-table :materials="materials_asteroid" />
      </n-collapse-item>

      <n-collapse-item title="Commodity" v-if="materials_commodity.length > 0">
        <blueprint-raw-material-table :materials="materials_commodity" />
      </n-collapse-item>

      <n-collapse-item title="Ice" v-if="materials_ice.length > 0">
        <blueprint-raw-material-table :materials="materials_ice" />
      </n-collapse-item>

      <n-collapse-item title="Moon" v-if="materials_moon.length > 0">
        <blueprint-raw-material-table :materials="materials_moon" />
      </n-collapse-item>

      <n-collapse-item title="Planetary Interaction" v-if="materials_pi.length > 0">
        <blueprint-raw-material-table :materials="materials_pi" />
      </n-collapse-item>

      <n-collapse-item title="Salvage" v-if="materials_salvage.length > 0">
        <blueprint-raw-material-table :materials="materials_salvage" />
      </n-collapse-item>
    </n-collapse>
  </div>
</template>

<script lang="ts">
import { BlueprintService, IBlueprintMaterial } from '@/services/blueprint';
import { NAlert, NCard, NCollapse, NCollapseItem, NSkeleton, NTable } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { ProjectService } from '@/services/project';

import BlueprintRawMaterialTable from '@/components/blueprint/RawMaterialTable.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import NameById from '@/components/NameById.vue';

class Props {
  // Array of Object
  // { bpid: number, runs: number }
  bpids = prop({
    type: Array,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NCard,
    NCollapse,
    NCollapseItem,
    NSkeleton,
    NTable,

    BlueprintRawMaterialTable,
    FormatNumber,
    ItemIcon,
    NameById,
  }
})
export default class BlueprintRawMaterial extends Vue.with(Props) {
  public busy: boolean = false;

  public materials_asteroid:  IBlueprintMaterial[] = [];
  public materials_commodity: IBlueprintMaterial[] = [];
  public materials_ice:       IBlueprintMaterial[] = [];
  public materials_moon:      IBlueprintMaterial[] = [];
  public materials_pi:        IBlueprintMaterial[] = [];
  public materials_salvage:   IBlueprintMaterial[] = [];

  public async created() {
    await this.loadMaterial();
  }

  public async loadMaterial() {
    this.busy = true;
    const materials = await ProjectService.rawMaterials(<string>this.$route.params.id);

    this.materials_asteroid  = materials.filter(x => x.group === 'Asteroid');
    this.materials_commodity = materials.filter(x => x.group === 'Commodity');
    this.materials_ice       = materials.filter(x => x.group === 'Ice');
    this.materials_moon      = materials.filter(x => x.group === 'Moon');
    this.materials_pi        = materials.filter(x => x.group === 'Planetary Interaction');
    this.materials_salvage   = materials.filter(x => x.group === 'Salvage');
    this.busy = false;
  }
}
</script>
