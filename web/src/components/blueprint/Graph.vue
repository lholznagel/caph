<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <v-treeview
      :activatable="false"
      :items="graph"
      dense
      v-if="!busy"
    >
      <template v-slot:prepend="{ item }">
        <c-item-icon :id="Number(item.item_id)" />
      </template>

      <template v-slot:label="{ item }">
        <c-name-by-id :id="Number(item.item_id)" />
        (<c-format-number :value="Number(item.quantity)" />)
      </template>
    </v-treeview>
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class BlueprintGraph extends Vue {
  // Blueprint item id
  @Prop(Number)
  public blueprintId!: number;

  public busy            = false;
  public search: string  = '';
  public graph: IGraph[] = [];

  public async created() {
    this.graph = [
      (await axios.get(`/api/items/${this.$route.params.id}/blueprint/graph`)).data
    ];
  }
}

interface IGraph {
  item_id: number;
  quantity: number;
  children: IGraph[];
}
</script>
