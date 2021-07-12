<template>
  <n-data-table :columns="columns" :data="projectCosts" />
</template>

<script lang="ts">
import { NDataTable } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { ProjectService, IProjectCost } from '@/services/project';
import { h } from 'vue';

import FormatNumber from '@/components/FormatNumber.vue';
import ProjectCostDetail from '@/components/project/CostDetail.vue';
import NameById from '../NameById.vue';

class Props {
  // Project id
  pid = prop({
    type: String,
    required: true,
  });
}

@Options({
  components: {
    NDataTable,

    FormatNumber,
    ProjectCostDetail
  }
})
export default class ProjectCost extends Vue.with(Props) {
  public busy: boolean = false;

  public columns: any[] = [];
  public projectCosts: IProjectCost[] = [];

  public async created() {
    this.busy = true;

    const id: string = <string>this.$route.params.id;
    let key = 0;
    this.projectCosts = (await ProjectService.cost(id))
      .map(x => {
        key += 1;
        return {
          key,
          ...x
        }
      });

    this.columns = this.createColumns();

    this.busy = false;
  }

  public createColumns() {
    return [
      {
        type: 'expand',
        expandable: (_: any, index: number) => index !== this.projectCosts.length,
        renderExpand: (rowData: IProjectCost) => {
          return h(
            ProjectCostDetail,
            {
              cost:  rowData,
            },
          )
        }
      }, {
        title: 'Blueprint',
        key:   'name',
        render(rowData: IProjectCost) {
          return h(
            NameById,
            { id: rowData.bpid }
          )
        }
      }, {
        title: 'Cost',
        key:   'total_cost',
        render(rowData: IProjectCost) {
          return h(
            FormatNumber,
            { value: rowData.total_cost, isTime: false },
          )
        }
      }
    ]
  }
}
</script>
