<template>
  <n-text :type="type">{{ format() }}</n-text>
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

  time = prop({
    type:     Boolean,
    required: false,
  });

  withComma = prop({
    type:     Boolean,
    required: false,
    default:  false
  });

  type = prop({
    type:     String,
    required: false,
    default:  'default'
  })
}

@Options({
  components: {
    NText
  }
})
export default class FormatNumber extends Vue.with(Props) {
  public format(): string {
    if (this.time) {
      return formatTime(this.value);
    } else {
      return formatNumber(this.value, this.withComma);
    }
  }
}
</script>
