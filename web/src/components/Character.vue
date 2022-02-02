<template>
  <n-space align="center">
    <n-image
      class="owner"
      :src="getCharacterPortrait()"
      width="32"
      height="32"
    />
    <span v-if="withText" style="">
      {{ name || id }}
    </span>
  </n-space>
</template>

<script lang="ts">
import { CharacterService } from '@/services/character';
import { NAvatar, NSpace, NImage } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

const BASE_URL_CHAR = 'https://images.evetech.net/characters';

class Props {
  id = prop({
    type: Number,
    required: true
  });
  withText = prop({
    type: Boolean,
    required: false,
  });
}

/// Example usage:
///
/// ```
/// <!-- This will show only the character portrait -->
/// <character :id="character.cid">
///
/// <!-- This will add the character portrait and the name besides it -->
/// <character :id="character.cid" with-text>
/// ```
@Options({
  components: {
    NAvatar,
    NSpace,
    NImage
  }
})
export default class Character extends Vue.with(Props) {
  public name: string = '';

  public created() {
    this.loadName();
  }

  public async loadName() {
    this.name = (await CharacterService.info(this.id)).character;
  }

  public getCharacterPortrait(): string {
    return `${BASE_URL_CHAR}/${this.id}/portrait?size=1024`;
  }
}
</script>
