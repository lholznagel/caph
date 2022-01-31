<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Characters
        </h1>
      </template>

      <template #extra>
        <n-button @click="addAlt" type="primary" ghost>Add character</n-button>
      </template>
    </n-page-header>

    <n-card>
      <n-skeleton text v-if="busy" :repeat="6" />

      <n-grid x-gap="12" :cols="6" v-if="!busy">
        <n-grid-item v-for="character in characters">
          <div>
            <n-image width="200" :src="character.character_icon" />

            <n-space vertical>
              <span>{{ character.character }}</span>
              <span>
                <n-space>
                  <n-button
                    @click="refresh(character.character_id)"
                    :disabled="refresh_map[character.character_id]"
                    :loading="refresh_map[character.character_id]"
                  >
                    Refresh
                  </n-button>
                  <n-button
                    @click="remove(character.character_id)"
                    type="error"
                    ghost
                  >
                    Remove
                  </n-button>
                </n-space>
              </span>
            </n-space>
          </div>
        </n-grid-item>
     </n-grid>
    </n-card>
  </div>
</template>

<script lang="ts">
import {CharacterService, ICharacter} from '@/services/character';
import { events } from '@/main';
import { ROUTE } from '@/event_bus';
import axios from 'axios';
import { NButton, NCard, NImage, NGrid, NGridItem, NSkeleton, NSpace,
NPageHeader } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

@Options({
  components: {
    NButton,
    NCard,
    NGrid,
    NGridItem,
    NImage,
    NPageHeader,
    NSkeleton,
    NSpace,
  }
})
export default class Settings extends Vue {
  public busy: boolean = false;

  public characters: ICharacter[] = [];

  public refresh_map: { [key: number]: boolean } = {}

  public async created() {
    console.log(this.$route.params);

    events.$emit(
      ROUTE,
      `settings_characters`
    );

    this.busy = true;

    await this.load();

    this.busy = false;
  }

  public addAlt() {
    window.location.href = `/api/auth/login/alt`;
  }

  public async refresh(cid: number) {
    this.refresh_map[cid] = true;

    await axios.get(`/api/character/${cid}/refresh`);

    this.refresh_map[cid] = false;
  }

  public async remove(cid: number) {
    await CharacterService.remove(cid);
    await this.load();
  }

  private async load() {
    this.characters = [];
    let character      = await CharacterService.info();
    let character_alts = await CharacterService.alts();
    this.characters.push(character);
    this.characters = this.characters.concat(character_alts);
  }
}
</script>
