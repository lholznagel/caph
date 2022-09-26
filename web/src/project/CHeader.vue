<template>
  <w-project :pid="$route.params.pid">
    <template v-slot="{ project }">
      <n-page-header>
        <template #title>
          <h1 style="margin-top: 10px">
            Project "{{ project.info.name || '' }}"
          </h1>
        </template>
      </n-page-header>
    </template>
  </w-project>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NPageHeader } from 'naive-ui';

import WProject from '@/project/wrapper/Project.vue';

@Options({
  components: {
    NPageHeader,

    WProject
  }
})
export default class ProjectHeader extends Vue {  }
</script>
