<template>
  <span v-if="character">
    <slot :info="character"></slot>
  </span>
</template>

<script lang="ts">
import { CharacterService, ICharacterInfo } from '@/services/character';
import { Options, Vue, prop } from 'vue-class-component';

class Props {
  id = prop({
    type: Number,
    required: true
  });
}

@Options({
  components: {  }
})
export default class Character extends Vue.with(Props) {
  public character: ICharacterInfo = <any>{};

  public async created() {
    this.character = await CharacterService.info(this.id);
  }
}
</script>
