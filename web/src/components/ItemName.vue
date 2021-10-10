<template>
  <n-text>{{ name || 'Unknown ' + tid }}</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { ItemService } from '@/services/item';

class Props {
  tid = prop({
    type:     Number,
    required: true
  })
}

@Options({
  components: {
    NText
  }
})
export default class ItemName extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    this.name = await ItemService.item_name(this.tid);
  }
}
</script>
