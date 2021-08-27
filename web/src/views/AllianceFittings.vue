<template>
  <n-card title="Alliance fittings">
    <n-form>
      <n-form-item label="Name" path="fitting.name">
        <n-input v-model:value="fitting.name" placeholder="Group name" />
      </n-form-item>

      <n-form-item label="Name" path="fitting.url">
        <n-input v-model:value="fitting.url" placeholder="Wiki-Url" />
      </n-form-item>

      <n-form-item
        :label="'Fitting ' + (index + 1)"
        v-for="(fitting, index) in [...Array(fittings).keys()]" :key="fitting"
      >
        <n-input
          v-model:value="fittingInputs[fitting]"
          type="textarea"
        />
      </n-form-item>

      <n-button-group>
        <n-button @click="fittings += 1">Add fitting</n-button>
        <n-button @click="fittings -= 1">Remove fitting</n-button>
      </n-button-group>

      <n-form-item label="How to fit" path="fitting.how_to_fit">
        <n-input
          v-model:value="fitting.how_to_fit"
          placeholder="How to fit"
          type="textarea"
        />
      </n-form-item>

      <n-form-item label="How to fly" path="fitting.how_to_fly">
        <n-input
          v-model:value="fitting.how_to_fly"
          placeholder="How to fly"
          type="textarea"
        />
      </n-form-item>
    </n-form>

    <n-button @click="save">Save</n-button>

    <n-table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Url</th>
          <th>Can Fly</th>
          <th>Missing</th>
          <th></th>
        </tr>
      </thead>
      <tbody v-for="fetch in fetched" :key="fetch.name">
        <tr>
          <td>{{ fetch.name }}</td>
          <td>{{ fetch.url }}</td>
          <td>{{ countCanFly(fetch.fittings) }} / {{ fetch.fittings.length }}</td>
          <td>{{ countMissing(fetch.fittings) }} / {{ totalSkills(fetch.fittings) }}</td>
          <td>
            <n-button-group>
              <n-button @click="fetch.show = !fetch.show">Details</n-button>
              <n-button type="error" ghost @click="deleteFitting(fetch.id)">Delete</n-button>
            </n-button-group>
          </td>
        </tr>
        <tr v-for="fitting in fetch.fittings" :key="fitting.name">
          <td colspan="5" v-if="fetch.show ? true : false">
            <h3>{{ fitting.name }}</h3>

            <n-table>
              <tbody>
                <tr v-for="type_id in fitting.type_ids" :key="type_id">
                  <td><name-by-id :id="type_id.type_id" /></td>
                  <td>{{ type_id.can_use }}</td>
                </tr>
              </tbody>
            </n-table>
          </td>
        </tr>
      </tbody>
    </n-table>
  </n-card>
</template>

<script lang="ts">
import { NButton, NButtonGroup, NCard, NForm, NFormItem, NInput, NTable } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

import NameById from '@/components/NameById.vue';
import { NameService } from '@/services/name';
import { AllianceService, IFitting, IFittingGroup, IFittingTypeId,
INewFittingGroup } from '@/services/alliance';

@Options({
  components: {
    NButton,
    NButtonGroup,
    NCard,
    NForm,
    NFormItem,
    NInput,
    NTable,

    NameById,
  }
})
export default class CorpBlueprints extends Vue {
  public busy: boolean = false;
  public fittings: number = 1;
  public fittingInputs: string[] = [];

  public fitting: INewFittingGroup = {
    fittings: [],
    name: '',
    url: '',
  };
  public error: string = '';

  public fetched: IFittingGroup[] = [];

  public async created() {
    this.fetched = (await AllianceService.get_fittings())
      .map(x => {
        x
          .fittings
          .map(y => {
            let reduced = new Set<IFittingTypeId>();
            for (let z of y.type_ids) {
              reduced.add(z);
            }
            y.type_ids = Array.from(reduced);
            return y;
          })

        return x
      })
      .sort((a, b) => a.name.localeCompare(b.name));
  }

  public countCanFly(fittings: IFitting[]): number {
    let count = 0;
    for (let fitting of fittings) {
      if (fitting.type_ids.filter(x => !x.can_use).length === 0) {
        count += 1;
      }
    }
    return count;
  }

  public countMissing(fittings: IFitting[]): number {
    let missing = new Set<number>();
    for (let fitting of fittings) {
      fitting
        .type_ids
        .filter(x => !x.can_use)
        .map(x => missing.add(x.type_id));
    }
    return Array.from(missing).length;
  }

  public totalSkills(fittings: IFitting[]): number {
    let typeIds = new Set<number>();
    for (let fitting of fittings) {
      for (let typeId of fitting.type_ids) {
        typeIds.add(typeId.type_id);
      }
    }
    return Array.from(typeIds).length;
  }

  public async save() {
    for (let fit of this.fittingInputs) {
      const parsedFitting: { name: string, type_ids: number[] } = {
        name: '',
        type_ids: []
      };

      const splitted = fit.split('\n');

      let type_names: Set<string> = new Set();
      for (let split of splitted) {
        if (split === '') {
          continue;
        }

        if (
          split.startsWith('[') &&
          split.endsWith(']') &&
          split.indexOf('DOC') > 0
        ) {
          parsedFitting.name = split;
        } else {
          let x = split.replace(/ x\d*/, '');
          type_names.add(x);
        }
      }

      const names_resolved = await NameService.resolve_names_to_id(Array.from(type_names));

      for (let split of splitted) {
        if (split === '') {
          continue;
        } else if (split.startsWith('[') && split.endsWith(']')) {
          continue;
        } else {
          let y = split.replace(/ x\d*/, '');
          let name = names_resolved.find(x => x.name === y);

          if (name && name !== undefined) {
            parsedFitting.type_ids.push(name.id);
          } else {
            console.log('Could not resolve name', y)
            return;
          }
        }
      }

      this.fitting.fittings.push(parsedFitting);
    }

    await AllianceService.set_fitting(this.fitting);
    this.fitting = {
      fittings:   [],
      name:       '',
      url:        '',
      how_to_fly: undefined,
      how_to_fit: undefined
    };
    this.fittings = 1;
    this.fittingInputs = [];
  }

  public async deleteFitting(id: string) {
    await AllianceService.delete_fitting(id);
  }
}
</script>
