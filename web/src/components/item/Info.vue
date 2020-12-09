<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class ItemInfo extends Vue {
  @Prop(Number)
  public id!: number;

  public busy: boolean = false;
  public quantityModifier: number = 0;
  public info: IReprocessing[] = [];

  public async created() {
    this.busy = true;
    this.info = (await axios.get(`/api/v1/items/${this.id}`)).data;
    console.log(this.info)
    this.busy = false;
  }
}

interface IReprocessing {
  id: number;
  material_id: number;
  quantity: number;
  reprocessed: number;
}
</script>
