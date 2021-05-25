<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <div v-if="!busy">
      <c-item-icon
        v-if="!bpChar.quantity || bpChar.quantity !== -2"
        :id="Number(bp.bid)"
        :type="'bp'"
        :max-height="Number(128)"
        :max-width="Number(128)" />
      <c-item-icon
        v-if="bpChar.quantity === -2"
        :id="Number(bp.bid)"
        :type="'bpc'"
        :max-height="Number(128)"
        :max-width="Number(128)" />

      <v-simple-table>
        <template v-slot:default>
          <tbody>
            <tr>
              <th>Name</th>
              <td><c-name-by-id :id="Number(bp.bid)"/></td>
            </tr>
            <tr>
              <th>Type</th>
              <td>{{ bpChar.quantity === -2 ? 'Copy' : 'Original' }}</td>
            </tr>
            <tr>
              <th>Activity</th>
              <td>{{ bp.activity }}</td>
            </tr>
            <tr>
              <th>Time per run</th>
              <td><c-format-number :value="bp.time" is-time /></td>
            </tr>
            <tr v-if="bpChar.material_efficiency">
              <th>Material efficiency</th>
              <td>{{ bpChar.material_efficiency }}</td>
            </tr>
            <tr v-if="bpChar.time_efficiency">
              <th>Time efficiency</th>
              <td>{{ bpChar.time_efficiency }}</td>
            </tr>
            <tr v-if="bpChar.runs">
              <th>Runs</th>
              <td>{{ bpChar.runs === -1 ? '∞' : bpChar.runs }}</td>
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
export default class BlueprintItemInfo extends Vue {
  // Id of the blueprint
  @Prop(Number)
  public blueprintId!: number;
  @Prop(Number)
  public itemId!: number;

  public busy                   = false;
  public bp:     IBlueprint     = {};
  public bpChar: ICharBlueprint = {};

  public async created() {
    this.busy = true;
    this.bp = (await axios.get(`/api/items/${this.blueprintId}/blueprint`)).data;

    if (this.itemId) {
      this.bpChar = (await axios.get(`/api/character/blueprints`))
        .data
        .find((x: ICharBlueprint) => x.item_id === Number(this.itemId));
    }

    this.busy = false;
  }
}

interface ICharBlueprint {
  item_id?: number;
  material_efficiency?: number;
  quantity?: number;
  runs?: number;
  time_efficiency?: number;
  type_id?: number;
}

export interface IBlueprint {
  activity?: 'Manufacturing' | 'Reaction';
  bid?:      number;
  time?:     number;
}
</script>
