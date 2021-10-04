<template>
  <n-table>
    <thead>
      <tr>
        <th width="24"></th>
        <th width="34"></th>
        <th>Name</th>
        <th width="50">Quantity</th>
        <th width="100">Owners</th>
      </tr>
    </thead>
    <tbody>
      <template v-for="entry in entries" :key="entry.type_id">
        <tr>
          <td>
            <n-icon size="22">
              <angle-right
                style="cursor: pointer"
                @click="entry.open = true"
                v-if="!entry.open"
              />
              <angle-down
                style="cursor: pointer"
                @click="entry.open = false"
                v-if="entry.open"
              />
            </n-icon>
          </td>
          <td width="34">
            <item-icon
              :id="entry.type_id"
              :type="entry.name.indexOf('Blueprint') === -1 ? 'icon' : 'bp'"
              :width="32"
            />
          </td>
          <td>{{ entry.name }}</td>
          <td>{{ entry.quantity }}</td>
          <td>
            <n-space>
              <owner
                v-for="owner in entry.owners"
                :key="owner"
                :id="owner"
              />
            </n-space>
          </td>
        </tr>
        <tr v-if="entry.open">
          <td colspan="5">
            <asset-info :iids="entry.item_ids" />
          </td>
        </tr>
      </template>
    </tbody>
  </n-table>
</template>

<script lang="ts">
import { NButton, NIcon, NPagination, NSkeleton, NSpace, NTable } from 'naive-ui';
import { AngleDown, AngleRight } from '@vicons/fa';
import { Options, Vue , prop} from 'vue-class-component';

import AssetInfo from '@/components/asset/Info.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import Owner from '@/components/Owner.vue';

class Props {
  entries = prop({
    type:    Object,
    default: {}
  });
}

@Options({
  components: {
    NButton,
    NIcon,
    NPagination,
    NSkeleton,
    NSpace,
    NTable,

    AngleDown,
    AngleRight,

    AssetInfo,
    ItemIcon,
    Owner,
  }
})
export default class CharacterAsset extends Vue.with(Props) {  }
</script>
