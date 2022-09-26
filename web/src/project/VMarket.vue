<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

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
              <th width="50px" style="text-align: right;">Buy (Single)</th>
              <th width="50px" style="text-align: right;">Sell (Single)</th>
              <th width="50px" style="text-align: right;">Buy (Total)</th>
              <th width="50px" style="text-align: right;">Sell (Total)</th>
            </tr>
            <tr v-for="market in project.market.items" :key="market.type_id">
              <td><eve-icon :id="market.type_id" type="icon" /></td>
              <td>{{ market.name }}</td>
              <td>
                <format-number :value="market.amount" />
              </td>
              <td style="text-align: right;">
                <format-number :value="market.buy_price" with-comma /> ISK
              </td>
              <td style="text-align: right;">
                <format-number :value="market.sell_price" with-comma /> ISK
              </td>
              <td style="text-align: right;">
                <format-number :value="market.buy_price_total" with-comma /> ISK
              </td>
              <td style="text-align: right;">
                <format-number :value="market.sell_price_total" with-comma /> ISK
              </td>
            </tr>
            <tr>
              <td></td>
              <td>Total</td>
              <td></td>
              <td></td>
              <td></td>
              <td style="text-align: right;">
                <format-number
                  with-comma
                  :value="project.market.buy_price"
                /> ISK
              </td>
              <td style="text-align: right;">
                <format-number
                  with-comma
                  :value="project.market.sell_price"
                /> ISK
              </td>
            </tr>
          </tbody>
        </n-table>
      </n-card>

      <!--p-export
        v-model:show="show_export"
        :data="project.market"
        :data-fields="['name', 'count', 's_min', 'a_min', 's_avg', 'a_avg', 's_max', 'a_max']"
        :data-fields-csv="['type_id', 'name', 'count', 's_min', 'a_min', 's_avg', 'a_avg', 's_max', 'a_max']"
        :pid="$route.params.pid"
      /-->
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NSpace, NTable, NTabs, NTabPane } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import FormatNumber from '@/components/FormatNumber.vue';
import EveIcon from '@/components/EveIcon.vue';

import PExport from '@/project/MExport.vue';
import PHeader from '@/project/CHeader.vue';
import WProject from '@/project/wrapper/Project.vue';

@Options({
  components: {
    NButton,
    NCard,
    NSpace,
    NTable,
    NTabPane,
    NTabs,

    FormatNumber,
    EveIcon,

    PExport,
    PHeader,
    WProject,
  }
})
export default class ProjectMarketView extends Vue {
  public show_export: boolean = false;

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );
  }
}
</script>
