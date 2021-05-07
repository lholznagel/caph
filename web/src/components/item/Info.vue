<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <div v-if="!busy">
      <c-item-icon
        :id="Number(id)"
        :max-height="Number(128)"
        :max-width="Number(128)"
        type="bp"
      />

      <v-simple-table>
        <template v-slot:default>
          <tbody>
            <tr>
              <th>Name</th>
              <td><c-name-by-id :id="Number(id)"/></td>
            </tr>
            <tr>
              <th>Volume</th>
              <td><c-format-number :value="info.volume" /> m³</td>
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

  public busy: boolean = false;
  public info: IItem = { item_id: 0, volume: 0 };

  public async created() {
    this.busy = true;
    this.info = (await axios.get(`/api/items/${this.id}`)).data;
    this.busy = false;
  }
}

interface IItem {
  item_id: number;
  volume: number;
}
</script>
