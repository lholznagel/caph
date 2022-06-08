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
          @click="redirect_login"
          v-if="!isLoggedIn()"
        >Login with Eve</n-button>
        <div v-if="isLoggedIn()" class="nav-header-character">
          <span class="nav-header-character-text">{{ whoami.character }}</span>

          <n-avatar size="medium" :src="whoami.character_icon" />
        </div>
      </n-layout-header>

      <n-layout position="absolute" style="top: 64px;" has-sider>
        <n-layout-sider
          bordered
          :native-scrollbar="false"
        >
          <n-menu
            :value="current_route"
            :options="options"
            :expanded-keys="['projects_projects']"
            :default-expanded-keys="['projects_projects']"
          />
        </n-layout-sider>

        <n-layout
          content-style="padding-left: 24px; padding-right: 24px;"
          :native-scrollbar="false"
        >
          <n-message-provider>
            <router-view
              style="margin-bottom: 10px"
              :key="$route.fullPath"
            />
          </n-message-provider>
        </n-layout>
      </n-layout>
    </n-layout>
  </n-config-provider>
</template>

<script lang="ts">
import {
  darkTheme, GlobalThemeOverrides, NAvatar, NButton, NConfigProvider, NGlobalStyle,
  NLayout, NLayoutHeader, NLayoutSider, NMenu, NMessageProvider, NResult
} from 'naive-ui';
import { Options, Vue } from 'vue-class-component';
import { h } from 'vue';
import { RouterLink } from 'vue-router';
import { events } from '@/main';
import { ROUTE_CHANGE, PROJECT_ROUTE } from '@/event_bus';

import { CharacterService, ICharacter } from '@/services/character';

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
    },
    Input: {
      borderRadius: '0'
    }
  };

  public whoami: ICharacter = <ICharacter>{  };
  public current_route: string = '';

  public options: any = [];

  public async created() {
    events.$on(ROUTE_CHANGE, (e: string) => this.current_route = e);

    events.$on(PROJECT_ROUTE, (e: string) => {
      this.options = [{
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

      if (e) {
        let pid = <string>this.$route.params.pid;
        this.options[0].children = [
          this.project_link('projects_overview', 'Overview', pid),
          this.project_link('projects_budget', 'Budget', pid),
          this.project_link('projects_blueprint', 'Blueprints', pid),
          this.project_link('projects_storage', 'Storage', pid),
          <any>{
            type:  'divider',
            key:   'divider_raw_materials'
          },
          <any>{
            type:  'group',
            key:   'raw_materials',
            label: 'Raw Materials',
            children: [
              this.project_link('projects_market', 'Market', pid),
              this.project_link('projects_raw_material', 'Materials', pid),
            ]
          },
          // Typescript does not like multiple types
          <any>{
            type: 'divider',
            key:  'divider_buildsteps'
          },
          this.project_link('projects_buildstep', 'Buildsteps', pid),
          this.project_link('projects_setting', 'Settings', pid),
        ];
        this.current_route = e;
      } else {
        this.current_route = 'projects_projects';
        this.options[0].children = undefined;
      }
    });

    await CharacterService
      .whoami()
      .then(x => {
        this.whoami = x;
        (<any>window).whoami = x;
      })
      .catch(_ => {});

    if (!this.isLoggedIn() && this.options.length === 0) {
      this.options = [];
    } else if(this.options.length === 0) {
      this.options = [{
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
    }
  }

  public redirect_login() {
    // TODO: Move to character seervice
    window.location.href = `/api/v1/auth/login`;
  }

  // FIXME:
  public isLoggedIn() {
    // TODO: not very secure
    return !!this.whoami.character;
  }

  public name() {
    if (window.location.origin === 'https://dev.caph.xyz') {
      return 'Caph DEV';
    } else {
      return 'Caph';
    }
  }

  private app_link(to: string, name: string) {
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

  private project_link(to: string, name: string, pid: string) {
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
