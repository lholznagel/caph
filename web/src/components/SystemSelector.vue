<template>
  <n-select
    v-model:value="value"
    :options="options"
    filterable
  />
</template>

<script lang="ts">
import { NSelect } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { UniverseService } from '@/services/universe';

@Options({
  components: {
    NSelect
  }
})
export default class SystemSelector extends Vue {
  public options: any[] = [];

  public value: number | null = null;

  public async created() {
    this.options = (await UniverseService.systems()).map(x => {
      return {
        label: x.name,
        value: x.id
      }
    });
  }
}
</script>
