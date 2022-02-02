<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Characters
        </h1>
      </template>
    </n-page-header>

    <n-card content-style="padding: 0">
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

      <n-skeleton text v-if="busy" :repeat="5" />

      <n-table v-if="!busy">
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
          <tr v-for="character in characters" :key="character.character_id">
            <td>
              <n-checkbox
                :checked="selected"
                :checked-value="character.character_id"
                @update:checked="handle_select"
                unchecked-value="undefined"
                name="selected"
              >
              </n-checkbox>
            </td>
            <td>
              <character :id="character.character_id" />
            </td>
            <td>
              {{ character.character }}
            </td>
            <td>
              <corporation :id="character.corporation_id" />
            </td>
            <td>
              {{ character.corporation }}
            </td>
            <td v-if="character.alliance_id">
              <alliance :id="character.alliance_id" />
            </td>
            <td v-if="!character.alliance_id"></td>
            <td v-if="character.alliance_id">
              {{ character.alliance }}
            </td>
            <td v-if="!character.alliance_id"></td>
            <td>
              <n-button @click="add_corp_blueprints(character.character_id)">Add Corporation</n-button>
            </td>
          </tr>
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
import {CharacterService, ICharacter} from '@/services/character';
import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';
import { NButton, NCard, NCheckbox, NPageHeader, NTable, NSkeleton, NSpace, useMessage } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

import Alliance from '@/components/Alliance.vue';
import Character from '@/components/Character.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import Corporation from '@/components/Corporation.vue';
import { CharacterId } from '@/utils';

@Options({
  components: {
    NButton,
    NCard,
    NCheckbox,
    NPageHeader,
    NSkeleton,
    NSpace,
    NTable,

    Alliance,
    Character,
    ConfirmDialog,
    Corporation,
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
