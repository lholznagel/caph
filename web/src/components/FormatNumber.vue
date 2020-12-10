<template>
  <label>{{ format() }}</label>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class FormatNumber extends Vue {
  @Prop(Number)
  public value!: string;

  public format(): string {
    const splitted = this.value.toString().split('.');
    const formatValue = splitted[0].split('').reverse().join('');
    const result: string[] = [];
    let count = 0;

    for (let i = 0; i < formatValue.length; i++) {
      if (count === 3) {
        result.push('.');
        count = 0;
      }
      result.push(formatValue.charAt(i));
      count += 1;
    }

    if (splitted[1]) {
      return result.reverse().join('') + ',' + splitted[1];
    } else {
      return result.reverse().join('');
    }
  }
}
</script>
