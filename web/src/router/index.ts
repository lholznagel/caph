import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/assets'
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
    path: '/projects/:pid',
    name: 'projects',
    component: () => import(
      /* webpackChunkName: "projects" */
      '@/project/VProject.vue'
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
