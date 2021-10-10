<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-table v-if="!busy">
      <thead>
        <tr>
          <th width="24"></th>
          <th width="34"></th>
          <th>Resource</th>
          <th>Amount</th>
        </tr>
      </thead>
      <tbody>
        <template v-for="entry in entries" :key="entry.key">
          <tr>
            <td>
              <n-icon size="22">
                <angle-right
                  style="cursor: pointer"
                  @click="entry.open = true"
                  v-if="!entry.open"
                />
                <angle-down
                  style="cursor: pointer"
                  @click="entry.open = false"
                  v-if="entry.open"
                />
              </n-icon>
            </td>
            <td><item-icon :id="entry.key" type="icon" /></td>
            <td>{{ entry.label }}</td>
            <td><format-number :value="entry.quantity" /></td>
          </tr>
          <tr v-if="entry.open">
            <td colspan="5">
              <h3>Raw materials</h3>
              <project-raw :pids="[entry.key]" />
            </td>
          </tr>
        </template>
      </tbody>
    </n-table>

    <!--n-collapse accordion>
      <template
        v-for="tree in trees"
        :key="tree.key"
      >
        <n-collapse-item
          v-for="children in tree.children"
          :key="children.key"
          :title="children.label"
        >
          <h3>Raw materials</h3>
          <project-raw :pids="[children.key]" />
        </n-collapse-item>
      </template>
    </n-collapse-->
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NCollapse, NCollapseItem, NIcon, NSkeleton, NTable, NTree } from 'naive-ui';
import { AngleDown, AngleRight } from '@vicons/fa';
import { AssetService } from '@/services/asset';
import { ProjectService } from '@/services/project';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import ProjectRaw from '@/components/project/Raw.vue';

class Props {
  // Project id
  pid = prop({
    type: String,
    required: true,
  });
}

@Options({
  components: {
    NCollapse,
    NCollapseItem,
    NIcon,
    NSkeleton,
    NTable,
    NTree,

    AngleDown,
    AngleRight,

    FormatNumber,
    ItemIcon,
    ProjectRaw,
  }
})
export default class ProjectTree extends Vue.with(Props) {
  public busy: boolean = false;

  public entries: any[] = [];

  public async created() {
    this.busy = true;

    let trees = [];
    let products: any[] = await ProjectService.products(this.pid);
    for (let product of products) {
      trees.push(await AssetService.blueprint_tree(product.type_id));
    }

    let resolved = new Map();
    for (let tree of trees) {
      let todo = tree.children;
      while (todo.length > 0) {
        let material = todo.shift();
        if (resolved.has(material.key)) {
          let cur = resolved.get(material.key);
          cur.quantity += material.quantity;
          cur.counter += 1;
          resolved.set(material.key, cur);
        } else {
          material.counter = 1;
          resolved.set(material.key, material);
        }

        if (material.children) {
          todo.push(...material.children);
        }
      }
    }

    let sorted = Array.from(resolved.values());
    console.log(sorted)
    sorted.sort((a, b) => b.counter - a.counter);
    this.entries = sorted;

    this.busy = false;
  }
}
</script>
