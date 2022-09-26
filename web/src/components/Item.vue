<template>
  <div v-if="item">
    <slot :item="item"></slot>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { ItemService, IItem } from '@/services/item';

class Props {
  // Type-Id of the item to resolve
  tid = prop({
    type: Number,
    required: true
  });
}

@Options({
  components: {  }
})
export default class Item extends Vue.with(Props) {
  public item: IItem = <any>{};

  public async created() {
    this.item = await ItemService.resolve_id(this.tid);
  }
}
</script>
