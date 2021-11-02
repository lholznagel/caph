<template>
  <n-text>{{ name || 'Unknown ' + id }}</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { AssetService } from '@/services/asset';
import { ItemService } from '@/services/item';

class Props {
  // Type-Id of the asset/item to resolve
  tid = prop({
    type: Number,
    required: true
  });

  // Uses the assets API to determine the name if the di
  isAsset = prop({
    type:     Boolean,
    required: false,
    default:  true
  });

  // Uses the item API for determining the ids name
  isItem = prop({
    type:     Boolean,
    required: false,
    default:  false
  });
}

@Options({
  components: {
    NText
  }
})
export default class AssetName extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    if (this.isAsset) {
      this.name = await AssetService.asset_name(this.tid);
    } else if (this.isItem) {
      this.name = await ItemService.item_name(this.tid);
    } else {
      console.error('[AssetName] neither `is-asset` nor `is-item` was given');
    }
  }
}
</script>
