<template>
  <n-card title="Settings">
    <n-skeleton text v-if="busy" :repeat="6" />

    <n-grid x-gap="12" :cols="5">
      <n-grid-item>
        <n-image width="256" :src="character.portrait" />
        <span>{{ character.name }}</span>
      </n-grid-item>
      <n-grid-item v-for="alias in character.aliase" :key="alias.name">
        <n-image width="256" :src="alias.portrait" />
        <span>{{ alias.name }}</span>
      </n-grid-item>
   </n-grid>

    <n-button @click="addAlt">Add character</n-button>
  </n-card>
</template>

<script lang="ts">
import axios from 'axios';
import {
  NButton, NCard, NImage, NSkeleton, NGrid, NGridItem
} from 'naive-ui';
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

  public character: ICharacter = DEFAULT_CHARACTER;

  public async created() {
    this.busy = true;

    this.character = (await axios.get('/api/character/info')).data;

    this.busy = false;
  }

  public addAlt() {
    window.location.href = `/api/eve/login/alt`;
  }
}

interface ICharacter {
  aliase:         ICharacter[];
  alliance?:      String;
  alliance_icon?: String;
  corp:           String;
  corp_icon:      String;
  name:           String;
  portrait:       String;
  user_id:        number;
}

const DEFAULT_CHARACTER: ICharacter = {
  aliase:    [],
  corp:      '',
  corp_icon: '',
  name:      '',
  portrait:  '',
  user_id:   0,
};
</script>
