<template>
  <n-text>{{ name || 'Unknown ' + id }}</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { AssetService } from '@/services/asset';

class Props {
  id = prop({
    type:     Number,
    required: true
  })
}

@Options({
  components: {
    NText
  }
})
export default class AssetName extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    this.name = await AssetService.asset_name(this.id);
  }
}
</script>
