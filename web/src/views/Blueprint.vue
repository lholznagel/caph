<template>
  <n-card>
    <template #header>
      {{ bp.name }}
    </template>

    <n-tabs type="line">
      <!--n-tab-pane name="Item info" :disabled="!bp.item_id"-->
      <n-tab-pane name="Item info">
        <!--blueprint-info :bp="bp" v-if="cbp.item_id" /-->
        <blueprint-info :bp="bp" :bp_product="bp_product" />
      </n-tab-pane>
      <!--n-tab-pane name="Copy" :disabled="!bp.copy">
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
      </n-tab-pane-->
    </n-tabs>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard, NTabs, NTabPane } from 'naive-ui';
import { AssetService, IBlueprint, IBlueprintMaterial } from '@/services/asset';

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
  public tid: any = undefined;
  public iid: any = undefined;

  public bp: {} | IBlueprint = {};
  public bp_product: IBlueprintMaterial[] = [];

  // Requires a bid -> BlueprintId
  // Optional a iid -> ItemId
  public async created() {
    this.tid = Number(this.$route.params.tid);
    this.iid = Number(this.$route.params.iid);

    console.log(this.iid)
    this.bp = await AssetService.blueprint(
      this.tid,
      this.iid === 0 ? null : this.iid
    );
    this.bp_product = await AssetService.blueprint_product(this.tid);
  }
}
</script>

