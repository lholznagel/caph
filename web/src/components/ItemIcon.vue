<template>
  <n-image
  :src="getItem()"
  :width="width ? width : 32"
  />
</template>

<script lang="ts">
import { NImage } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

const BASE_URL = 'https://images.evetech.net/types';

class Props {
  id = prop({
    type: Number,
    required: true
  });
  // icon, bp, bpc, render
  type = prop({
    type: String,
    required: true
  });
  // optional width
  width = prop({
    type: Number,
    required: false,
  })
}

@Options({
  components: {
    NImage
  }
})
export default class ItemIcon extends Vue.with(Props) {
  public getItem() {
    return `${BASE_URL}/${this.id}/${this.type}?size=1024`;
  }
}
</script>
