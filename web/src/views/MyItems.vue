<template>
  <div>
    <v-dialog v-model="addItems" width="500">
      <template v-slot:activator="{ on, attrs }">
        <v-btn v-bind="attrs" v-on="on"> Add items </v-btn>
      </template>

      <v-card>
        <v-card-title class="headline"> Add items </v-card-title>

        <v-card-text>
          <v-textarea
            filled
            placeholder="Insert items"
            v-model="items"
          ></v-textarea>
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="primary" text @click="pushItems()"> Add items </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-card class="mt-5">
      <v-card-title>My items</v-card-title>

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
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue } from 'vue-property-decorator';

@Component
export default class MyItems extends Vue {
  public items: string = '';
  public itemFilter: string = '';
  public addItems: boolean = false;

  public myItemsOrig: IMyItems[] = [];
  public myItems: IMyItems[] = [];
  public names: IName[] = [];

  public async created() {
    const items = (await axios.get('/api/v1/items/my')).data;
    this.names = (
      await axios.post(
        '/api/v1/resolve',
        items.map(x => x.id)
      )
    ).data;
    this.myItemsOrig = items;
    this.myItems = items;
  }

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
  }

  public async pushItems() {
    const parsed: IParseInput[] = [];

    for (const item of this.items.split('\n')) {
      const splitted = item.split('\t');
      parsed.push({
        name: splitted[0],
        quantity: Number(splitted[1]) <= 0 ? 1 : Number(splitted[1])
      });
    }

    // TODO: use resolve api
    const search: IName[] = (
      await axios.post(
        '/api/v1/items/search?exact=true',
        parsed.map(x => x.name)
      )
    ).data;

    const req = {};
    for (const parse of parsed) {
      if (parse.name === '') {
        continue;
      }

      const id = (search.find(y => y.name === parse.name) || { id: 1 }).id;

      // if there is already an entry, add the quantity
      if (req[id]) {
        req[id] += parse.quantity;
      } else {
        req[id] = parse.quantity;
      }
    }
    await axios.post('/api/v1/items/my', req);
    this.addItems = false;
  }
}

interface IParseInput {
  name: string;
  quantity: number;
}

interface IName {
  name: string;
  id: number;
}

interface IMyItems {
  id: number;
  name: string;
}
</script>
