<template>
  <n-space>
    <n-tag
      style="height: 40px"
      closable
      @close="handleClose(key.toString())"
      v-for="(value, key) in filters"
      :key="key"
    >
      <n-space class="filter" align="center" size="small">
        <strong >{{ options[key].label }}: </strong>

        <div v-if="options[key].element === 'OWNER'">
          <owner
            :id="Number(value)"
            withText
          />
        </div>
        <span v-else>{{ value }}</span>
      </n-space>
    </n-tag>
  </n-space>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NSpace, NTag } from 'naive-ui';

import Owner from './Owner.vue';

class Props {
  filters = prop({
    type: Object,
    required: true
  });
  options = prop({
    type: Object,
    required: true
  });
}

@Options({
  components: {
    NSpace,
    NTag,

    Owner
  }
})
export default class FilterElement extends Vue.with(Props) {
  public handleClose(key: string) {
    delete this.filters[key];
  }
}
</script>

<style>
.filter .owner {
  margin-top: 8px;
}
</style>
