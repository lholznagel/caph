<template>
  <div>
    <div v-for="group in groups" :key="group.name">
      <template
        v-if="group.data.length > 0"
      >
        <h2 style="padding-left: 10px">{{ group.name }}</h2>
        <p-material :materials="group.data" />
      </template>
    </div>

    <p-material-export
      :data="export_items"
      :pid="$route.params.pid"
      v-model:show="show_export"
    />
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton } from 'naive-ui';
import { IProjectMaterial, ProjectService } from '@/project/service';

import PMaterial, {IMaterial} from '@/project/CMaterial.vue';
import PMaterialExport from '@/project/MExport.vue';
import ItemExport from '@/components/ItemExport.vue';

class Props {
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NButton,

    PMaterial,
    PMaterialExport,
    ItemExport,
  }
})
export default class ProjectTree extends Vue.with(Props) {
  public busy: boolean        = false;
  public show_export: boolean = false;

  public groups: IGroupInfo[] = [];
  public export_items: any[]  = [];

  public async created() {
    this.busy = true;

    let minerals = new Map();
    let ice      = new Map();
    let moons    = new Map();
    let pi0      = new Map();
    let pi1      = new Map();
    let pi2      = new Map();
    let pi3      = new Map();
    let pi4      = new Map();
    let salvage  = new Map();
    let gas      = new Map();
    let misc     = new Map();

    let project = await ProjectService.by_id(this.pid);
    let stored  = project.stored_materials;

    this.export_items = project
      .raw_materials
      .sort((a, b) => a.quantity - b.quantity);

    for (let material of project.raw_materials) {
      switch(material.group_id) {
        case 18:
          minerals = this.group(material, minerals, stored);
          continue;
        case 423:
          ice = this.group(material, ice, stored);
          continue;
        case 427:
          moons = this.group(material, moons, stored);
          continue;
        case 711:
          gas = this.group(material, gas, stored);
          continue;
        case 754:
          salvage = this.group(material, salvage, stored);
          continue;
        case 1032:
        case 1033:
        case 1035:
          pi0 = this.group(material, pi0, stored);
          continue;
        case 1042:
          pi1 = this.group(material, pi1, stored);
          continue;
        case 1034:
          pi2 = this.group(material, pi2, stored);
          continue;
        case 1040:
          pi3 = this.group(material, pi3, stored);
          continue;
        case 1041:
          pi4 = this.group(material, pi4, stored);
          continue;
        default:
          misc = this.group(material, misc, stored);
          continue;
      }
    };

    this.groups.push({
      data: Array.from(minerals.values()),
      name: 'Minerals'
    });
    this.groups.push({
      data: Array.from(ice.values()),
      name: 'Ice'
    });
    this.groups.push({
      data: Array.from(moons.values()),
      name: 'Moon'
    });
    this.groups.push({
      data: Array.from(gas.values()),
      name: 'Gas'
    });
    this.groups.push({
      data: Array.from(pi0.values()),
      name: 'Planetary interaction Raw'
    });
    this.groups.push({
      data: Array.from(pi1.values()),
      name: 'Planetary interaction Tier 1'
    });
    this.groups.push({
      data: Array.from(pi2.values()),
      name: 'Planetary interaction Tier 2'
    });
    this.groups.push({
      data: Array.from(pi3.values()),
      name: 'Planetary interaction Tier 3'
    });
    this.groups.push({
      data: Array.from(pi4.values()),
      name: 'Planetary interaction Tier 4'
    });
    this.groups.push({
      data: Array.from(salvage.values()),
      name: 'Salvage'
    });
    this.groups.push({
      data: Array.from(misc.values()),
      name: 'Misc',
    });

    this.busy = false;
  }

  private group(
    material: IProjectMaterial,
    map:      Map<number, IMaterial>,
    stored:   IProjectMaterial[]
  ): Map<number, IMaterial> {
    if (map.has(material.type_id)) {
      let cur = <IMaterial>map.get(material.type_id);
      cur.quantity += material.quantity;
      map.set(material.type_id, cur);
    } else {
      let entry = {
        type_id:  material.type_id,
        name:     material.name,
        quantity: material.quantity,
        group_id: material.group_id,
        stored:   0,
        bonus:    material.material
      };

      let stored_material = stored
        .find(x => x.type_id === material.type_id);
      if (stored_material) {
        entry.stored = stored_material.quantity;
      }
      map.set(material.type_id, entry);
    }
    return map;
  }
}

interface IGroupInfo {
  data: IMaterial[];
  name: string;
}
</script>
