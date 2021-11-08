<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <div v-for="group in groups" :key="group.name">
      <template
        v-if="group.data.length > 0"
      >
        <h3>{{ group.name }}</h3>
        <!--n-collapse-item
          v-if="group.data.length > 0"
          :title="group.name"
        -->
          <n-table>
            <thead>
              <tr>
                <th width="48px"></th>
                <th width="300px">Resource</th>
                <th width="150px">Required</th>
                <th width="150px">Stored</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="x in group.data" :key="x.type_id">
                <td><item-icon :id="x.type_id" type="icon" /></td>
                <td>{{ x.name }}</td>
                <td><format-number :value="x.quantity" /></td>
                <td><format-number :value="x.stored || 0" /></td>
                <td>
                  <n-progress
                    type="line"
                    :percentage="calc_progress(x)"
                    :indicator-placement="'inside'"
                    :status="calc_progress(x) >= 100 ? 'success' : 'default'"
                  />
                </td>
              </tr>
            </tbody>
          </n-table>
        <!--/n-collapse-item-->
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NCollapse, NCollapseItem, NProgress, NSkeleton, NTable, NTree } from 'naive-ui';
import { IAssetBlueprintRaw } from '@/services/asset';
import { ProjectService, IProjectStored, ITemplateMaterial } from '@/services/project';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

class Props {
  pid = prop({
    type:     String,
    required: true,
  });
  // Project id
  pids = prop({
    type:     Array,
    required: true,
  });
}

@Options({
  components: {
    NCollapse,
    NCollapseItem,
    NProgress,
    NSkeleton,
    NTable,
    NTree,

    FormatNumber,
    ItemIcon,
  }
})
export default class ProjectTree extends Vue.with(Props) {
  public busy: boolean = false;

  public groups: IGroupInfo[] = [];

  public async created() {
    this.busy = true;

    let minerals = new Map();
    let ice      = new Map();
    let moons    = new Map();
    let pi       = new Map();
    let salvage  = new Map();
    let misc     = new Map();

    let project   = await ProjectService.project(this.pid);
    let stored    = await ProjectService.project_stored_materials(this.pid);
    let materials = await ProjectService.template_required_materials(project.template);

    for (let material of materials) {
      if (material.group_id === 18) {
        minerals = this.group(material, minerals, stored);
      } else if (material.group_id === 423) {
        ice = this.group(material, ice, stored);
      } else if (material.group_id === 427) {
        moons = this.group(material, moons, stored);
      } else if (material.group_id === 754) {
        salvage = this.group(material, salvage, stored);
      } else if(material.group_id === 1032 || material.group_id === 1033 || material.group_id === 1035) {
        pi = this.group(material, pi, stored);
      } else {
        misc = this.group(material, misc, stored);
      }
    }

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
      data: Array.from(pi.values()),
      name: 'Planetary interaction'
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

  public calc_progress(x: IAssetBlueprintRaw): number {
    return Math.ceil(x.stored / (x.quantity / 100) * 100) / 100 || 0;
  }

  private group(
    material: ITemplateMaterial,
    map:      Map<number, ITemplateMaterial>,
    stored:   IProjectStored[]
  ): Map<number, ITemplateMaterial> {
    if (map.has(material.type_id)) {
      let cur = <ITemplateMaterial>map.get(material.type_id);
      cur.quantity += material.quantity;
      map.set(material.type_id, cur);
    } else {
      let stored_material = stored
        .find(x => x.type_id === material.type_id);

      if (stored_material) {
        material.stored = stored_material.quantity;
      }
      map.set(material.type_id, material);
    }
    return map;
  }
}

interface IGroupInfo {
  data: IAssetBlueprintRaw[];
  name: string;
}
</script>
