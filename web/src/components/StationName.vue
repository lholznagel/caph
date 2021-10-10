<template>
  <span>
    <n-text v-if="name">{{ name }} (<system-name :id="system_id" />)</n-text>
    <n-text v-if="!name">Unknown {{ id }}</n-text>
  </span>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { UniverseService } from '@/services/universe';

import SystemName from '@/components/SystemName.vue';

class Props {
  id = prop({
    type:     Number,
    required: true
  })
}

@Options({
  components: {
    NText,

    SystemName
  }
})
export default class LocationName extends Vue.with(Props) {
  public name: string      = '';
  public system_id: number = 0;

  public async created() {
    await UniverseService
      .station(this.id)
      .then(x => {
        this.name = x.name;
        this.system_id = x.system_id;
      });
  }
}
</script>
