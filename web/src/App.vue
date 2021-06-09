<template>
  <v-app id="app">
    <v-navigation-drawer clipped fixed app>
      <v-list>
        <v-list-group no-action v-if="loggedIn" :value="true">
          <template v-slot:activator>
            <v-list-item-content>
              <v-list-item-title>My shit n stuff</v-list-item-title>
            </v-list-item-content>
          </template>

          <v-list-item to="/my/assets">
            <v-list-item-content>
              <v-list-item-title>Assets</v-list-item-title>
            </v-list-item-content>
          </v-list-item>

          <v-list-item to="/my/blueprints">
            <v-list-item-content>
              <v-list-item-title>Blueprints</v-list-item-title>
            </v-list-item-content>
          </v-list-item>

          <v-list-item to="/my/skills">
            <v-list-item-content>
              <v-list-item-title>Skills</v-list-item-title>
            </v-list-item-content>
          </v-list-item>
        </v-list-group>

        <v-list-item to="/market">
          <v-list-item-content>
            <v-list-item-title>Market</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app fixed dense clipped-left>
      <v-toolbar-title>Caph</v-toolbar-title>

      <v-spacer></v-spacer>

      <v-btn v-if="!loggedIn" href="/api/eve/login">Eve login</v-btn>

      <div v-if="loggedIn">
          {{ characterName }}

        <v-list-item-avatar>
          <v-img :src="characterPortrait"></v-img>
        </v-list-item-avatar>
      </div>
    </v-app-bar>

    <v-main>
      <v-container fluid fill-height>
        <v-layout justify-center align-center>
          <v-flex text-xs-center fill-height>
            <router-view></router-view>
          </v-flex>
        </v-layout>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue } from 'vue-property-decorator';

@Component
export default class App extends Vue {
  public loggedIn: boolean = false;

  public characterName: string = '';
  public characterPortrait: string = '';

  public async created() {
    await this.character();
  }

  private async character() {
    axios
      .get(`/api/eve/whoami`)
      .then(_ => {
        return axios.get(`/api/character/name`);
      })
      .then(x => {
        this.characterName = x.data;
        return axios.get(`/api/character/portrait`);
      })
      .then(x => {
        this.characterPortrait = x.data;
        this.loggedIn = true;
      })
      .catch(_ => this.loggedIn = false);
  }
}
</script>
