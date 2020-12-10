<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <div v-if="!busy">
      <v-img
        :src="'https://image.eveonline.com/Type/' + id + '_128.png'"
        max-height="128"
        max-width="128"
      ></v-img>

      <v-simple-table>
        <template v-slot:default>
          <tbody>
            <tr>
              <th>Name</th>
              <td>{{ info.name }}</td>
            </tr>
            <tr>
              <th>Quantity owned</th>
              <td><c-format-number :value="quantity" /></td>
            </tr>
            <tr>
              <th>Volume</th>
              <td><c-format-number :value="info.volume" /> m³ (<c-format-number :value="info.volume * quantity" /> m³) </td>
            </tr>
          </tbody>
        </template>
      </v-simple-table>
    </div>
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class ItemInfo extends Vue {
  @Prop(Number)
  public id!: number;
  @Prop(Number)
  public quantity!: number;

  public busy: boolean = false;
  public quantityModifier: number = 0;
  public info: IReprocessing[] = [];

  public async created() {
    this.busy = true;
    this.info = (await axios.get(`/api/v1/items/${this.id}`)).data;
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
