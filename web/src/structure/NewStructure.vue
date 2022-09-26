<template>
  <div>
    <n-page-header>
      <template #title>
        <h1 style="margin-bottom: 0">
          New structure
        </h1>
      </template>
    </n-page-header>

    <n-form ref="formRef" :model="structure" :rules="rules">
      <n-card title="General Information">
        <n-form-item
          path="name"
          label="Structure name"
        >
          <n-input
            v-model:value="structure.name"
            placeholder="Name"
            @keydown.enter.prevent
          />
        </n-form-item>

        <n-form-item
          path="location"
          label="Location"
        >
          <n-input
            v-model:value="structure.location"
            placeholder="Location"
            @keydown.enter.prevent
          />
        </n-form-item>

        <n-form-item
          path="structure_type"
          label="Strucutre Type"
        >
          <n-select
            v-model:value="structure.structure_type"
            :options="structure_types"
            placeholder="Structure Type"
            @keydown.enter.prevent
            @update:value="selectStructure"
          />
        </n-form-item>
      </n-card>

      <n-card
        style="margin-top: 10px"
        title="Rigs"
      >
        <n-form-item
          v-for="index in [0, 1, 2]"
          :key="index"
          :label="'Slot ' + (index + 1)"
          path=""
        >
          <n-select
            :options="structure_rigs"
            :disabled="!structure.structure_type"
            placeholder="Structure Type"
            @update:value="(v, o) => selectStructureRig(index, v, o)"
            @keydown.enter.prevent
            clearable
            filterable
          />
          <n-switch
            :round="false"
            :disabled="!(structure.rigs && structure.rigs[index])"
            size="large"
            style="margin-left: 10px"
            @update:value="(s) => selectTier(index, s)"
          >
            <template #checked>
              T2
            </template>
            <template #unchecked>
              T1
            </template>
          </n-switch>
        </n-form-item>
      </n-card>

      <n-space style="margin-top: 10px" justify="end">
        <n-button
          @click="$router.back()"
          quaternary
        >Cancel</n-button>

        <n-button
          :disabled="!structure.name"
          @click="validate"
          type="info"
        >
          Add structure
        </n-button>
      </n-space>
    </n-form>

    {{ structure }}
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { FormRules, FormValidationError, NButton, NCard, NForm, NFormItem, NInput, NPageHeader, NSelect, NSpace, NSwitch, NText, SelectGroupOption, SelectOption } from 'naive-ui';

import { Service, IStructure } from '@/structure/service';

@Options({
  components: {
    NButton,
    NCard,
    NForm,
    NFormItem,
    NInput,
    NPageHeader,
    NSelect,
    NSpace,
    NSwitch,
    NText,
  }
})
export default class ProjectNew extends Vue {
  public formRef: any = null;

  public structure: IStructure = <IStructure>{  };

  public structure_types: SelectGroupOption[] = Service.structure_types();
  public structure_rigs: SelectGroupOption[] = [];

  public validate(e: MouseEvent) {
    e.preventDefault();

    this.formRef.validate(
      (errors: Array<FormValidationError> | undefined) => {
        if (!errors) {
          console.log('ok')
        } else {
          console.log(errors)
        }

        this.add_structure();
      }
    )
  }

  public async add_structure() {
    let rigs = this.structure
      .rigs
      .map(x => x.value);
  }

  public selectStructure(value: any, option: SelectOption) {
    this.structure.rigs = [];
    this.structure_rigs = Service.rigs(value);
  }

  public selectStructureRig(index: number, value: any, option: SelectOption) {
    if (!value) {
      this.structure.rigs[index] = <any>null;
      return;
    }

    if (!this.structure.rigs) {
      this.structure.rigs = [];
    }

    this.structure.rigs[index] = {
      value:    value,
      value_t1: value,
      value_t2: <number>option.value_t2
    }
  }

  public selectTier(index: number, value: any) {
    if (value) {
      this.structure.rigs[index].value = this.structure.rigs[index].value_t2;
    } else {
      this.structure.rigs[index].value = this.structure.rigs[index].value_t1;
    }
  }

  public rules: FormRules = {
    name: [{
      required: true,
      message:  'The field is required'
    }],
    location: [{
      required: true,
      message:  'The field is required'
    }],
    structure_type: [{
      required: true,
      message:  'The field is required'
    }]
  }
}
</script>
