<template>
  <span>
    <span v-if="name">{{ name }}</span>
    <span v-if="!name">Unknown {{ sid }}</span>
  </span>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { UniverseService } from '@/services/universe';

class Props {
  sid = prop({
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
    await this.load();

    this.$watch('sid', () => this.load());
  }

  async load() {
    await UniverseService
      .system(this.sid)
      .then(x => {
        this.name = x.name;
      });
  }
}
</script>
