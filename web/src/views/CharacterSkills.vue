<template>
  <div>
    <v-skeleton-loader
      class="mx-auto"
      type="card"
      v-if="busy"
    ></v-skeleton-loader>

    <v-card class="mt-5" v-if="!busy">
      <v-card-title>
        Skills
      </v-card-title>

      <v-card-text>
        <v-expansion-panels>
          <v-expansion-panel v-for="(entry, i) in skillplans" :key="i">
            <v-expansion-panel-header>
              {{ entry.name }}
              <v-spacer />
              ToDo: {{ entry.todo }} / Queued: {{ entry.queued }} / Done: {{ entry.done }}
            </v-expansion-panel-header>

            <v-expansion-panel-content>
              <v-simple-table dense>
                <template v-slot:default>
                  <thead>
                    <th>Skill</th>
                    <th>Level</th>
                    <th>State</th>
                  </thead>
                  <tbody>
                  <tr
                    v-for="(skill, k) in entry.skills"
                    :key="k"
                    :class="state_as_css(skill.state)"
                  >
                      <td><c-name-by-id :id="Number(skill.skill_id)"/></td>
                      <td class="text-center">{{ skill.level }}</td>
                      <td class="text-center">{{ skill.state }}</td>
                    </tr>
                  </tbody>
                </template>
              </v-simple-table>
            </v-expansion-panel-content>
          </v-expansion-panel>
        </v-expansion-panels>
      </v-card-text>
    </v-card>
  </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue } from 'vue-property-decorator';

@Component
export default class MySkills extends Vue {
  public learnedSkills: ICharacterSkill[]      = [];
  public queuedSkills:  ICharacterSkillQueue[] = [];
  public skillplans:    ISkillplan[]           = [];

  public busy = false;

  public async created() {
    this.busy = true;

    this.learnedSkills = (await axios.get(`/api/character/skills`)).data;
    this.queuedSkills  = (await axios.get(`/api/character/skillqueue`)).data;

    this.skillplans = JSON
      .parse((await axios.get(`/api/character/corp/skillplans`)).data)
      .map(this.skill_state)
      .map(this.todo_queue_done);

    this.busy = false;
  }

  public skill_state(plan: ISkillplan): ISkillplan {
    plan
      .skills
      .forEach((skill: ISkillplanEntry) => {
        const learned = this.learnedSkills.find(x => x.skill_id === skill.skill_id);
        const queued = this.queuedSkills.filter(x => x.skill_id === skill.skill_id);

        if (learned && learned.active_skill_level > skill.level) {
          skill.state = 'DONE';
        } else if (queued && queued.find(x => x.finished_level >= skill.level)) {
          skill.state = 'QUEUED';
        } else {
          skill.state = 'TODO';
        }
      });
    return plan;
  }

  public todo_queue_done(plan: ISkillplan): ISkillplan {
    plan.todo   = 0;
    plan.queued = 0;
    plan.done   = 0;

    plan
      .skills
      .forEach(x => {
        if (x.state === 'DONE') {
          plan.done += 1;
        } else if (x.state === 'QUEUED') {
          plan.queued += 1;
        } else {
          plan.todo += 1;
        }
      });
    return plan;
  }

  public state_as_css(state: SkillState): string {
    if (state === 'DONE') {
      return 'green darken-3';
    } else if (state === 'QUEUED') {
      return 'orange darken-3';
    } else {
      return 'red darken-3';
    }
  }
}

type SkillState = 'TODO' | 'QUEUED' | 'DONE';

interface ICharacterSkill {
    active_skill_level: number;
    skill_id:           number;
}

interface ICharacterSkillQueue {
    finished_level: number;
    queue_position: number;
    skill_id:       number;
}

interface ISkillplan {
  name:   string;
  skills: ISkillplanEntry[];

  todo:   number;
  queued: number;
  done:   number;
}

interface ISkillplanEntry {
  skill_id: number;
  level:    number;
  state:    SkillState;
}
</script>
