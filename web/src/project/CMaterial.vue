<template>
  <div>
    <n-table v-if="entries.length > 0">
      <thead>
        <tr>
          <th width="48px"></th>
          <th width="200px">Name</th>
          <th width="150px">Required</th>
          <th width="150px">Stored</th>
          <th width="150px">Missing</th>
          <th>Progress</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="x in entries" :key="x.type_id">
          <td><item-icon :id="x.type_id" type="icon" /></td>
          <td>{{ x.name }}</td>
          <td><format-number :value="x.quantity" /></td>
          <td><format-number :value="x.stored || 0" /></td>
          <td><format-number :value="missing_materials(x.quantity, x.stored)" /></td>
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

    <n-empty
      v-if="entries.length === 0"
      description="No materials required"
    >
    </n-empty>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NEmpty, NProgress, NTable } from 'naive-ui';
import { ItemGroup } from '@/utils';
import { ProjectService, Project, IRequiredMaterial, IMaterial } from '@/project/service';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

class Props {
  pid = prop({
    type:     String,
    required: true,
  });

  materials = prop({
    type: Array,
    required: false,
  });

  filter = prop({
    type:     String,
    required: false
  });
}

@Options({
  components: {
    NEmpty,
    NProgress,
    NTable,

    FormatNumber,
    ItemIcon,
  }
})
export default class ProjectMaterial extends Vue.with(Props) {
  // Prevent that the type is declared as "undefined"
  public entries: IRequiredMaterial[] = [];

  public async created() {
    let project = await ProjectService.by_id(this.pid);
    await project.init();

    this.entries = this.select_items(project);
  }

  public calc_progress(x: IMaterial): number {
    return Math.ceil(x.stored / (x.quantity / 100) * 100) / 100 || 0;
  }

  public missing_materials(quantity: number, stored: number): number {
    if (stored > quantity) {
      return 0;
    } else {
      return quantity - stored;
    }
  }

  private select_items(project: Project): IRequiredMaterial[] {
    if (this.materials) {
      return <IRequiredMaterial[]>this.materials;
    } else if (!this.filter || this.filter === 'ALL') {
      return project.required_material();
    } else if (this.filter === 'MINERALS') {
      return project.required_material(ItemGroup.Minerals);
    } else if (this.filter === 'ICE') {
      return project.required_material(ItemGroup.Ice);
    } else if (this.filter === 'MOON') {
      return project.required_material(ItemGroup.Moon);
    } else if (this.filter === 'GAS') {
      return project.required_material(ItemGroup.Gas);
    } else if (this.filter === 'SALVAGE') {
      return project.required_material(ItemGroup.Salvage);
    } else if (this.filter === 'PI0') {
      return project.required_material(
        ItemGroup.PI0Solid,
        ItemGroup.PI0Liquid,
        ItemGroup.PI0Organic,
      );
    } else if (this.filter === 'PI1') {
      return project.required_material(ItemGroup.PI1);
    } else if (this.filter === 'PI2') {
      return project.required_material(ItemGroup.PI2);
    } else if (this.filter === 'PI3') {
      return project.required_material(ItemGroup.PI3);
    } else if (this.filter === 'PI4') {
      return project.required_material(ItemGroup.PI4);
    } else {
      return [];
    }
  }
}
</script>
