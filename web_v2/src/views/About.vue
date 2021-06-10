<template>
  <div class="about">
    <h1>This is an about page</h1>

    <n-card title="card">
      Card-content
    </n-card>
  </div>
</template>


<script lang="ts">
import { NCard } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

@Options({
  components: {
    NCard
  }
})
export default class About extends Vue {}
</script>
