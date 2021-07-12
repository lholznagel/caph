<template>
  <n-text>{{ format() }}</n-text>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { formatNumber, formatTime } from '../services/formatNumber';
import { NText } from 'naive-ui';

class Props {
  value = prop({
    type: Number,
    required: true
  });

  isTime = prop({
    type: Boolean,
    required: false,
  });
}

@Options({
  components: {
    NText
  }
})
export default class FormatNumber extends Vue.with(Props) {
  public format(): string {
    if (this.isTime) {
      return formatTime(this.value);
    } else {
      return formatNumber(this.value);
    }
  }
}
</script>
