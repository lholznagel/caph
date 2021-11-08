<template>
  <div v-if="infos.length > 0">
    <n-table
      style="width: 100%; margin-top: 10px"
      v-for="(info, idx) in infos"
      :key="info.item_id"
    >
      <tbody>
        <tr>
          <th width="200px">Name</th>
          <td>{{ info.name }}</td>
        </tr>

        <tr>
          <th>Category</th>
          <td><category-name :id="info.category_id" /></td>
        </tr>

        <tr>
          <th>Quantity</th>
          <td>{{ info.quantity }}</td>
        </tr>

        <tr>
          <th>Volume</th>
          <td><format-number :value="info.quantity * info.volume" />m³</td>
        </tr>

        <tr>
          <th>Location</th>
          <td>
            <station-name
              v-if="info.reference_id && references[idx]"
              :id="references[idx].location_id"
            />
            <station-name
              v-if="!info.reference_id"
              :id="info.location_id"
            />
          </td>
        </tr>

        <tr>
          <th>Flag</th>
          <td>{{ info.location_flag }}</td>
        </tr>

        <tr v-if="info.reference_id && references[idx]">
          <th>Reference</th>
          <td>
            {{ references[idx].name }}
            (<asset-name :tid="references[idx].item_id" />)
          </td>
        </tr>

        <tr v-if="info.category_id === 6">
          <th>Ship Name</th>
          <td>
            <asset-name :tid="info.item_id" />
          </td>
        </tr>

        <tr v-if="info.original !== null">
          <th>BP-Type</th>
          <td>
            <n-tag v-if="info.original" type="info">Original</n-tag>
            <n-tag v-if="!info.original" type="warning">Copy</n-tag>
          </td>
        </tr>

        <tr v-if="info.original !== null">
          <th>Material Efficiency</th>
          <td>{{ info.material_eff }}</td>
        </tr>

        <tr v-if="info.original !== null">
          <th>Time Efficiency</th>
          <td>{{ info.time_eff }}</td>
        </tr>

        <tr v-if="info.original !== null">
          <th>Remaining runs</th>
          <td>{{ info.runs }}</td>
        </tr>

        <tr>
          <th>Owner</th>
          <td><owner :id="info.owner" :with-text="true" /></td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { NTable, NTag } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

import { AssetService, IAsset } from '@/services/asset';

import AssetName from '@/components/AssetName.vue';
import CategoryName from '@/components/CategoryName.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import StationName from '@/components/StationName.vue';
import Owner from '@/components/Owner.vue';

class Props {
  iids = prop({
    type: Array,
    required: true
  });
}

@Options({
  components: {
    NTable,
    NTag,

    AssetName,
    CategoryName,
    FormatNumber,
    StationName,
    Owner
  }
})
export default class AssetInfo extends Vue.with(Props) {
  public infos: IAsset[]      = [];
  public references: IAsset[] = [];

  public async created() {
    for (let iid of this.iids) {
      const info = await AssetService.by_id(<number>iid);

      let reference_exists = this.infos
        .find(x => x.reference_id === info.reference_id);
      if (info.location_flag === 'Hangar' || !reference_exists) {
        this.infos.push(info);
      } else {
        let res: any = this.infos.find(x => x.reference_id === info.reference_id)
        if (res) {
          res.quantity += 1;
        }
      }
    }

    for (let info of this.infos) {
      if (info.reference_id) {
        const reference = await AssetService.by_id(<number>info.reference_id);
        this.references.push(reference);
      } else {
        this.references.push(<any>null);
      }
    }
  }

  public open_details(tid: number, iid: number) {
    this.$router.push({
      name: 'asset_details',
      params: { tid, iid }
    });
  }
}
</script>
