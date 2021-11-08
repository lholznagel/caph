<template>
  <n-select
    v-model:value="value"
    :options="options"
    :render-label="render_entry"
    :render-tag="render_selected"
    filterable
  />
</template>

<script lang="ts">
import { NSelect } from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { CharacterService } from '@/services/character';
import { h } from 'vue';

import Owner from '@/components/Owner.vue';

@Options({
  components: {
    NSelect,

    Owner
  }
})
export default class CharacterSelector extends Vue {
  public options: any[] = [];

  public value: number | null = null;

  public async created() {
    let character = (<any>window).whoami;
    this.options.push({
      label: character.character,
      value: character.character_id
    });

    (await CharacterService.alts())
      .forEach(x => {
          this.options.push({
            label: x.character,
            value: x.character_id
          });
      });
  }

  public render_selected({ option }: any) {
    return this.render_entry(option);
  }

  public render_entry(option: any) {
    return h(
      'div',
      {
        style: {
          display: 'flex',
          alignItems: 'center'
        }
      },
      [
        h(Owner, {
          id: option.value,
          withText: false,
          style: {
            marginTop:   '2px',
            marginRight: '10px'
          }
        }),
        option.label
      ]
    )
  }
}
</script>
