<template>
  <n-modal
    preset="card"
    style="width: 600px;"
    v-model:show="show"
    :on-update:show="close"
    :on-after-leave="() => confirm_delete_text = ''"
    :title="'Delete ' + resource + '?'"
  >
    <n-alert type="warning" style="margin-bottom: 10px;">
      <slot></slot>
    </n-alert>

    <n-input v-model:value="confirm_delete_text" placeholder="delete" />

    <template #footer>
      <n-space justify="end">
        <n-button
          @click="close"
          quaternary
        >
          Cancel
        </n-button>

        <n-button
          @click="confirm_delete"
          :disabled="confirm_delete_text.toLowerCase() !== 'delete'"
        >
          Delete
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NAlert, NButton, NInput, NModal, NSpace } from 'naive-ui';

class Props {
  confirm = prop({
    type:     Function,
    required: true,
  });

  resource = prop({
    type:     String,
    required: true,
  });

  show = prop({
    type:     Boolean,
    required: true,
  });
}

@Options({
  components: {
    NAlert,
    NButton,
    NInput,
    NModal,
    NSpace,
  }
})
export default class ConfirmDialog extends Vue.with(Props) {
  public confirm_delete_text: string = '';

  public close() {
    this.$emit('update:show', false);
  }

  public confirm_delete() {
    this.confirm();
    this.close();
  }
}
</script>
