<template>
  <slot :busy="busy" :project="project">
  </slot>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { Project, ProjectId, ProjectService } from '@/project/service';

class Props {
  // Project id
  pid = prop({
    type:     String,
    required: true,
  });
}

@Options({
  components: {  }
})
export default class ProjectWrapper extends Vue.with(Props) {
  public busy: boolean = false;

  public project: Project = <Project>{  };

  public async created() {
    this.busy = true;

    this.project = await ProjectService.by_id(<ProjectId>this.$route.params.pid);
    await this.project.init();

    this.busy = false;
  }
}
</script>
