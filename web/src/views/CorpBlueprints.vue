<template>
  <n-card title="Corp Blueprints">
    <pre>{{ error }}</pre>

    <n-input v-model:value="blueprintEntries" type="textarea" />

    <n-button @click="save">Parse</n-button>

    <pre>{{ a }}</pre>
  </n-card>
</template>

<script lang="ts">
import { NButton, NCard, NInput } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { CorporationService, ICorporationBlueprint } from '@/services/corporation';

import NameById from '@/components/NameById.vue';
import { NameService } from '@/services/name';
import { CharacterService } from '@/services/character';

@Options({
  components: {
    NButton,
    NCard,
    NInput,

    NameById,
  }
})
export default class CorpBlueprints extends Vue {
  public busy: boolean = false;
  public blueprintEntries: string = '';

  public a: any[] = [];
  public error: string = '';

  public async save() {
    await CorporationService.deleteBlueprints();

    const blueprints: ICorporationBlueprint[] = [];

    let names = this
      .blueprintEntries
      .split('\n')
      .map(x => {
        const splitted = x.split('\t');
        let name = splitted[0].slice(splitted[0].indexOf(' x ') + 3, splitted[0].length);
        return name;
      });
    let names_resolved = await NameService.resolve_names_to_id(names);
    console.log(names_resolved);

    this.blueprintEntries
      .split('\n')
      .forEach(x => {
        const splitted = x.split('\t');

        let quantity            = 1;
        let material_efficiency = Number(splitted[1]);
        let time_efficiency     = Number(splitted[2]);
        let runs                = Number(splitted[3]);
        let location            = splitted[5];

        let location_id = CharacterService.itemLocationByName(location);
        if (!location_id) {
          this.error = `Could not find Location ${location}`;
          return;
        }

        let name = splitted[0].slice(splitted[0].indexOf(' x ') + 3, splitted[0].length);
        let type_id = Object
          .keys(names_resolved)
          .find(key => names_resolved[key] === name);
        if (splitted[0].indexOf(' x ') !== -1) {
          quantity = Number(splitted[0].slice(0, splitted[0].indexOf(' x ')));
        }

        blueprints.push({
          location_id: location_id.id,
          material_efficiency,
          quantity,
          runs,
          time_efficiency,
          type_id: Number(type_id) || 1,
        });
      });

    this.a = blueprints;
    await CorporationService.deleteBlueprints();
    await CorporationService.setBlueprints(blueprints);

    this.a = 'ok';
  }
}
</script>
