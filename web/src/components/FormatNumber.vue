<template>
  <label>{{ format() }}</label>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { formatNumber } from '../utils/format';

@Component
export default class FormatNumber extends Vue {
  @Prop(Number)
  public value!: string;

  public format(): string {
    return formatNumber(this.value);
  }
}
</script>
