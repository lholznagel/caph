<template>
  <div>
    <v-card>
      <v-card-title>
        Blueprint info for "<c-name-by-id :id="Number($route.params.id)"/>"
      </v-card-title>

      <v-row dense>
        <v-col cols="3">
          <v-card elevation="5" style="height: 100%">
            <v-card-title>Blueprint Info</v-card-title>
            <c-blueprint-info
              :blueprint-id="Number($route.params.id)"
              :item-id="Number($route.params.itemId)" />
          </v-card>
        </v-col>
        <v-col cols="9">
          <v-card elevation="5" style="height: 100%">
            <v-card-title>Item cost</v-card-title>
            <c-blueprint-item :blueprint-id="Number($route.params.id)" />
          </v-card>
        </v-col>
      </v-row>
    </v-card>

    <v-card class="mt-5">
      <v-card-title>
        Item graph
      </v-card-title>

      <v-card-text>
        <c-blueprint-graph :blueprint-id="Number($route.params.id)" />
      </v-card-text>
    </v-card>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Blueprint extends Vue {
}

</script>
