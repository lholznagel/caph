<template>
  <v-img
    :src="imagePath"
    :max-height="maxHeight ? maxHeight : 32"
    :max-width="maxWidth ? maxWidth: 32"
    v-on:error="loadAlternative()"
  ></v-img>

</template>

<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';

const BASE_URL = 'https://images.evetech.net/types';

@Component
export default class NameById extends Vue {
  @Prop(Number)
  public id!:        number;
  @Prop(Number)
  public maxHeight!: number;
  @Prop(Number)
  public maxWidth!:  number;
  @Prop(String)
  public type!:      string; // render, icon, bp, bpc

  public imagePath = `${BASE_URL}/${this.id}/${this.type}?size?1024`;

  public loadAlternative() {
    this.imagePath = `${BASE_URL}/${this.id}/icon?size?1024`;
  }

  @Watch('id')
  public watchId() {
    this.imagePath = `${BASE_URL}/${this.id}/${this.type}?size?1024`;
  }
}
</script>
