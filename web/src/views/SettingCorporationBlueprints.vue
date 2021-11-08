<template>
  <n-card :title="$route.params.tid ? 'Update template' : 'New template'">
    <n-skeleton text :repeat="5" v-if="busy" />

    <resolve v-model="aaa" />

    <pre>{{ aaa }}</pre>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NSkeleton } from 'naive-ui';

import Resolve from '@/components/Resolve.vue';

@Options({
  components: {
    NCard,
    NSkeleton,

    Resolve
  }
})
export default class SettingCorporationBlueprints extends Vue {
  public busy: boolean = false;

  public aaa = [];
}
</script>
