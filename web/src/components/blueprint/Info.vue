<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <div v-if="!busy">
      <v-img
        :src="`https://images.evetech.net/types/${blueprintId}/bp?size=1024`"
        max-height="128"
        max-width="128"
      ></v-img>

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
            <tr>
              <th>Material efficiency</th>
              <td>{{ blueprint.material_efficiency }}</td>
            </tr>
            <tr>
              <th>Time efficiency</th>
              <td>{{ blueprint.time_efficiency }}</td>
            </tr>
            <tr>
              <th>Runs</th>
              <td>{{ blueprint.runs }}</td>
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

    this.blueprint = (await axios.get(`/api/character/blueprints`))
      .data
      .find((x: IBlueprint) => x.item_id === Number(this.$route.params.itemId));

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
