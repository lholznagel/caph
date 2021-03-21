<template>
  <v-img
    :src="require(`../../assets/img_64/${id}_64.png`)"
    max-height="32"
    max-width="32"
  ></v-img>

</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class NameById extends Vue {
  @Prop(Number)
  public id!: number;
}
</script>
