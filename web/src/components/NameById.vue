<template>
  <div v-if="name !== ''">{{ name }}</div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class NameById extends Vue {
  @Prop(Number)
  public id!: number;

  public name: string = '';

  public async created() {
    this.name = (await axios.get(`/api/v1/resolve/${this.id}`)).data.name;
  }
}
</script>
