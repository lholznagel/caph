<template>
  <n-text>{{ name || '' }} (<name-by-id :id="systemId" />)</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { CharacterService, IItemLocation } from '@/services/character';

import NameById from '@/components/NameById.vue';

class Props {
  // Location id of the item
  lid = prop({
    type: Number,
    required: true,
  });
}

@Options({
  components: {
    NText,

    NameById,
  }
})
export default class Location extends Vue.with(Props) {
  public name:     string = '';
  public systemId: number = 0;

  public async created() {
    let info: IItemLocation = await CharacterService.itemLocation(this.lid);

    this.name = info.name;
    this.systemId = info.system_id
  }
}
</script>
