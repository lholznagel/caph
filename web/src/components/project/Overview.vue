<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-list v-if="!busy">
      <n-list-item>
        <n-thing title="Products">
          <n-table>
            <thead>
              <tr>
                <th></th>
                <th>Count</th>
                <th>Stored</th>
                <th>Name</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="product in project.products" :key="product.type_id">
                <td><item-icon :id="product.type_id" type="icon" /></td>
                <td>{{ product.count }}</td>
                <td>{{ stored_material(product.type_id) }}</td>
                <td><item-name :tid="product.type_id" /></td>
              </tr>
            </tbody>
          </n-table>
        </n-thing>
      </n-list-item>
    </n-list>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NList, NListItem, NSkeleton, NTable, NThing } from 'naive-ui';
import { ProjectService, IProject, IProjectStoredMaterial } from '@/services/project';

import ItemIcon from '@/components/ItemIcon.vue';
import ItemName from '@/components/ItemName.vue';

class Props {
  // Project id
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NList,
    NListItem,
    NSkeleton,
    NTable,
    NThing,

    ItemIcon,
    ItemName,
  }
})
export default class ProjectOverview extends Vue.with(Props) {
  public busy: boolean = false;

  public project: IProject = this.default_project();

  public materials: IProjectStoredMaterial[] = [];

  public async created() {
    this.busy = true;

    this.project = await ProjectService.project(<string>this.$route.params.pid);
    this.project.products = await ProjectService.products(<string>this.$route.params.pid);
    this.materials = await ProjectService.required_raw_materials(this.pid);

    this.busy = false;
  }

  public stored_material(tid: number | null): number {
    let material = this.materials
      .find(x => x.type_id === <number>tid);
    return material ? material.stored : 0;
  }

  private default_project(): IProject {
    return {
      name: '',
      containers: [],
      products: []
    }
  }
}
</script>
