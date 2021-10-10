<template>
  <span>
    <span v-if="name">{{ name }}</span>
    <span v-if="!name">Unknown {{ id }}</span>
  </span>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { UniverseService } from '@/services/universe';

class Props {
  id = prop({
    type:     Number,
    required: true
  })
}

@Options({
  components: {
    NText
  }
})
export default class SystemName extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    await UniverseService
      .system(this.id)
      .then(x => {
        this.name = x.name;
      });
  }
}
</script>
