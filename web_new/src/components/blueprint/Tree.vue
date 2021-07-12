<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-tree
      block-line
      :data="tree"
      v-if="!busy"
    />
  </div>
</template>

<script lang="ts">
import { BlueprintService, IBlueprintTree } from '@/services/blueprint';
import { NSkeleton, NTree } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

class Props {
  // Blueprint item id object
  bid = prop({
    type: Number,
    required: true,
  });
}

@Options({
  components: {
    NSkeleton,
    NTree,
  }
})
export default class BlueprintTree extends Vue.with(Props) {
  public busy: boolean = false;

  public tree: IBlueprintTree[] = [];

  public async created() {
    this.busy = true;
    this.tree = [await BlueprintService.tree(this.bid)];
    this.busy = false;
  }
}
</script>

