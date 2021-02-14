<template>
  <v-app id="app">
    <v-navigation-drawer clipped fixed app>
      <v-list>
        <v-list-item to="/items/my">
          <v-list-item-content>
            <v-list-item-title>My items</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item to="/market/raw">
          <v-list-item-content>
            <v-list-item-title>Raw market</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app fixed dense clipped-left>
      <v-toolbar-title>Caph</v-toolbar-title>
    </v-app-bar>

    <v-main>
      <v-container fluid fill-height>
        <v-layout justify-center align-center>
          <v-flex text-xs-center fill-height>
            <v-progress-linear indeterminate v-if="busy"></v-progress-linear>
            <router-view></router-view>
          </v-flex>
        </v-layout>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class App extends Vue {
  public busy: boolean = false;

  public created() {
    axios.interceptors.request.use(
      config => {
        this.busy = true;
        return config;
      },
      error => Promise.reject(error)
    );

    // Add a response interceptor
    axios.interceptors.response.use(
      response => {
        this.busy = false;
        return response;
      },
      error => {
        this.busy = false;
        return Promise.reject(error);
      }
    );
  }
}
</script>
