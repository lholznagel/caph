<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-tree
      v-if="!busy"
      block-line
      :data="trees"
    />
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NSkeleton, NTree } from 'naive-ui';
import { AssetService } from '@/services/asset';
import { ProjectService } from '@/services/project';

class Props {
  // Project id
  pid = prop({
    type: String,
    required: true,
  });
}

@Options({
  components: {
    NSkeleton,
    NTree,
  }
})
export default class ProjectTree extends Vue.with(Props) {
  /*public busy: boolean = false;
  public trees: any[] = [];

  public async created() {
    this.busy = true;

    let products: any[] = await ProjectService.products(this.pid);
    for (let product of products) {
      this.trees.push(await AssetService.blueprint_tree(product.type_id));
    }

    this.busy = false;
  }*/
}
</script>
