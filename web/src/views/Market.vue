<template>
  <!--div>
    <v-card class="mt-5">
      <v-card-title>Items</v-card-title>

      <v-card-text>
        <v-text-field
          label="#WithFilter"
          v-model="itemFilter"
          @input="filterItems"
          clearable
        ></v-text-field>

        <v-simple-table>
          <template v-slot:default>
            <thead>
              <tr>
                <th>Name</th>
                <th>Quantity</th>
                <th>Info</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in myItems" :key="item.id">
                <td>{{ names.find((x) => x.id === item.id).name }}</td>
                <td><c-format-number :value="item.quantity" /></td>
                <td>
                  <v-btn
                    icon
                    color="blue"
                    :to="{ name: 'MarketInfo', params: { id: item.id } }"
                  >
                    <v-icon>mdi-open-in-new</v-icon>
                  </v-btn>
                </td>
              </tr>
            </tbody>
          </template>
        </v-simple-table>
      </v-card-text>
    </v-card>
  </div-->
  <v-btn
    icon
    color="blue"
    :to="{ name: 'MarketInfo', params: { id: 28606 } }"
    >
    Temp
  </v-btn>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue } from 'vue-property-decorator';

@Component
export default class MyItems extends Vue {
  /*public items: string = '';
  public itemFilter: string = '';
  public addItems: boolean = false;

  public myItemsOrig: IMyItems[] = [];
  public myItems: IMyItems[] = [];
  public names: IName[] = [];

  public filterItems() {
    if (!this.itemFilter) {
      this.myItems = this.myItemsOrig;
      return;
    }

    const ids = this.names
      .filter(
        x => x.name.toLowerCase().indexOf(this.itemFilter.toLowerCase()) !== -1
      )
      .map(x => x.id);
    this.myItems = this.myItemsOrig.filter(x => ids.indexOf(x.id) !== -1);
  }*/
}

interface IName {
  id: number;
  name: string;
}

interface IMyItems {
  id: number;
  name: string;
}
</script>
