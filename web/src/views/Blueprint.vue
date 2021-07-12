<template>
  <n-card>
    <template #header>
      <name-by-id :id="Number($route.params.bid)" />
    </template>

    <n-tabs type="line">
      <n-tab-pane name="Item info" :disabled="!cbp.item_id">
        <blueprint-info :bp="bp" :cbp="cbp" v-if="cbp.item_id" />
      </n-tab-pane>
      <n-tab-pane name="Copy" :disabled="!bp.copy">
        <blueprint-action :action="bp.copy" v-if="bp.copy" />
      </n-tab-pane>
      <n-tab-pane name="Invention" :disabled="!bp.invention">
        <blueprint-action :action="bp.invention" />
      </n-tab-pane>
      <n-tab-pane name="Manufacture" :disabled="!bp.manufacture">
        <blueprint-action :action="bp.manufacture" />
      </n-tab-pane>
      <n-tab-pane name="Reaction" :disabled="!bp.reaction">
        <blueprint-action :action="bp.reaction" />
      </n-tab-pane>
      <n-tab-pane name="Research Material" :disabled="!bp.research_mat">
        <blueprint-action :action="bp.research_mat" />
      </n-tab-pane>
      <n-tab-pane name="Research Time" :disabled="!bp.research_time">
        <blueprint-action :action="bp.research_time" />
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NTabs, NTabPane } from 'naive-ui';
import { BlueprintService, IBlueprint } from '@/services/blueprint';
import { CharacterService, ICharacterBlueprint } from '@/services/character';

import BlueprintAction from '@/components/blueprint/Action.vue';
import BlueprintInfo from '@/components/blueprint/Info.vue';
import NameById from '@/components/NameById.vue';

@Options({
  components: {
    NCard,
    NTabs,
    NTabPane,

    BlueprintAction,
    BlueprintInfo,
    NameById,
  }
})
export default class Blueprint extends Vue {
  public bid: any = undefined;
  public iid: any = undefined;

  public bp:  {} | IBlueprint          = {};
  public cbp: {} | ICharacterBlueprint = {};

  // Requires a bid -> BlueprintId
  // Optional a iid -> ItemId
  public async created() {
    this.bid = Number(this.$route.params.bid);
    this.iid = Number(this.$route.params.iid);

    this.bp = await BlueprintService.blueprint(this.bid);
    if (this.iid) {
      this.cbp = (await CharacterService.blueprints())
        .find((x: ICharacterBlueprint) => x.item_id === Number(this.iid)) || {};
    }
  }
}
</script>

