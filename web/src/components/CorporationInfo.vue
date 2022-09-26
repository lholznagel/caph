<template>
  <div v-if="corporation">
    <slot :info="corporation"></slot>
  </div>
</template>

<script lang="ts">
import { CharacterService, ICorporationInfo } from '@/services/character';
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
export default class Corporation extends Vue.with(Props) {
  public corporation: ICorporationInfo = <any>{};

  public async created() {
    this.corporation = await CharacterService.corporation_info(this.id);
  }
}
</script>
