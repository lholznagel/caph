<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ project }">
      <p-header />

      <n-card content-style="padding: 0">
        <p-table-header
          :reload="() => project.god()"
          no-primary
        />

        <n-table>
          <thead>
            <tr>
              <th width="48px"></th>
              <th width="200px">Name</th>
              <th width="150px">Required</th>
              <th width="150px">Stored</th>
              <th width="150px">Missing</th>
              <th>Progress</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="entry in project.stored_products()"
              :key="entry.type_id"
            >
              <td><item-icon :id="entry.type_id" type="icon" /></td>
              <td>{{ entry.name }}</td>
              <td><format-number :value="entry.quantity" /></td>
              <td><format-number :value="entry.stored || 0" /></td>
              <td>
                <format-number
                  :value="missing_materials(entry.quantity, entry.stored)"
                />
              </td>
              <td>
                <n-progress
                  type="line"
                  :percentage="calc_progress(entry)"
                  :indicator-placement="'inside'"
                  :status="calc_progress(entry) >= 100 ? 'success' : 'default'"
                />
              </td>
            </tr>
          </tbody>
        </n-table>
      </n-card>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NProgress, NSkeleton, NTable } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import FormatNumber from '@/components/FormatNumber.vue';
import ItemIcon from '@/components/ItemIcon.vue';

import PHeader from '@/project/CHeader.vue';
import PTableHeader from '@/project/CTableHeader.vue';
import WProject from '@/project/WProject.vue';
import { IRequiredMaterial } from '@/project/service';

@Options({
  components: {
    NCard,
    NProgress,
    NSkeleton,
    NTable,

    FormatNumber,
    ItemIcon,

    PHeader,
    PTableHeader,
    WProject,
  }
})
export default class ProjectOverviewView extends Vue {
  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );
  }

  public calc_progress(x: IRequiredMaterial): number {
    return Math.ceil(x.stored / (x.quantity / 100) * 100) / 100 || 0;
  }

  public missing_materials(quantity: number, stored: number): number {
    if (stored > quantity) {
      return 0;
    } else {
      return quantity - stored;
    }
  }
}
</script>
