<template>
  <n-space align="center">
    <n-image
      class="owner"
      :src="getCharacterPortrait()"
      width="32"
      height="32"
      v-if="!onlyText"
    />
    <span v-if="withText || onlyText" style="">
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
  onlyText = prop({
    type: Boolean,
    required: false,
  });
}

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
    const name = await CharacterService.character_name(this.id);
    this.name = name;
  }

  public getCharacterPortrait(): string {
    return `${BASE_URL_CHAR}/${this.id}/portrait?size=1024`;
  }
}
</script>
