<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

    <n-tree
      block-line
      :data="trees"
    />
  </div>
</template>

<script lang="ts">
import { IBlueprintTree } from '@/services/blueprint';
import { ProjectService } from '@/services/project';
import { NSkeleton, NTree } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

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
export default class BlueprintTree extends Vue.with(Props) {
  public busy: boolean = false;

  public trees: IBlueprintTree[] = [];

  public async created() {
    this.busy = true;
    this.trees = await ProjectService.tree(this.pid);
    this.busy = false;
  }
}
</script>

