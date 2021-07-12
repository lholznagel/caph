<template>
  <n-avatar
    :src="getCharacterPortrait(id)"
    :key="id"
    v-for="id in ids"
    size="small"
  />
</template>

<script lang="ts">
import { NAvatar } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

const BASE_URL_CHAR = 'https://images.evetech.net/characters';
const BASE_URL_CORP = 'https://images.evetech.net/corporations'

class Props {
  ids = prop({
    type: Array,
    required: true
  });
}

@Options({
  components: {
    NAvatar
  }
})
export default class Owner extends Vue.with(Props) {
  public getCharacterPortrait(id: number): string {
    // FIXME: Hack!
    if (id === 692480993) {
      return `${BASE_URL_CORP}/${id}/logo?size?1024`;
    }

    return `${BASE_URL_CHAR}/${id}/portrait?size?1024`;
  }
}
</script>
