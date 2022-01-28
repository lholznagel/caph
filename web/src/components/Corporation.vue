<template>
  <n-image
    class="owner"
    :src="getCorporationLogo()"
    width="32"
    height="32"
  />
</template>

<script lang="ts">
import { NImage } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

const BASE_URL_CHAR = 'https://images.evetech.net/corporations';

class Props {
  id = prop({
    type: Number,
    required: true
  });
}

@Options({
  components: {
    NImage
  }
})
export default class Corporation extends Vue.with(Props) {
  public name: string = '';

  public getCorporationLogo(): string {
    return `${BASE_URL_CHAR}/${this.id}/logo?size=1024`;
  }
}
</script>
