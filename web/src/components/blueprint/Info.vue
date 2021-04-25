<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <div v-if="!busy">
      <c-item-icon
        v-if="!blueprint.quantity || blueprint.quantity !== -2"
        :id="Number(blueprint.type_id)"
        :type="'bp'"
        :max-height="Number(128)"
        :max-width="Number(128)" />
      <c-item-icon
        v-if="blueprint.quantity === -2"
        :id="Number(blueprint.type_id)"
        :type="'bpc'"
        :max-height="Number(128)"
        :max-width="Number(128)" />

      <v-simple-table>
        <template v-slot:default>
          <tbody>
            <tr>
              <th>Name</th>
              <td><c-name-by-id :id="Number(blueprintId)"/></td>
            </tr>
            <tr>
              <th>Type</th>
              <td>{{ blueprint.quantity === -2 ? 'Copy' : 'Original' }}</td>
            </tr>
            <tr v-if="blueprint.material_efficiency">
              <th>Material efficiency</th>
              <td>{{ blueprint.material_efficiency }}</td>
            </tr>
            <tr v-if="blueprint.time_efficiency">
              <th>Time efficiency</th>
              <td>{{ blueprint.time_efficiency }}</td>
            </tr>
            <tr v-if="blueprint.runs">
              <th>Runs</th>
              <td>{{ blueprint.runs === -1 ? '∞' : blueprint.runs }}</td>
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

  public busy                  = false;
  public blueprint: IBlueprint = {};

  public async created() {
    this.busy = true;

    if (this.itemId) {
      this.blueprint = (await axios.get(`/api/character/blueprints`))
        .data
        .find((x: IBlueprint) => x.item_id === Number(this.itemId));
    } else {
      const item = (await axios.get(`/api/items/${this.blueprintId}`)).data;
      // FIXME: Fix in server to return type_id
      this.blueprint = item;
      this.blueprint.type_id = item.item_id;
    }

    this.busy = false;
  }
}

interface IBlueprint {
  item_id?: number;
  material_efficiency?: number;
  quantity?: number;
  runs?: number;
  time_efficiency?: number;
  type_id?: number;
}
</script>
