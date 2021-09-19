<template>
  <n-text>{{ name || '' }}</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { NameService } from '@/services/name';

class Props {
  id = prop({
    type: Number,
    required: true
  })
}

@Options({
  components: {
    NText
  }
})
export default class NameById extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    this.name = await NameService.resolve(this.id);
    this.$watch('id', async () => {
      this.name = await NameService.resolve(this.id);
    });
  }
}
</script>
