<template>
  <v-simple-table>
    <template v-slot:default>
      <thead>
        <tr>
          <th>
            Location
          </th>
          <th>
            Station
          </th>
          <th>
            Quantity
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="location in asset.locations" :key="location.item_id">
          <td>{{ getItemName(location) }}</td>
          <td>{{ getOrigItemLocation(location.item_id) }}</td>
          <td>{{ location.quantity }}</td>
        </tr>
      </tbody>
    </template>
  </v-simple-table>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { IdNameCache } from '@/services/resolve_names';
import { IAsset, IAssetName, ILocation } from '@/services/character';

@Component
export default class CharacterAssetLocation extends Vue {
  @Prop(Object)
  public asset!: IAsset;
  @Prop(Array)
  public assets!: IAsset[];
  @Prop(Array)
  public names!:  IAssetName[];

  public getItemName(location: ILocation): string {
    const result = this
      .names
      .find(x => x.item_id === location.item_id) ||
      { name: IdNameCache.resolve_name_sync(location.item_id) };

    if (location.typ === 'Hangar') {
      return 'Hangar';
    } else {
      const origName = this.assets
        .find(x => x.item_ids.indexOf(location.item_id) !== -1) || { name: 'Unknown' };
      return `${origName.name} (${result.name})`;
    }
  }

  public getOrigItemLocation(itemId: number): string {
    const orig = this.assets.find(x => x.item_ids.indexOf(itemId) !== -1);
    if (orig) {
      const location = orig.locations.find(x => x.typ === 'Hangar');
      return IdNameCache.resolve_name_sync(location?.item_id || 0);
    } else {
      return IdNameCache.resolve_name_sync(itemId);
    }
  }
}
</script>
