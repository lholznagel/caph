<template>
  <div>
    <loading
      description="Getting things ready"
      :busy=busy
    />

    <slot v-if="!busy"
      :busy="busy || project.busy"
      :project="project"
    >
    </slot>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { Service } from '@/project/service';
import { Project, ProjectId } from './project';

import Loading from '@/components/Loading.vue';

class Props {
  // Project id
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {
    Loading
  }
})
export default class ProjectWrapper extends Vue.with(Props) {
  public busy: boolean = false;

  public project: Project = <Project>{  };

  public async created() {
    this.busy = true;

    this.project = await Service.by_id(<ProjectId>this.pid);
    await this.project.init();

    this.busy = false;
  }
}
</script>
