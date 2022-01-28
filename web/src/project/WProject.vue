<template>
  <div>
    <n-empty
      description="Getting things ready"
      size="large"
      style="margin-top: 10%"
      v-if="busy"
    >
      <template #icon>
        <n-spin size="large" />
      </template>
    </n-empty>

    <slot v-if="!busy" :busy="busy" :project="project">
    </slot>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NEmpty, NSpin } from 'naive-ui';
import { Project, ProjectId, ProjectService2 } from '@/project/service';

class Props {
  // Project id
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    NEmpty,
    NSpin
  }
})
export default class ProjectWrapper extends Vue.with(Props) {
  public busy: boolean = false;

  public project: Project = <Project>{  };

  public async created() {
    this.busy = true;

    this.project = await ProjectService2.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();

    this.busy = false;
  }
}
</script>
