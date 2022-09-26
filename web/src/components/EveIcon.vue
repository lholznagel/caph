<template>
  <n-image
    :src="getImage()"
    :width="width"
  />
</template>

<script lang="ts">
import { NImage } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

const BASE_URL = 'https://images.evetech.net';

class Props {
  id = prop({
    type:     Number,
    required: true
  });
  // icon, bp, bpc, render
  type = prop({
    type:     String,
    required: false,
    default:  'icon'
  });
  // optional width
  width = prop({
    type:     Number,
    required: false,
    default:  32
  });

  alliance = prop({
    type:     Boolean,
    required: false,
    default:  false
  });

  character = prop({
    type:     Boolean,
    required: false,
    default:  false
  });

  corporation = prop({
    type:     Boolean,
    required: false,
    default:  false
  });

  item = prop({
    type:     Boolean,
    required: false,
    default:  false
  });
}

@Options({
  components: {
    NImage
  }
})
export default class EveIcon extends Vue.with(Props) {
  public getImage() {
    if (this.alliance) {
      return `${BASE_URL}/alliances/${this.id}/logo?size=1024`;
    } else if (this.character) {
      return `${BASE_URL}/characters/${this.id}/portrait?size=1024`;
    } else if (this.corporation) {
      return `${BASE_URL}/corporations/${this.id}/logo?size=1024`;
    } else {
      return `${BASE_URL}/types/${this.id}/${this.type}?size=1024`;
    }
  }
}
</script>
