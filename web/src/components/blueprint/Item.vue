<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <v-simple-table v-if="!busy && reqResources.length > 0">
      <template v-slot:default>
        <tbody>
          <tr v-for="item in reqResources" :key="item.itemId">
            <td style="width: 32px"><c-item-icon :id="Number(item.itemId)" /></td>
            <td><c-name-by-id :id="Number(item.itemId)" /></td>
            <td><c-format-number :value="Number(item.quantity)" /></td>
          </tr>
        </tbody>
      </template>
    </v-simple-table>
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

  public busy: boolean                = false;
  public reqResources: IReqResource[] = [];

  public async created() {
    this.busy = true;

    const graph: IGraph = (await axios.get(`/api/items/${this.blueprintId}/blueprint/graph`)).data;
    const resources = this.resources(new Map(), 1, graph.children);
    this.reqResources = Array.from(resources.values());

    this.busy = false;
  }

  private resources(map: Map<number, IReqResource>, mutliplier: number, children: IGraph[]): Map<number, IReqResource> {
    for (const child of children) {
      if (child.children.length > 0) {
        map = this.resources(map, child.quantity, child.children);
      } else {
        const entry = map.get(child.item_id);
        if (entry) {
          entry.quantity += child.quantity * mutliplier;
          map.set(child.item_id, entry);
        } else {
          map.set(child.item_id, {
            itemId: child.item_id,
            quantity: child.quantity * mutliplier
          });
        }

        map = this.resources(map, child.quantity, child.children);
      }
    }

    return map;
  }
}

interface IReqResource {
  itemId:   number;
  quantity: number;
}

interface IGraph {
  item_id: number;
  quantity: number;
  children: IGraph[];
}
</script>
