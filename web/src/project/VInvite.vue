<template>
  <div>
    <n-page-header>
      <template #title>
        <h1>
          Invite
        </h1>
      </template>
    </n-page-header>

    <n-card>
      <n-skeleton v-if="busy"></n-skeleton>

      <div v-if="!busy">
        You where invited to join the project '{{ project.name }}', created by {{ owner }}.<br>
        Please either accept or ignore the invite.
        You can still join at a later time.
      </div>

      <template #action>
        <n-space justify="end">
          <n-button
            @click="$router.push({ name: 'projects_overview' })"
          >
            Ignore
          </n-button>

          <n-button
            @click="add_member"
            type="Info"
          >
            Accept
          </n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script lang="ts">
import axios from 'axios';
import { IProject, ProjectService2 } from './service';
import { CharacterService } from '@/services/character';
import { Options, Vue } from 'vue-class-component';
import { NButton, NCard, NPageHeader, NSkeleton, NSpace } from 'naive-ui';

@Options({
  components: {
    NButton,
    NCard,
    NPageHeader,
    NSkeleton,
    NSpace,
  }
})
export default class ProjectInvite extends Vue {
  public busy: boolean = false;
  public owner: string = '';
  public project: { name: string, owner: number}  = { name: '', owner: 0};

  public async created() {
    this.busy = true;
    this.project = (await axios.get(`/api/v1/projects/${this.$route.params.pid}/name`)).data;
    this.owner = await CharacterService.character_name(this.project.owner);
    this.busy = false;
  }

  public async add_member() {
    ProjectService2.add_member(<string>this.$route.params.pid);
    this.$router.push({
      name: 'projects_overview',
      params: {
        pid: <string>this.$route.params.pid
      }
    })
  }
}
</script>
