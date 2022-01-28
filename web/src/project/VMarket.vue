<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-tabs v-if="!busy" type="line">
        <n-tab-pane
          v-for="filter in filters"
          :name="filter.label"
          :key="filter.key"
        >
          <n-card content-style="padding: 0">
            <n-space justify="end" style="margin: 10px">
              <n-button
                @click="show_export = true"
              >
                Export
              </n-button>
            </n-space>

            <n-table v-if="!busy">
              <tbody>
                <tr>
                  <th width="16px"></th>
                  <th width="400px">Name</th>
                  <th width="100px">Required (Total)</th>
                  <th width="50px" style="text-align: right;">Min (Stack)<br /> Min (Single)</th>
                  <th width="50px" style="text-align: right;">Avg (Stack)<br /> Avg (Single)</th>
                  <th width="50px" style="text-align: right;">Max (Stack)<br /> Max (Single)</th>
                </tr>
                <tr v-for="market in project.market_info(filter.key, sid)" :key="market.type_id">
                  <td><item-icon :id="market.type_id" type="icon" /></td>
                  <td>{{ market.name }}</td>
                  <td>
                    <format-number :value="market.count" />
                  </td>
                  <td style="text-align: right;">
                    <format-number :value="market.a_min" with-comma /> ISK<br />
                    <format-number :value="market.s_min" with-comma /> ISK
                  </td>
                  <td style="text-align: right;">
                    <format-number :value="market.a_avg" with-comma /> ISK<br />
                    <format-number :value="market.s_avg" with-comma /> ISK
                  </td>
                  <td style="text-align: right;">
                    <format-number :value="market.a_max" with-comma /> ISK<br />
                    <format-number :value="market.s_max" with-comma /> ISK
                  </td>
                </tr>
                <tr>
                  <td></td>
                  <td>Total</td>
                  <td></td>
                  <td style="text-align: right;">
                    <format-number
                      with-comma
                      :value="project.market_total_min(filter.key, sid)"
                    /> ISK
                  </td>
                  <td style="text-align: right;">
                    <format-number
                      with-comma
                      :value="project.market_total_avg(filter.key, sid)"
                    /> ISK
                  </td>
                  <td style="text-align: right;">
                    <format-number
                      with-comma
                      :value="project.market_total_max(filter.key, sid)"
                    /> ISK
                  </td>
                </tr>
              </tbody>
            </n-table>
          </n-card>

          <p-export
            v-model:show="show_export"
            :data="project.market_info(filter.key, sid)"
            :data-fields="['name', 'count', 's_min', 'a_min', 's_avg', 'a_avg', 's_max', 'a_max']"
            :data-fields-csv="['type_id', 'name', 'count', 's_min', 'a_min', 's_avg', 'a_avg', 's_max', 'a_max']"
            :pid="$route.params.pid"
          />
        </n-tab-pane>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NSpace, NTable, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { SystemId } from '@/utils';
import { PROJECT_ROUTE } from '@/event_bus';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

import PExport from '@/project/MExport.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NSpace,
    NTable,
    NTabPane,
    NTabs,

    FormatNumber,
    ItemIcon,

    PExport,
    PHeader,
    WProject,
  }
})
export default class ProjectMarketView extends Vue {
  public show_export: boolean = false;

  public sid: SystemId = 30000142; // Jita

  public filters: { label: string, key: string }[] = [{
    label: 'Market buy',
    key:   'BUY'
  }, {
    label: 'Market sell',
    key:   'SELL'
  }];

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_market`
    );
  }
}
</script>
