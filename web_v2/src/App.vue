<template>
  <n-config-provider :theme="dark">
    <n-global-style />

    <div>
      <n-layout position="absolute">
        <n-layout-header class="header-text" bordered>
          Caph

          <n-button text class="nav-end">Login with Eve</n-button>
        </n-layout-header>
        <n-layout position="absolute" style="top: 64px;" has-sider>
          <n-layout-sider
            :native-scrollbar="false"
            bordered
          >
            <n-menu @update:value="handleUpdateValue" :options="options" />
          </n-layout-sider>
          <n-layout content-style="padding: 24px;" :native-scrollbar="false">
            <router-view />
          </n-layout>
        </n-layout>
      </n-layout>
    </div>
  </n-config-provider>
</template>

<script lang="ts">
import {
  darkTheme, NButton, NConfigProvider, NGlobalStyle, NLayout,
  NLayoutHeader, NLayoutSider, NMenu, NText
} from 'naive-ui';
import { Options, Vue } from 'vue-class-component';

@Options({
  components: {
    NButton,
    NConfigProvider,
    NGlobalStyle,
    NLayout,
    NLayoutHeader,
    NLayoutSider,
    NMenu,
    NText,
  }
})
export default class App extends Vue {
  public dark = darkTheme;
  public menuValue = '';

  public options = [{
    label: 'Home',
    key: 'home',
  }, {
    label: 'About',
    key: 'about'
  }];

  public handleUpdateValue(key: string, _: string) {
    this.$router.push({ name: key });
  }
}
</script>

<style scoped>
.header-text {
  cursor: pointer;

  font-size: 28px;
  height: 64px;
  padding: 24px;

  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
