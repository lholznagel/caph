import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/settings'
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
    path: '/blueprints',
    name: 'blueprint_overview',
    component: () => import(
      /* webpackChunkName: "blueprintOverview" */
      '@/views/BlueprintOverview.vue'
    )
  },
  {
    path: '/blueprint/:bid/:iid?',
    name: 'blueprint',
    component: () => import(
      /* webpackChunkName: "blueprintInfo" */
      '../views/Blueprint.vue'
    )
  },
  {
    path: '/projects',
    name: 'projects',
    component: () => import(
      /* webpackChunkName: "project_overview" */
      '@/views/ProjectOverview.vue'
    )
  },
  {
    path: '/projects/:id',
    name: 'project',
    component: () => import(
      /* webpackChunkName: "project" */
      '@/views/Project.vue'
    )
  },
  {
    path: '/projects/new',
    name: 'project_new',
    component: () => import(
      /* webpackChunkName: "project_new" */
      '@/views/ProjectNew.vue'
    )
  },
  {
    path: '/industry',
    name: 'industry_jobs',
    component: () => import(
      /* webpackChunkName: "industry_jobs" */
      '@/views/Industry.vue'
    )
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import(
      /* webpackChunkName: "settings" */
      '@/views/Settings.vue'
    )
  },
  {
    path: '/admin/meta',
    name: 'meta',
    component: () => import(
      /* webpackChunkName: "meta" */
      '@/views/Meta.vue'
    )
  },
  {
    path: '/admin/network',
    name: 'network',
    component: () => import(
      /* webpackChunkName: "network" */
      '@/views/Network.vue'
    )
  },
  {
    path: '/admin/corp/blueprints',
    name: 'corp_blueprints',
    component: () => import(
      /* webpackChunkName: "corp_blueprints" */
      '@/views/CorpBlueprints.vue'
    )
  },
  {
    path: '/admin/alliance/fittings',
    name: 'alliance_fittings',
    component: () => import(
      /* webpackChunkName: "alliance_fittings" */
      '@/views/AllianceFittings.vue'
    )
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router;
