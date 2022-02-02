<template>
  <n-modal
    v-model:show="show"
    :on-update:show="close"
  >
    <n-card
      title="Invite member"
      style="width: 600px"
      :bordered="false"
    >
      <n-input v-model:value="link" disabled />

      <template #action>
        <n-space justify="end">
          <n-button
            @click="close"
            quaternary
          >
            Close
          </n-button>
          <n-button
            @click="copy"
            type="info"
          >
            Copy link
          </n-button>
        </n-space>
      </template>
    </n-card>
  </n-modal>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NCard, NInput, NModal, NSpace } from 'naive-ui';

class Props {
  pid = prop({
    type:     String,
    required: true
  });

  show = prop({
    type:     Boolean,
    required: true,
  });
}

@Options({
  components: {
    NButton,
    NCard,
    NInput,
    NModal,
    NSpace,
  }
})
export default class ProjectInvite extends Vue.with(Props) {
  public link = `${window.location.origin}/projects/${this.pid}/invite`;

  public close() {
    this.$emit('update:show', false);
  }

  public copy() {
    navigator.clipboard.writeText(this.link);
  }
}
</script>
