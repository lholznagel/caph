<template>
  <n-card title="Features">
  </n-card>
</template>

<script lang="ts">
import { NCard } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

@Options({
  components: {
    NCard
  }
})
export default class Assets extends Vue {
  public busy: boolean = false;
}
</script>
