<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <v-simple-table v-if="!busy && reprocessed.length > 0">
      <template v-slot:default>
        <tbody>
          <tr v-for="item in reprocessed" :key="item.material_id">
            <th><c-name-by-id :id="item.material_id" /></th>
            <td>
              <c-format-number :value="Math.floor(item.reprocessed)" />
            </td>
          </tr>
        </tbody>
      </template>
    </v-simple-table>

    <v-alert color="blue-grey" type="info" v-if="!busy && reprocessed.length === 0">
      Reprocessing not available
    </v-alert>
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
  public reprocessed: IReprocessing[] = [];

  public async created() {
    this.busy = true;
    this.reprocessed = (await axios.get(`/api/items/${this.id}/reprocessing`)).data;
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
