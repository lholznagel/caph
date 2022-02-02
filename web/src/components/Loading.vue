<template>
  <n-empty
    size="large"
    style="margin-top: 10%"
    v-if="busy"
    :description="description"
  >
    <template #icon>
      <n-spin size="large" />
    </template>
  </n-empty>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NEmpty, NSpin } from 'naive-ui';

class Props {
  busy = prop({
    type:     Boolean,
    required: true
  });

  description = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NEmpty,
    NSpin,
  }
})
export default class Loading extends Vue.with(Props) {  }
</script>
