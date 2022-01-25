<template>
  <div>
    <n-skeleton text :repeat="5" v-if="busy" />

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
        <tr v-for="market in market_entries" :key="market.type_id">
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
            <format-number :value="total_min()" with-comma /> ISK
          </td>
          <td style="text-align: right;">
            <format-number :value="total_avg()" with-comma /> ISK
          </td>
          <td style="text-align: right;">
            <format-number :value="total_max()" with-comma /> ISK
          </td>
        </tr>
      </tbody>
    </n-table>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NInput, NInputNumber, NModal, NSkeleton, NSpace,
NTable, NPageHeader, NStatistic, NGrid, NGridItem } from 'naive-ui';
import { IProjectMarket, ProjectService } from '@/project/service';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';
import SystemName from '@/components/SystemName.vue';

class Props {
  pid = prop({
    type:     String,
    required: true,
  });

  sid = prop({
    type:     Number,
    required: true,
    default:  () => 30000142 // Jita
  });

  isSell = prop({
    type: Boolean,
    required: false,
    default:  false
  });
}

@Options({
  components: {
    NButton,
    NCard,
    NInput,
    NInputNumber,
    NModal,
    NSkeleton,
    NSpace,
    NTable,

    NPageHeader,
    NStatistic,
    NGrid,
    NGridItem,

    FormatNumber,
    ItemIcon,
    SystemName
  }
})
export default class ProjectMarket extends Vue.with(Props) {
  public market_entries: IProjectMarket[] = [];

  public busy = false;

  public async created() {
    await this.load();

    this.$watch('sid', () => this.load());
  }

  public async load() {
    this.busy = true;

    if (this.isSell) {
      // TODO: refactor when we have a reload button
      this.market_entries = await ProjectService.market_sell(
        this.pid,
        this.sid
      );
    } else {
      // TODO: refactor when we have a reload button
      this.market_entries = await ProjectService.market_buy(
        this.pid,
        this.sid
      );
    }

    this.busy = false;
  }

  public total_min(): number {
    return this.market_entries
      .map(x => x.a_min)
      .reduce((acc, x) => acc += x, 0);
  }

  public total_avg(): number {
    return this.market_entries
      .map(x => x.a_avg)
      .reduce((acc, x) => acc += x, 0);
  }

  public total_max(): number {
    return this.market_entries
      .map(x => x.a_max)
      .reduce((acc, x) => acc += x, 0);
  }
}
</script>
