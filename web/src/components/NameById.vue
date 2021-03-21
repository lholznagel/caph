<template>
  <label>{{ name || '' }}</label>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';
import { IdNameCache } from '../services/resolve_names';

@Component
export default class NameById extends Vue {
  @Prop(Number)
  public id!: number;

  public name: string = '';

  public async created() {
    this.name = (await IdNameCache.resolve(this.id));
  }
}
</script>
