<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Characters
        </h1>
      </template>
    </n-page-header>

    <loading
      description="Loading characters"
      :busy=busy
    />

    <n-card content-style="padding: 0" v-if="!busy">
      <n-space justify="end" style="margin: 10px">
        <n-button
          @click="refresh"
          :disabled="!selected"
          :loading="refresh_active"
        >
          Refresh character
        </n-button>
        <n-button
          @click="remove"
          :disabled="!selected"
        >
          Remove character
        </n-button>

        <n-button
          @click="add"
          type="info"
        >
          Add character
        </n-button>
      </n-space>

      <n-table v-if="!busy">
        <thead>
          <tr>
            <th width="24"></th>
            <th width="34px"></th>
            <th width="40px"></th>
            <th width="500px">Name</th>
            <th width="40px"></th>
            <th width="500px">Corporation</th>
            <th width="40px"></th>
            <th width="500px">Alliance</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="c in characters" :key="c.character_id">
            <tr>
              <td>
                <n-icon size="22">
                  <angle-right
                    style="cursor: pointer"
                    @click="c.open = true"
                    v-if="!c.open"
                  />
                  <angle-down
                    style="cursor: pointer"
                    @click="c.open = false"
                    v-if="c.open"
                  />
                </n-icon>
              </td>
              <td>
                <n-checkbox
                  :checked="selected"
                  :checked-value="c.character_id"
                  @update:checked="handle_select"
                  unchecked-value="undefined"
                  name="selected"
                >
                </n-checkbox>
              </td>
              <td>
                <character :id="c.character_id" />
              </td>
              <td>
                {{ c.character }}
              </td>
              <td>
                <corporation :id="c.corporation_id" />
              </td>
              <td>
                {{ c.corporation }}
              </td>
              <td v-if="c.alliance_id">
                <alliance :id="c.alliance_id" />
              </td>
              <td v-if="!c.alliance_id"></td>
              <td v-if="c.alliance_id">
                {{ c.alliance }}
              </td>
              <td v-if="!c.alliance_id"></td>
            </tr>
            <tr v-if="c.open">
              <td></td>
              <td colspan="8" style="padding-top: 0; padding-right: 0; padding-bottom: 0">
                <n-list>
                  <template #header>
                    <h3>Given permissions:</h3>
                  </template>
                  <template #footer>
                    <n-button @click="add_corp_blueprints(c.character_id)">Add corporation permissions</n-button>
                  </template>

                  <n-list-item>
                    <div v-for="p in c.esi_tokens" :key="p">
                      {{ p }}<br>
                    </div>
                  </n-list-item>
                </n-list>
              </td>
            </tr>
          </template>
        </tbody>
      </n-table>

      <confirm-dialog
        v-if="selected"
        v-model:show="show_confirm"
        :confirm="confirm_remove"
        :resource="characters.find(x => x.character_id === parseInt(selected)).character"
      >
        Are you sure you want to delete {{ characters.find(x => x.character_id === parseInt(selected)).character }}?<br>
        You can add the character back to a later time.<br>
        Please type in 'delete' to confirm.
      </confirm-dialog>
    </n-card>
  </div>
</template>

<script lang="ts">
import { AngleDown, AngleRight } from '@vicons/fa';
import { CharacterId } from '@/utils';
import { CharacterService, ICharacter } from '@/services/character';
import { events } from '@/main';
import { NButton, NCard, NCheckbox, NIcon, NList, NListItem, NPageHeader, NTable, NSpace, useMessage } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { ROUTE_CHANGE } from '@/event_bus';

import Alliance from '@/components/Alliance.vue';
import Character from '@/components/Character.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import Corporation from '@/components/Corporation.vue';
import Loading from '@/components/Loading.vue';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NIcon,
    NList,
    NListItem,
    NPageHeader,
    NSpace,
    NTable,

    AngleDown,
    AngleRight,

    Alliance,
    Character,
    ConfirmDialog,
    Corporation,
    Loading
  }
})
export default class CharacterSettings extends Vue {
  public busy: boolean = false;
  public refresh_active: boolean = false;

  public characters: ICharacter[] = [];
  public selected: number = 0;
  public show_confirm: boolean = false;

  public message = useMessage();

  public async created() {
    events.$emit(
      ROUTE_CHANGE,
      this.$route.name
    );

    this.busy = true;
    await this.load();
    this.busy = false;
  }

  public add() {
    CharacterService.add();
  }

  public async refresh() {
    if (!this.selected) { return; }

    this.refresh_active = true;

    await CharacterService.refresh(this.selected);
    this.selected = 0;

    this.refresh_active = false;
  }

  public remove() {
      if (!this.selected) { return; }

    if (this.selected === (<any>window).whoami.character_id) {
      this.message.warning('You cannot delete the main character.');
      return;
    }

    this.show_confirm = true;
  }

  public async confirm_remove() {
    if (!this.selected) { return; }

    CharacterService.remove(this.selected);
    this.selected = 0;
    this.show_confirm = false;

    this.busy = true;
    await this.load();
    this.busy = false;
  }

  public handle_select(cid: number | string) {
    if (cid === 'undefined') {
      this.selected = 0;
      return;
    }
    this.selected = <number>cid;
  }

  public add_corp_blueprints(cid: CharacterId) {
    window.location.href = `/api/v1/auth/scope/${cid}/corporation_blueprints`;
  }

  private async load() {
    this.characters = [];

    let character      = await CharacterService.whoami();
    let character_alts = await CharacterService.alts();
    this.characters.push(character);

    this.characters = this.characters.concat(character_alts);
  }
}
</script>
