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
    path: '/industry/projects',
    name: 'industry_projects',
    component: () => import(
      /* webpackChunkName: "industry_projects" */
      '@/views/Projects.vue'
    )
  },
  {
    path: '/industry/projects/new',
    name: 'project_new',
    component: () => import(
      /* webpackChunkName: "industry_project_settings" */
      '@/views/ProjectSettings.vue'
    )
  },
  {
    path: '/industry/projects/:pid',
    name: 'project',
    component: () => import(
      /* webpackChunkName: "industry_project" */
      '@/views/Project.vue'
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
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router;
