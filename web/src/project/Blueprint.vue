<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-card content-style="padding: 0">
        <p-table-header
          :reload="() => {
            project.load_blueprints();
            project.load_stored_blueprints();
          }"
        >
          <template #primary>
            <n-button
              type="info"
              @click="project.import_blueprints()"
              :disabled="busy"
              :loading="busy"
            >
              Import from characters
            </n-button>
          </template>
          <template #additional>
            <n-button
              @click="show_export = true"
              :disabled="busy"
            >
              Export
            </n-button>
          </template>
        </p-table-header>

        <div style="margin: 10px">
          <filter-text
            v-model:entries="project.blueprints"
            :filters="filters"
            :options="filterOptions"
            style="width: 100%"
          />

          <filter-element
            style="margin-top: 5px"
            :filters="filters"
            :options="filterOptions"
          />
        </div>

        <n-table v-if="project.blueprints.length > 0">
          <thead>
            <tr>
              <th width="34"></th>
              <th width="500">Name</th>
              <th width="100">Runs</th>
              <th width="150">Material efficiency</th>
              <th width="150">Time efficiency</th>
              <th width="200">Status</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="entry in project.blueprints"
              :key="entry.type_id"
            >
              <td>
                <item-icon
                  :id="entry.type_id"
                  :type="
                    entry.runs === -1
                      ? 'bp'
                      : 'bpc'
                  "
                  :width="32"
                />
              </td>
              <td>{{ entry.name }}</td>
              <td>{{
                    entry.runs
                      ? entry.runs === -1
                        ? '∞'
                        : entry.runs
                      : ''
                  }}
              </td>
              <td>{{ entry.me }}</td>
              <td>{{ entry.te }}</td>
              <td>
                <n-tag v-if="entry.stored > 0" type="success">Stored</n-tag>
                <n-tag v-else type="error">Missing</n-tag>
              </td>
            </tr>
          </tbody>
        </n-table>

        <n-empty
          description="Nothing to see here"
          size="large"
          style="margin: 5%"
          v-if="!busy && project.blueprints.length === 0"
        />
      </n-card>

      <p-export
        v-model:show="show_export"
        :data="project.blueprints"
        :data-fields="['name']"
        :data-fields-csv="['type_id', 'name', 'me', 'te']"
        :pid="$route.params.pid"
      />
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NCheckbox, NEmpty, NLayout, NLayoutContent, NLayoutHeader, NSpace, NTable, NTag } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import FilterElement from '@/components/FilterElement.vue';
import FilterText, { IFilterOption } from '@/components/Filter.vue';
import ItemIcon from '@/components/ItemIcon.vue';

import PExport from '@/project/MExport.vue';
import PHeader from '@/project/CHeader.vue';
import PTableHeader from '@/project/CTableHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NEmpty,
    NLayout,
    NLayoutContent,
    NLayoutHeader,
    NSpace,
    NTable,
    NTag,

    FilterElement,
    FilterText,
    ItemIcon,

    PExport,
    PHeader,
    PTableHeader,
    WProject,
  }
})
export default class ProjectBlueprintView extends Vue {
  public show_export: boolean = false;

  public filters: any = {};
  public filterOptions: { [key: string]: IFilterOption } = {};

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );

    await this.filter();
  }

  private async filter() {
    this.filterOptions = {
      name: {
        label:   'Name',
        matcher: (_: string, val: string, entry: any): boolean => entry.name.indexOf(val) > -1
      },
      status: {
        label:   'Status',
        options:  ['Stored', 'Missing'],
        matcher: (_: string, val: string, entry: any): boolean => {
          if (val === 'Stored') {
            return entry.stored;
          } else if (val === 'Missing') {
            return !entry.stored;
          } else {
            return true;
          }
        }
      },
      type: {
        label:   'Type',
        options:  ['Blueprints', 'Reactions'],
        matcher: (_: string, val: string, entry: any): boolean => {
          if (val === 'Blueprints') {
            return entry.is_manufacture;
          } else if (val === 'Reactions') {
            return entry.is_reaction;
          } else {
            return true;
          }
        }
      }
    };
  }
}
</script>
