<template>
  <n-dropdown
    trigger="click"
    placement="bottom-start"
    @select="filterSelected"
    @clickoutside="onClickOutside"
    :options="filterOptions"
    :show="showOptions"
    :render-label="renderLabel"
  >
    <n-input
      type="text"
      placeholder="#WithFilter"
      ref="filterInput"
      v-model:value="search"
      @click="showOptions = true"
      @keydown="handleKeydown"
    >
      <template #prefix>
        <n-icon>
          <search />
        </n-icon>
      </template>
    </n-input>
  </n-dropdown>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NDropdown, NIcon, NInput } from 'naive-ui';
import { Search } from '@vicons/fa';
import { h } from 'vue';

import NameById from './NameById.vue';
import Owner from './Owner.vue';

class Props {
  filters = prop({
    type: Object,
    required: true
  });
  options = prop({
    type: Object,
    required: true
  });
}

@Options({
  components: {
    NDropdown,
    NIcon,
    NInput,

    NameById,
    Search,
    Owner
  }
})
export default class Filter extends Vue.with(Props) {
  public search: string = '';
  public selectedKey: string = '';

  public filterOptions: any = [];
  public filterOptionsOrig: any = [];
  public showOptions: boolean = false;

  public created() {
    for (let key of Object.keys(this.options)) {
      let val = this.options[key];
      this.filterOptions.push({
        label: val.label,
        key: key
      });
    }

    this.filterOptionsOrig = this.filterOptions;
  }

  public filterSelected(key: string) {
    if (this.selectedKey && this.options[key]) {
      this.selectedKey = '';
      this.filterSelected(key);
    } else if (!this.selectedKey) {
      this.selectedKey = key;

      this.search = `${this.options[this.selectedKey].label}: `;

      const entry = this.options[this.selectedKey];
      if (!entry.options) {
        this.showOptions = false;
        (<any>this.$refs['filterInput']).focus();
      } else {
        this.filterOptions = [];
        for (let option of entry.options) {
          this.filterOptions.push({
            label: option,
            key: option
          });
        }
        console.log(this.filterOptions)
      }
    } else {
      this.filters[this.selectedKey] = key;
      this.reset();
    }
  }

  private reset() {
      this.search = '';
      this.selectedKey = '';
      this.filterOptions = this.filterOptionsOrig;
      this.showOptions = false;
  }

  public handleKeydown(event: any) {
    if (event.keyCode === 8) {
      if (this.search.indexOf(':') === -1) {
        this.reset();
      }
    } else if (!this.selectedKey && event.keyCode === 13) {
      this.filters['name'] = this.search;
      this.reset();
    } else if (event.keyCode === 13 && !this.options[this.selectedKey].options) {
      this.filters[this.selectedKey] = this
        .search
        .replace(`${this.options[this.selectedKey].label}: `, '');
      this.reset();
    }
  }

  public renderLabel(x: any) {
    const entry = this.options[this.selectedKey];

    if (!this.selectedKey) {
      return x.label;
    } else if(entry.element === 'OWNER') {
      return h(
        Owner,
        { ids: [Number(x.key)], withText: true }
      )
    } else {
      return x.label;
    }
  }

  public onClickOutside() {
    this.showOptions = false;
  }
}

export interface IFilterOption {
  label: string;
  element: 'OWNER';
  options?: string[];
}
</script>

