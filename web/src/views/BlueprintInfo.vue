<template>
  <div>
    <v-card>
      <v-card-title>
        Blueprint info for "<c-name-by-id :id="Number($route.params.id)"/>"
      </v-card-title>

      <v-card-text>
        <c-blueprint-item :blueprint-id="Number($route.params.id)" />
      </v-card-text>
    </v-card>

    <v-card class="mt-5">
      <v-card-title>
        Item graph
      </v-card-title>

      <v-card-text>
        <v-treeview
          :activatable="false"
          dense
          :items="items"
        >
          <template v-slot:prepend="{ item }">
            <c-item-icon  :id="Number(item.item_id)" />
          </template>

          <template v-slot:label="{ item }">
            <c-name-by-id :id="Number(item.item_id)" />
            (<c-format-number :value="Number(item.quantity)" />)
          </template>
        </v-treeview>
      </v-card-text>
    </v-card>
  </div>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class BlueprintInfo extends Vue {
  public items:              IGraph[] = [];

  public async created() {
    this.items = [(await axios.get(`/api/items/${this.$route.params.id}/blueprint/graph`)).data];
  }
}

interface IGraph {
  item_id: number;
  quantity: number;
  children: IGraph[];
}
</script>
