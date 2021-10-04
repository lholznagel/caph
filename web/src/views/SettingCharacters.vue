<template>
  <n-card title="Settings">
    <n-skeleton text v-if="busy" :repeat="6" />

    <n-grid x-gap="12" :cols="5" v-if="!busy">
      <n-grid-item>
        <div>
          <n-image width="340" :src="character.character_icon" />
          <span>{{ character.character }}</span>
        </div>
      </n-grid-item>
      <n-grid-item v-for="alias in characterAlts" :key="alias.character_id">
        <n-image width="340" :src="alias.character_icon" />
        <span>{{ alias.character }}</span>
      </n-grid-item>
   </n-grid>

    <n-button @click="addAlt">Add character</n-button>
  </n-card>
</template>

<script lang="ts">
import {CharacterService, DEFAULT_CHARACTER, ICharacter} from '@/services/character';
import { NButton, NCard, NImage, NSkeleton, NGrid, NGridItem } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

@Options({
  components: {
    NButton,
    NCard,
    NGrid,
    NGridItem,
    NImage,
    NSkeleton
  }
})
export default class Settings extends Vue {
  public busy: boolean = false;

  public character: ICharacter        = DEFAULT_CHARACTER;
  public characterAlts: ICharacter[] = [];

  public async created() {
    this.busy = true;

    this.character     = await CharacterService.info();
    this.characterAlts = await CharacterService.alts();

    this.busy = false;
  }

  public addAlt() {
    window.location.href = `/api/auth/login/alt`;
  }
}
</script>
