<template>
  <n-config-provider :theme="dark">
    <n-global-style />

    <n-layout position="absolute">
      <n-layout-header class="header" bordered>
        <div class="nav-header-text">
          Caph
        </div>

        <n-button
          text
          @click="redirectLogin"
          v-if="!isLoggedIn()"
        >Login with Eve</n-button>
        <div v-if="isLoggedIn()" class="nav-header-character">
          <span class="nav-header-character-text">{{ whoami.character }}</span>

          <n-avatar size="medium" :src="whoami.character_icon" />
        </div>
      </n-layout-header>

      <n-layout position="absolute" style="top: 64px;" :has-sider="isLoggedIn()">
        <n-layout-sider
          :native-scrollbar="false"
          bordered
          v-if="isLoggedIn()"
        >
          <n-menu
            @update:value="handleUpdateValue"
            :options="options"
            default-expand-all
          />
        </n-layout-sider>

        <n-layout content-style="padding: 24px;" :native-scrollbar="false">
          <router-view :key="$route.fullPath" v-if="isLoggedIn()" />

          <n-result
            status="403"
            title="403 Forbidden"
            description="Some of the doors are always close to you."
            v-if="!isLoggedIn()"
          >
            <template #footer>
              <n-button @click="redirectLogin">Login with eve</n-button>
            </template>
          </n-result>
        </n-layout>
      </n-layout>
    </n-layout>
  </n-config-provider>
</template>

<script lang="ts">
import axios from 'axios';
import {
  darkTheme, NAvatar, NButton, NConfigProvider, NGlobalStyle,
  NLayout, NLayoutHeader, NLayoutSider, NMenu, NResult
} from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';
import { RouterLink } from 'vue-router';

@Options({
  components: {
    NAvatar,
    NButton,
    NConfigProvider,
    NGlobalStyle,
    NLayout,
    NLayoutHeader,
    NLayoutSider,
    NMenu,
    NResult,

    RouterLink
  }
})
export default class App extends Vue {
  public dark = darkTheme;
  public menuValue = '';

  public whoami: ICharacter = DEFAULT_CHARACTER;

  public options = [
  {
    label: 'Assets',
    key:   'assets',
    type:  'group',
  }, {
    label: 'Industry',
    key:   'industry',
    type:  'group',
    children: [
      this.app_link('industry_projects', 'Projects'),
    ]
  }, {
    label: 'Settings',
    key:   'settings',
    type:  'group',
    children: [
      {
        label: 'Characters',
        key:   'characters'
      },
      {
        label: 'Stations',
        key:   'stations'
      }
    ]
  }];

  public async created() {
    const res = await (axios.get<ICharacter>('/api/auth/whoami'));
    if (res.status === 200) {
      this.whoami = res.data;
      let globalWindow: any = window;
      globalWindow.whoami = res.data;
    }

    this.options[0].children = <any>[
      {
        label: () =>
        h(
          RouterLink,
          {
            to: {
              name: 'assets',
            }
          },
          { default: () => 'All' }
        ),
        key: 'assets',
      },
    ];

    const views = (await axios.get<any>('/api/character/asset/views')).data;
    for (let view of views) {
      (this.options[0].children || []).push(<any>{
        label: () =>
          h(
            RouterLink,
            {
              to: {
                name: 'assets',
                query: view.query
              }
            },
            { default: () => view.name }
          ),
        key: 'assets_' + view.name,
      })
    }
  }

  public app_link(to: string, name: string) {
    return {
      label: () =>
        h(
          RouterLink,
          {
            to: {
              name: to,
            }
          },
          { default: () => name }
        ),
      key: to,
    };
  }

  public handleUpdateValue(key: string, _: string) {
    if (!key.startsWith('assets')) {
      this.$router.push({ name: key });
    }
  }

  public redirectLogin() {
    window.location.href = `/api/auth/login`;
  }

  public isLoggedIn() {
    // TODO: not very secure
    return this.whoami.character !== '';
  }
}

interface ICharacter {
  character:        string,
  character_id:     number,
  character_icon:   string,
  corporation:      string,
  corporation_icon: string,
  corporation_id:   number,
  alliance:         string,
  alliance_icon:    string,
  alliance_id:      number,
}
const DEFAULT_CHARACTER: ICharacter = {
  character:        '',
  character_id:     0,
  character_icon:   '',
  corporation:      '',
  corporation_icon: '',
  corporation_id:   0,
  alliance:         '',
  alliance_icon:    '',
  alliance_id:      0,
}
</script>

<style scoped>
.header {
  cursor: pointer;

  height: 64px;
  padding: 24px;

  display: flex;
  align-items: center;
  justify-content: space-between;
}

.nav-header-text {
  font-size: 28px;
}

.nav-header-character {
  display: flex;
  align-items: center;
}

.nav-header-character-text {
  margin-right: 10px;
  font-size: 16px;
}
</style>
