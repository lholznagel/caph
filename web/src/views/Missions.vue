<template>
  <div>
    <v-card>
      <v-card-title>Add mission</v-card-title>

      <v-card-text>
        <v-timeline dense>
          <v-timeline-item>
            <v-card>
              <v-card-title>
                Misc info
              </v-card-title>

              <v-card-text>
                <v-row>
                  <v-col cols="12" md="4">
                    <v-select
                      v-model="missionInfo.level"
                      dense
                      :items="missionLevels"
                      @change="filterNames()"
                      required />
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-select
                      v-model="missionInfo.type"
                      dense
                      :items="missionTypes"
                      @change="filterNames()"
                      required />
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-select
                      v-model="missionInfo.key"
                      dense
                      item-value="key"
                      item-text="name"
                      :items="selectableMissions"
                      :disabled="selectableMissions.length === 0"
                      @change="setSelectedMission()"
                      required />
                  </v-col>
                </v-row>

                <v-row>
                  <v-col cols="12" md="4">
                    <v-text-field
                      label="Payment"
                      type="number"
                      prefix="ISK"
                      suffix="k"
                      v-model="missionInfo.payment"
                      dense
                      required />
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-text-field
                      label="Bonus"
                      type="number"
                      prefix="ISK"
                      suffix="k"
                      v-model="missionInfo.bonus"
                      dense
                      required />
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-text-field
                      label="LP"
                      type="number"
                      v-model="missionInfo.lp"
                      dense
                      required />
                  </v-col>
                </v-row>

                <v-row>
                  <v-col cols="12" md="6">
                    <v-text-field
                      label="Missles"
                      type="number"
                      v-model="missilesStart"
                      dense
                      required />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-text-field
                      label="Jumps"
                      type="number"
                      v-model="missionInfo.jumps"
                      dense
                      required />
                  </v-col>
                </v-row>
              </v-card-text>

              <v-card-actions>
                <v-btn
                  @click="setTimer('START')"
                  v-if="missionInfo.timeStart === undefined"
                  color="primary"
                  plain
                >Start</v-btn>
              </v-card-actions>
            </v-card>
         </v-timeline-item>

          <v-timeline-item v-for="(ms, idx) in selectedMission.pockets" :key="idx">
            <v-card>
              <v-card-title>
                {{ ms.name }}
              </v-card-title>
              <v-card-subtitle>
                {{ ms.note }}
              </v-card-subtitle>

              <v-card-text>
                <v-card v-for="(msg, idx) in ms.groups" :key="idx">
                  <v-card-title>
                    {{ msg.name }}
                  </v-card-title>

                  <v-card-subtitle>
                    {{ msg.note }}
                  </v-card-subtitle>

                  <v-card-text>
                    <v-simple-table>
                      <template v-slot:default>
                        <thead>
                          <tr>
                            <th>Ship</th>
                            <th>Name</th>
                            <th>Count</th>
                            <th>Bounty</th>
                            <th>WD</th>
                            <th>Ewar</th>
                            <th>Trigger</th>
                            <th>Loot</th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr v-for="(e, idx) in msg.enemies" :key="idx">
                            <td width="100">{{ e.getShip() }}</td>
                            <td width="500">{{ e.getEnemyNames() }}</td>
                            <td width="50" >{{ e.getCount() }}
                            <td width="200">{{ e.getBounty() }}</td>
                            <td width="200">{{ e.wd }}</td>
                            <td width="200">{{ e.ewar }}</td>
                            <td width="50" >
                              <v-icon v-if="e.trigger" color="red">
                                mdi-skull-crossbones
                              </v-icon>
                            </td>
                            <td width="50" >
                              <v-icon v-if="e.loot" color="green">
                                mdi-treasure-chest
                              </v-icon>
                            </td>
                          </tr>
                        </tbody>
                      </template>
                    </v-simple-table>
                  </v-card-text>
                </v-card>
              </v-card-text>

              <v-card-actions>
                <v-btn color="primary" @click="pocketDone(idx)" :disabled="!missionInfo.timeStart" plain>Next</v-btn>
              </v-card-actions>
            </v-card>
          </v-timeline-item>

          <v-timeline-item>
            <v-card>
              <v-card-title>
                Finish mission
              </v-card-title>

              <v-card-text>
                <v-text-field
                  label="Missles"
                  type="number"
                  v-model="missilesEnd"
                  @change="setMissiles()"
                  dense
                  required />

                <v-textarea label="Loot" lines="5" v-model="missionInfo.loot" dense filled></v-textarea>
              </v-card-text>

              <v-card-actions>
                <v-btn
                  @click="finish"
                  color="primary"
                  :disabled="!missionInfo.timeStart"
                  plain
                >Finish</v-btn>
              </v-card-actions>
            </v-card>
          </v-timeline-item>
        </v-timeline>
      </v-card-text>
    </v-card>

    <pre>{{ missionInfo }}</pre>
  </div>
</template>

<script lang="ts">
import { Mission, missions } from '@/constants/mission/missions';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class AddMission extends Vue {
  public missionLevels: ISelect[] = [
    { text: 'Level 1', value: 'LEVEL_1' },
    { text: 'Level 2', value: 'LEVEL_2' },
    { text: 'Level 3', value: 'LEVEL_3' },
    { text: 'Level 4', value: 'LEVEL_4' },
    { text: 'Level 5', value: 'LEVEL_5' },
  ];
  public missionTypes: ISelect[] = [
    { text: 'Mining',   value: 'MINING' },
    { text: 'Security', value: 'SECURITY' },
  ];

  public selectedMission: Mission | {} = {};
  public selectableMissions: Mission[] = [];

  public missionInfo: IMissionInfo = {
    level:   'LEVEL_3',
    type:    'SECURITY',
    pockets: {}
  };

  public missionRunning: boolean = false;
  public missilesStart:   number = 0;
  public missilesEnd:     number = 0;

  public created() {
    this.filterNames();
  }

  public filterNames() {
    this.selectableMissions = missions
      .filter(x => x.level === this.missionInfo.level)
      .filter(x => x.typ  === this.missionInfo.type);

    if (this.selectableMissions.length > 0) {
      this.missionInfo.key = this.selectableMissions[0].key;
      this.setSelectedMission();
    }
  }

  public setMissiles() {
    this.missionInfo.missiles = Number(this.missilesStart) - Number(this.missilesEnd);
  }

  public setTimer(type: string) {
    const date = new Date().getTime();
    if (type === 'START') {
      Vue.set(this.missionInfo, 'timeStart', date);
    } else if (type === 'END') {
      Vue.set(this.missionInfo, 'timeEnd', date);
    } else {
      Vue.set(this.missionInfo.pockets, type, date);
    }
  }

  public setSelectedMission() {
    this.selectedMission = this
      .selectableMissions
      .find(x => x.key === this.missionInfo.key) || {};
  }

  public pocketDone(idx: number) {
    this.setTimer(idx.toString());
  }

  public finish() {
    this.setTimer('END');
  }
}

interface ISelect {
  text:   string;
  value:  string;
}

interface IMissionInfo {
  level:      string;
  type:       string;
  key?:       string;

  loot?:      string;
  missiles?:  number;

  payment?:   number;
  bonus?:     number;
  lp?:        number;

  timeStart?: number;
  timeEnd?:   number;
  pockets:    { [key: number]: number };
}
</script>
