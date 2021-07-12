<template>
  <div>
    <n-table>
      <thead>
        <tr>
          <th>A</th>
          <th>B</th>
          <th>C</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><n-button @click="test = !test">Click mich du sau</n-button></td>
          <td>dsfsdfsdf</td>
          <td>ddddd</td>
        </tr>
        <tr v-show="test" v-if="projectCosts[0]">
          <td colspan="3">
            <project-manufacture-detail :product="projectCosts[0]" />
          </td>
        </tr>
      </tbody>
    </n-table>

    <n-data-table :columns="columns" :data="projectCosts" />
  </div>
</template>

<script lang="ts">
import { NButton, NCollapse, NCollapseItem, NDataTable, NTable, NTag } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { ProjectService, IRequiredProduction } from '@/services/project';
import { h } from 'vue';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '../ItemIcon.vue';
import NameById from '@/components/NameById.vue';
import ProjectManufactureDetail from '@/components/project/ManufactureDetail.vue';

class Props {
  // Project id
  pid = prop({
    type: String,
    required: true,
  });
}

@Options({
  components: {
    NButton,
    NCollapse,
    NCollapseItem,
    NDataTable,
    NTable,
    NTag,

    ProjectManufactureDetail
  }
})
export default class ProjectRequiredProducts extends Vue.with(Props) {
  public busy: boolean = false;

  public test: boolean = false;

  public columns: any[] = [];
  public projectCosts: IRequiredProduction[] = [];

  public async created() {
    this.busy = true;

    const id: string = <string>this.$route.params.id;
    let key = 0;
    this.projectCosts = (await ProjectService.manufacture(id))
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
        renderExpand: (rowData: IRequiredProduction) => {
          return h(
            ProjectManufactureDetail,
            { product: rowData }
          )
        }
      }, {
        title: '',
        key:   'icon',
        width: 48,
        render(rowData: IRequiredProduction) {
          return h(
            ItemIcon,
            {
              id: rowData.pid,
              type: 'icon',
              width: 32
            }
          )
        }
      }, {
        title: 'Product',
        key:   'name',
        render(rowData: IRequiredProduction) {
          return h(
            NameById,
            { id: rowData.pid }
          )
        }
      }, {
        title: '',
        key:   'tags',
        render(rowData: IRequiredProduction) {
          if (rowData.stored >= rowData.quantity) {
            return h(
              NTag,
              { type: 'success' },
              { default: () => 'Done' }
            )
          }
        }
      }, {
        title: 'Required',
        key:   'required',
        render(rowData: IRequiredProduction) {
          return h(
            FormatNumber,
            { value: rowData.quantity, isTime: false }
          )
        }
      }, {
        title: 'Stored',
        key:   'sotred',
        render(rowData: IRequiredProduction) {
          return h(
            FormatNumber,
            { value: rowData.stored, isTime: false }
          )
        }
      }
    ]
  }
}
</script>
