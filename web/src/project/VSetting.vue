<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy, project }">
      <p-header />

      <n-tabs
        type="line"
        v-if="!busy"
      >
        <n-tab-pane name="General">
          <n-card>
            <n-space vertical>
              <n-text>Project name</n-text>
              <n-input v-model:value="config.name" placeholder="Name" />

              <n-text>Items to produce</n-text>
              <n-dynamic-input
                v-model:value="config.products"
                :on-create="add_item_to_produce"
                :min="1"
                #="{ value }"
              >
                <div style="width: 100%;">
                  <div style="display: flex; align-items: center;">
                    <n-input-number
                      v-model:value="value.count"
                      style="margin-right: 5px"
                    />
                    <n-select
                      :options="buildable_items"
                      v-model:value="value.type_id"
                      placeholder="Select Item"
                      filterable
                    />
                  </div>
                </div>
              </n-dynamic-input>
              <resolve v-model="project.products" />
            </n-space>

            <template #action>
              <n-space justify="end">
                <n-button
                  :disabled="
                    !config.name ||
                    !config.products ||
                    !config.products[0] ||
                    !config.products[0].type_id"
                  @click="edit_project"
                  type="info"
                >Save</n-button>
              </n-space>
            </template>
          </n-card>
        </n-tab-pane>
        <n-tab-pane name="Members">
          <n-card content-style="padding: 0">
            <n-space justify="end" style="margin: 10px">
              <n-button
                @click="kick_member"
                :disabled="!selected_member"
              >
                Kick member
              </n-button>

              <n-button
                @click="show_invite = true"
                type="info"
              >
                Invite member
              </n-button>
            </n-space>

            <n-table v-if="project.members">
              <thead>
                <tr>
                  <th width="10px"></th>
                  <th width="40px"></th>
                  <th width="500px">Name</th>
                  <th width="40px"></th>
                  <th width="500px">Corporation</th>
                  <th width="40px"></th>
                  <th width="500px">Alliance</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="member in project.members" :key="member.character_id">
                  <td>
                    <n-checkbox
                      :checked="selected_member"
                      :checked-value="member.character_id"
                      @update:checked="handle_member_select"
                      unchecked-value="undefined"
                      name="selected_member"
                    >
                    </n-checkbox>
                  </td>
                  <td>
                    <character :id="member.character_id" />
                  </td>
                  <td>
                    {{ member.character_name }}
                  </td>
                  <td>
                    <corporation :id="member.corporation_id" />
                  </td>
                  <td>
                    {{ member.corporation_name }}
                  </td>
                  <td v-if="member.alliance_id">
                    <alliance :id="member.alliance_id" />
                  </td>
                  <td v-if="!member.alliance_id"></td>
                  <td v-if="member.alliance_id">
                    {{ member.alliance_name }}
                  </td>
                  <td v-if="!member.alliance_id"></td>
                </tr>
              </tbody>
            </n-table>

            <confirm-dialog
              v-if="selected_member"
              v-model:show="show_confirm_kick_member"
              :confirm="confirm_kick_member"
              :resource="project.members.find(x => x.character_id === selected_member).character_name"
            >
              Are you sure you want to kick {{ project.members.find(x => x.character_id === selected_member).character_name }}?
              The character will no longer have access to the project.
              Please type in 'delete' to confirm.
            </confirm-dialog>

            <p-invite-modal
              v-model:show="show_invite"
              :pid="$route.params.pid"
            />
          </n-card>
        </n-tab-pane>
      </n-tabs>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NCheckbox, NDynamicInput, NInput, NInputNumber, NSpace,
NTabs, NSelect, NTable, NTabPane, SelectOption, NText, useMessage } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { Project } from './project';
import { Service, IConfig } from '@/project/service';
import { ProjectId } from '@/project/project';
import { ItemService } from '@/services/item';

import Alliance from '@/components/Alliance.vue';
import Character from '@/components/Character.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import Corporation from '@/components/Corporation.vue';
import Resolve from '@/components/Resolve.vue';

import PHeader from '@/project/CHeader.vue';
import PInviteModal from '@/project/MInvite.vue';
import WProject from '@/project/WProject.vue';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NDynamicInput,
    NInput,
    NInputNumber,
    NSelect,
    NSpace,
    NTable,
    NTabPane,
    NTabs,
    NText,

    Alliance,
    Character,
    ConfirmDialog,
    Corporation,
    Resolve,

    PHeader,
    PInviteModal,
    WProject
  }
})
export default class ProjectOverviewView extends Vue {
  public buildable_items: SelectOption[] = [];

  public selected_member: string | undefined = '';
  public show_confirm_kick_member: boolean = false;
  public show_invite: boolean = false;

  public config: IConfig  = <IConfig>{  };
  public project: Project = <Project>{  };

  public message = useMessage();

  public async created() {
    events.$emit(
      PROJECT_ROUTE,
      `projects_setting`
    );

    this.buildable_items = (await ItemService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });

    this.config = (
      await Service.by_id(<string>this.$route.params.pid)
    ).info;

    this.project = await Service.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();
  }

  public async edit_project() {
    // The selector adds it as an object, we want it as an array
    await Service.edit(
      <string>this.$route.params.pid,
      this.config
    );

    this.$router.push({
      name: 'projects_overview',
      params: {
        pid: this.$route.params.pid
      }
    });
  }

  public add_item_to_produce() {
    return {
      type_id: undefined,
      count:   1,
    }
  }

  public handle_member_select(cid: string) {
    if (cid === 'undefined') {
      this.selected_member = undefined;
      return;
    }

    this.selected_member = cid;
  }

  public kick_member() {
    if (!this.selected_member) { return; }

    if (parseInt(this.selected_member) === this.project.info.owner) {
      this.message.warning('You cannot delete the owner.');
      return;
    }

    this.show_confirm_kick_member = true;
  }

  public confirm_kick_member() {
    if (!this.selected_member) { return; }

    this.project.kick_member(parseInt(<string>this.selected_member));
    this.selected_member = undefined;
    this.show_confirm_kick_member = false;
  }
}
</script>
