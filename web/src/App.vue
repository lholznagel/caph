<template>
  <n-config-provider :theme="dark" :theme-overrides="theme_overrides">
    <n-global-style />

    <n-layout position="absolute">
      <n-layout-header class="header" bordered>
        <div class="nav-header-text">
          {{ name() }}
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
            :value="current_route"
            :options="options"
            :expanded-keys="['projects_projects']"
            :default-expanded-keys="['projects_projects']"
          />
        </n-layout-sider>

        <n-layout
          content-style="padding-left: 24px; padding-right: 24px"
          :native-scrollbar="false"
        >
          <n-message-provider>
            <router-view
              :key="$route.fullPath"
              v-if="isLoggedIn()"
            />
          </n-message-provider>

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
  darkTheme, GlobalThemeOverrides, NAvatar, NButton, NConfigProvider, NGlobalStyle,
  NLayout, NLayoutHeader, NLayoutSider, NMenu, NMessageProvider, NResult
} from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';
import { RouterLink } from 'vue-router';
import { events } from '@/main';
import { ROUTE, PROJECT_ROUTE } from '@/event_bus';

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
    NMessageProvider,
    NResult,

    RouterLink
  }
})
export default class App extends Vue {
  public dark = darkTheme;

  public theme_overrides: GlobalThemeOverrides = {
    Button: {
      borderRadiusMedium: '0'
    },
    Card: {
      borderRadius: '0'
    },
    Menu: {
      borderRadius: '0'
    },
    Table: {
      borderRadius: '0'
    },
    Tag: {
      borderRadius: '0'
    }
  };

  public whoami: ICharacter = DEFAULT_CHARACTER;

  public current_route: string = '';
  public expand_keys: string[] = [];

  public options = [{
    label: () => h(
      RouterLink,
      {
        to: {
          name: 'projects_projects'
        }
      },
      { default: () => 'Projects' }
    ),
    key:   'projects_projects',
  }, {
    label: 'Settings',
    key:   'settings',
    type:  'group',
    children: [
      this.app_link('settings_characters', 'Characters'),
    ]
  }];

  public async created() {
    events.$on(PROJECT_ROUTE, (e: string) => {
      if (e) {
        let pid = <string>this.$route.params.pid;
        this.options[0].children = [
          this.project_link('projects_overview', 'Overview', pid),
          this.project_link('projects_market', 'Market', pid),
          this.project_link('projects_budget', 'Budget', pid),
          this.project_link('projects_blueprint', 'Blueprint', pid),
          this.project_link('projects_raw_material', 'Raw Materials', pid),
          this.project_link('projects_buildstep', 'Buildsteps', pid),
          this.project_link('projects_setting', 'Settings', pid),
        ];
        this.current_route = e;
      } else {
        this.current_route = 'projects_projects';
        this.options[0].children = undefined;
      }
    });

    const res = await (axios.get<ICharacter>('/api/auth/whoami'));
    if (res.status === 200) {
      this.whoami = res.data;
      let globalWindow: any = window;
      globalWindow.whoami = res.data;
    }

    events.$on(ROUTE, (e: string) => this.current_route = e);
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

  public project_link(to: string, name: string, pid: string) {
    return {
      label: () =>
        h(
          RouterLink,
          {
            to: {
              name: to,
              params: {
                pid
              }
            }
          },
          { default: () => name }
        ),
      key: to,
    };
  }

  public redirectLogin() {
    window.location.href = `/api/auth/login`;
  }

  public isLoggedIn() {
    // TODO: not very secure
    return this.whoami.character !== '';
  }

  public name() {
    if (window.location.origin === 'https://dev.caph.xyz') {
      return 'Caph DEV';
    } else {
      return 'Caph';
    }
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
