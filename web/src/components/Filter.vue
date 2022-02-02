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
import { NDropdown, NIcon, NInput, NSelect } from 'naive-ui';
import { Search } from '@vicons/fa';
import { VNode } from 'vue';

class Props {
  filters = prop({
    type: Object,
    required: true
  });

  options = prop({
    type: Object,
    required: true
  });

  entries = prop({
    type:     Array,
    required: true,
  });
}

@Options({
  components: {
    NDropdown,
    NIcon,
    NInput,
    NSelect,

    Search,
  }
})
export default class Filter extends Vue.with(Props) {
  public search: string      = '';
  public selectedKey: string = '';

  public filterOptions: any     = [];
  public filterOptionsOrig: any = [];
  public showOptions: boolean   = false;

  public entries_orig = this.entries;

  public created() {
    for (let key of Object.keys(this.options)) {
      let val = this.options[key];
      this.filterOptions.push({
        label: val.label,
        key: key
      });
    }

    this.filterOptionsOrig = this.filterOptions;

    this.$watch(() => this.filters, () => {
      console.log('asdasdasd')
      this.filter_entries();
    }, { deep: true });
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
      }
    } else {
      this.filters[this.selectedKey] = key;
      this.filter_entries();
      this.reset();
    }
  }

  public handleKeydown(event: any) {
    if (event.keyCode === 8) {
      if (this.selectedKey && this.search.indexOf(':') === -1) {
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

    if (entry && entry.template) {
      return entry.template(x.key);
    } else {
      return x.label;
    }
  }

  public onClickOutside() {
    this.showOptions = false;
  }

  private reset() {
      this.search = '';
      this.selectedKey = '';
      this.filterOptions = this.filterOptionsOrig;
      this.showOptions = false;
  }

  private filter_entries() {
    let entries = this.entries_orig;

    for (const key in this.filters) {
      console.log(this.filters)
      const matcher_exact = (
        key: string,
        val: string,
        entry: any
      ) => entry[key].localeCompare(val, undefined, { sensitivity: 'accent' });
      const matcher_fuzzy = (
        key: string,
        val: string,
        entry: any
      ) => entry[key].toLowerCase().includes(val.toLowerCase());
      let matcher = matcher_exact;

      if (this.options[key].matcher) {
        matcher = this.options[key].matcher;
      } else if (this.options[key].fuzzy) {
        matcher = matcher_fuzzy;
      }

      entries = entries
        .filter((x: any) => matcher(key, this.filters[key], x));
    }

    this.$emit('update:entries', entries);
  }
}

export interface IFilterOption {
  label:     string;
  fuzzy?:    boolean;
  options?:  string[] | number[];
  matcher?: (key: string, val: string, entry: any) => boolean;
  template?: (val: string) => VNode;
}
</script>

