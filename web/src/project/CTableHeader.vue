<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ busy }">
      <n-space justify="end" style="margin: 10px">
        <n-button
          @click="() => reload()"
          :disabled="busy"
          :loading="busy"
        >
          <template #icon>
            <n-icon><refresh /></n-icon>
          </template>
        </n-button>

        <slot name="additional" />

        <slot name="primary">
          <n-button
            type="info"
            v-if="!noPrimary"
            @click="$router.push(primaryAction)"
            :disabled="busy"
          >
            {{ primaryContent }}
          </n-button>
        </slot>
      </n-space>
    </template>
  </w-project>
</template>

<script lang="ts">
import { NButton, NIcon, NSpace } from 'naive-ui';
import { Options, prop, Vue } from 'vue-class-component';
import { Refresh } from '@vicons/tabler';

import WProject from '@/project/WProject.vue';

class Props {
  reload = prop({
    type:     Function,
    required: true,
  });

  noPrimary = prop({
    type:    Boolean,
    default: () => false
  });

  // Example: { name: 'name_of_the_route', params: { additional: 'params' } }
  primaryAction = prop({
    type:     Object,
    required: !this.noPrimary
  });

  primaryContent = prop({
    type:     String,
    required: !this.noPrimary
  });
}

@Options({
  components: {
    NButton,
    NIcon,
    NSpace,

    Refresh,

    WProject,
  }
})
export default class TableHeader extends Vue.with(Props) {  }
</script>
