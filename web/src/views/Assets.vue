<template>
  <n-card title="Assets">
    <n-skeleton text v-if="busy" :repeat="10" />

    <div v-if="!busy">
      <filter-text
        :filters="filters"
        :options="filterOptions"
      />
      <filter-element
        style="margin-top: 5px"
        :filters="filters"
        :options="filterOptions"
      />

      <asset-table style="margin-top: 10px" :entries="entries" />
    </div>
  </n-card>
</template>

<script lang="ts">
import { NButton, NButtonGroup, NCard, NDataTable, NInput, NSkeleton, NSpace,
NTag } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h, VNode } from 'vue';

import AssetInfo from '@/components/asset/Info.vue';
import AssetTable from '@/components/asset/Table.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import Owner from '@/components/Owner.vue';

import { AssetService, ICharacterAsset } from '@/services/asset';
import { CharacterService } from '@/services/character';
import FilterText, { IFilterOption } from '@/components/Filter.vue';
import FilterElement from '@/components/FilterElement.vue';
import CategoryName from '../components/CategoryName.vue';

@Options({
  components: {
    NButton,
    NButtonGroup,
    NCard,
    NDataTable,
    NInput,
    NSkeleton,
    NSpace,
    NTag,

    AssetInfo,
    AssetTable,
    FilterText,
    FilterElement,
    ItemIcon,
    Owner,
  }
})
export default class CharacterAsset extends Vue {
  public busy: boolean = false;

  public entries: ICharacterAsset[] = [];

  public filters = {};
  public filterOptions: { [key: string]: IFilterOption } = {};

  public async created() {
    this.busy = true;

    await this.fetch_assets();

    let owner_opts = [];
    this.entries.map(x => x.owners
        .forEach(x => {
          if (owner_opts.indexOf(x) === -1) {
            owner_opts.push(x)
          }
        })
    );

    let category_opts = [];
    this.entries.map(x => {
      if (category_opts.indexOf(x.category_id) === -1) {
        category_opts.push(x.category_id)
      }
    });
    category_opts.sort();

    this.filterOptions = {
      name: {
        label: 'Name',
      },
      asset_name: {
        label: 'Asset Name',
      },
      category: {
        label:    'Category',
        options:  category_opts,
        template: (val: string): VNode => {
          return h(
            CategoryName,
            { id: Number(val) }
          )
        }
      },
      owner: {
        label:    'Owner',
        options:  owner_opts,
        template: (val: string): VNode => {
          return h(
            Owner,
            { id: Number(val), withText: true }
           )
        }
      }
    };

    this.$watch(() => this.filters, async () => {
      await this.fetch_assets();
    }, { deep: true });

    this.busy = false;
  }

  private async fetch_assets() {
    let key = 0;
    this.entries = (await AssetService.assets(this.filters))
      .map(x => {
        key += 1;
        return {
          key,
          ...x
        }
      });
  }
}
</script>
