<template>
  <label>{{ format() }}</label>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { formatNumber, formatTime } from '../utils/format';

@Component
export default class FormatNumber extends Vue {
  @Prop(Number)
  public value!: number;
  @Prop(Boolean)
  public isTime!: boolean;

  public format(): string {
    if (this.isTime) {
      return formatTime(this.value);
    } else {
      return formatNumber(this.value);
    }
  }
}
</script>
