<template>
  <n-select
    v-model:value="value"
    :options="options"
    filterable
  />
</template>

<script lang="ts">
import { NSelect } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { AssetService } from '@/services/asset';

class Props {
  filter = prop({
    type:     Object,
    required: false
  });
}

@Options({
  components: {
    NSelect
  }
})
export default class AssetSelector extends Vue.with(Props) {
  public options: any[] = [];

  public value: number | null = null;

  public async created() {
    let assets = await AssetService.assets(this.filter || {});

    for (let asset of assets) {
      for (let item of asset.item_ids) {
        let asset_name = await AssetService
          .asset_name(item)
          .catch(_ => {});

        this.options.push({
          label: `${asset.name} (${asset_name})`,
          value: item
        });
      }
    }
  }
}
</script>
