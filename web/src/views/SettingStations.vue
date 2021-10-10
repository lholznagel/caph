<template>
  <n-card title="POS">
    <n-skeleton text v-if="busy" :repeat="5" />

    <n-table v-if="!busy">
      <thead>
        <tr>
          <th>ID</th>
          <th>Name</th>
          <th>System</th>
          <th width="300">Station type</th>
          <th width="75"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in entries" :key="entry.id">
          <td>{{ entry.id }}</td>
          <td>{{ entry.name }}</td>
          <td><system-name :id="entry.system_id" /></td>
          <td>{{ entry.structure }}</td>
          <td>
            <n-button @click="delete_pos(entry.id)" type="error" ghost>Delete</n-button>
          </td>
        </tr>
        <tr>
          <td>
            <n-input-number
              v-model:value="new_station.id"
              placeholder="POS ID"
              clearable
            />
          </td>
          <td>
            <n-input
              v-model:value="new_station.name"
              type="text"
              placeholder="POS Name"
              clearable
            />
          </td>
          <td>
            <system-selector
              v-model:value="new_station.system_id"
            />
          </td>
          <td>
            <n-select
              v-model:value="new_station.structure"
              :options="structure_types"
            />
          </td>
          <td>
            <n-button
              @click="add_pos()"
              type="success"
              ghost
            >
              Add
            </n-button>
          </td>
        </tr>
      </tbody>
    </n-table>
  </n-card>
</template>

<script lang="ts">
import { NButton, NCard, NInput, NInputNumber, NSelect, NSkeleton, NTable } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { IStation, UniverseService } from '@/services/universe';

import SystemName from '@/components/SystemName.vue';
import SystemSelector from '@/components/SystemSelector.vue';

@Options({
  components: {
    NButton,
    NCard,
    NInput,
    NInputNumber,
    NSelect,
    NSkeleton,
    NTable,

    SystemName,
    SystemSelector,
  }
})
export default class Settings extends Vue {
  public busy: boolean = false;

  public new_station: IStation = <IStation>{};

  public entries: IStation[] = [];

  public structure_types: any[] = [{
    label: 'Azbel',
    value: 'Azbel'
  }, {
    label: 'Raitaru',
    value: 'Raitaru'
  }, {
    label: 'Sotiyo',
    value: 'Sotiyo'
  }, {
    label: 'Athanor',
    value: 'Athanor'
  }, {
    label: 'Tatara',
    value: 'Tatara'
  }, {
    label: 'Astrahus',
    value: 'Astrahus'
  }, {
    label: 'Fortizar',
    value: 'Fortizar'
  }, {
    label: 'Keepstar',
    value: 'Keepstar'
  }];

  public async created() {
    this.busy = true;

    this.entries = (await UniverseService.stations()).filter(x => x.pos);

    this.busy = false;
  }

  public async add_pos() {
    await UniverseService.add_station(<IStation>this.new_station);
    this.entries = (await UniverseService.stations()).filter(x => x.pos);
    this.new_station = <IStation>{};
  }

  public async delete_pos(id: number) {
    await UniverseService.delete_station(id);
    this.entries = (await UniverseService.stations()).filter(x => x.pos);
  }
}
</script>
