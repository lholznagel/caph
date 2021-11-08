<template>
  <n-text v-if="name">{{ name }}</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';
import { ProjectService } from '@/project/service';

class Props {
  busy = prop({
    type:     Boolean,
    required: true,
  });

  pid = prop({
    type:     String,
    required: true
  });
}

@Options({
  components: {
    NText,
  }
})
export default class ProjectName extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    this.$emit('update:busy', true);

    let project = await ProjectService.by_id(this.pid);
    this.name = project.name;

    this.$emit('update:busy', false);
  }
}
</script>
