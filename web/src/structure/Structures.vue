<template>
  <div>
    <n-page-header v-if="!busy">
      <template #title>
        <h1>
          Structures
        </h1>
      </template>
    </n-page-header>

    <n-card content-style="padding: 0" v-if="!busy">
      <n-space justify="end" style="margin: 10px;" v-if="!busy">
        <n-button
          :disabled="!selected_structure"
          @click="
            $router.push({ name: 'projects_overview', params: {
              sid: selected_structure || ''
            } })
          "
        >
          View structure
        </n-button>

        <n-button
          @click="show_confirm = true"
          :disabled="!selected_structure"
        >
          Delete project
        </n-button>

        <n-button
          @click="$router.push({ name: 'settings_structures_new' })"
          type="info"
        >
          New structure
        </n-button>
      </n-space>

      <n-table v-if="!busy && structures.length > 0">
        <thead>
          <tr>
            <th width="10px"></th>
            <th width="500px">Name</th>
            <th width="500px">Location</th>
            <th width="500px">Type</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="structure in structures" :key="structure.structure">
            <td>
              <n-checkbox
                :checked="selected_structure"
                :checked-value="structure.structure"
                @update:checked="handle_select"
                unchecked-value="undefined"
                name="select_structure"
              >
              </n-checkbox>
            </td>
            <td>
              <n-button text type="info">
                <router-link
                  :to="
                    {
                      name: 'structure_overview',
                      params: { pid: structure.structure }
                    }
                  "
                  style="color: inherit; text-decoration: none"
                >
                  TODO
                </router-link>
              </n-button>
            </td>
            <td>
              TODO
            </td>
          </tr>
        </tbody>
      </n-table>

      <n-empty
        description="No structures yet"
        size="large"
        style="margin: 5%"
        v-if="!busy && structures.length === 0"
      />

      <loading
        description="Loading"
        :busy="busy"
      />

      <confirm-dialog
        v-model:show="show_confirm"
        :confirm="confirm_delete"
        :resource="structure_name()"
      >
        Are you sure you want to delete {{ structure_name() }}?
        This action will delete everything that is stored about this project.
        This is not be undone.<br>
        Please type in 'delete' to confirm.
      </confirm-dialog>
    </n-card>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NCheckbox, NEmpty, NPageHeader, NSpace, NTable } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import { Service, IStructure } from '@/structure/service';

import ConfirmDialog from '@/components/ConfirmDialog.vue';
import Loading from '@/components/Loading.vue';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NEmpty,
    NPageHeader,
    NSpace,
    NTable,

    ConfirmDialog,
    Loading,
  }
})
export default class ProjectsView extends Vue {
  public busy: boolean         = false;
  public show_confirm: boolean = false;

  public selected_structure: string | undefined = '';

  public structures: IStructure[] = [];

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      this.$route.name
    );

    this.busy = true;
    await this.load();
    this.busy = false;
  }

  public async confirm_delete() {
    if (!this.selected_structure) { return; }

    await Service.remove(this.selected_structure);
    await this.load();

    this.selected_structure = undefined;
    this.show_confirm = false;
  }

  public handle_select(sid: string) {
    if (sid === 'undefined') {
      this.selected_structure = undefined;
      return;
    }

    this.selected_structure = sid;
  }

  public structure_name(): string {
    let info = this.structures
      .find(x => x.project === this.selected_structure) || <IStructure>{ name: '' };
    return info.name;
  }

  private async load() {
    this.structures = await Service.all();
  }
}
</script>
