import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/projects'
  },
  {
    path: '/assets',
    name: 'assets',
    component: () => import(
      /* webpackChunkName: "assets" */
      '@/views/Assets.vue'
    )
  },
  {
    path: '/industry/jobs',
    name: 'industry_jobs',
    component: () => import(
      /* webpackChunkName: "industry_jobs" */
      '@/views/Jobs.vue'
    )
  },
  {
    path: '/projects',
    name: 'projects_projects',
    component: () => import(
      /* webpackChunkName: "projects_projects" */
      '@/project/VProjects.vue'
    )
  },
  {
    path: '/projects/new',
    name: 'projects_new',
    component: () => import(
      /* webpackChunkName: "projects_new" */
      '@/project/VNewProject.vue'
    )
  },
  {
    path: '/projects/:pid',
    name: 'projects_overview',
    component: () => import(
      /* webpackChunkName: "projects_overview" */
      '@/project/VOverview.vue'
    )
  },
  {
    path: '/projects/:pid/market',
    name: 'projects_market',
    component: () => import(
      /* webpackChunkName: "projects_market" */
      '@/project/VMarket.vue'
    )
  },
  {
    path: '/projects/:pid/budget',
    name: 'projects_budget',
    component: () => import(
      /* webpackChunkName: "projects_budget" */
      '@/project/VBudget.vue'
    )
  },
  {
    path: '/projects/:pid/budget/add',
    name: 'projects_add_budget',
    component: () => import(
      /* webpackChunkName: "projects_add_budget" */
      '@/project/VAddBudget.vue'
    )
  },
  {
    path: '/projects/:pid/blueprint',
    name: 'projects_blueprint',
    component: () => import(
      /* webpackChunkName: "projects_blueprint" */
      '@/project/VBlueprint.vue'
    )
  },
  {
    path: '/projects/:pid/material',
    name: 'projects_raw_material',
    component: () => import(
      /* webpackChunkName: "projects_raw_material" */
      '@/project/VMaterial.vue'
    )
  },
  {
    path: '/projects/:pid/buildsteps',
    name: 'projects_buildstep',
    component: () => import(
      /* webpackChunkName: "projects_buildstep" */
      '@/project/VBuildstep.vue'
    )
  },
  {
    path: '/projects/:pid/settings',
    name: 'projects_setting',
    component: () => import(
      /* webpackChunkName: "projects_setting" */
      '@/project/VSetting.vue'
    )
  },
  {
    path: '/settings/characters',
    name: 'characters',
    component: () => import(
      /* webpackChunkName: "settings_characters" */
      '@/views/SettingCharacters.vue'
    )
  },
  {
    path: '/settings/stations',
    name: 'stations',
    component: () => import(
      /* webpackChunkName: "settings_stations" */
      '@/views/SettingStations.vue'
    )
  },
  {
    path: '/settings/corporation/blueprints',
    name: 'corporation_blueprints',
    component: () => import(
      /* webpackChunkName: "settings_corporation_blueprints" */
      '@/views/SettingCorporationBlueprints.vue'
    )
  }, {
    path: '/admin/features',
    name: 'admin_features',
    component: () => import(
      /* webpackChunkName: "admin_features" */
      '@/views/AdminFeatures.vue'
    )
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
});

router.beforeEach((to, from, next) => {
  // TODO: implement check if the user can see the page
  next();
});

export default router;
