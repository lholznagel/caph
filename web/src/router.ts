import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/projects'
  },
  {
    path: '/projects',
    name: 'projects_projects',
    component: () => import(
      /* webpackChunkName: "projects_projects" */
      '@/project/Projects.vue'
    )
  },
  {
    path: '/projects/new',
    name: 'projects_new',
    component: () => import(
      /* webpackChunkName: "projects_new" */
      '@/project/NewProject.vue'
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
    name: 'projects_budget_add',
    component: () => import(
      /* webpackChunkName: "projects_budget_add" */
      '@/project/VBudgetAdd.vue'
    )
  },
  {
    path: '/projects/:pid/budget/edit/:bid',
    name: 'projects_budget_edit',
    component: () => import(
      /* webpackChunkName: "projects_budget_edit" */
      '@/project/VBudgetEdit.vue'
    )
  },
  {
    path: '/projects/:pid/blueprint',
    name: 'projects_blueprint',
    component: () => import(
      /* webpackChunkName: "projects_blueprint" */
      '@/project/Blueprint.vue'
    )
  },
  {
    path: '/projects/:pid/material/raw',
    name: 'projects_raw_material',
    component: () => import(
      /* webpackChunkName: "projects_material_raw" */
      '@/project/VRawMaterial.vue'
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
    path: '/projects/:pid/storage',
    name: 'projects_storage',
    component: () => import(
      /* webpackChunkName: "project_storage" */
      '@/project/VStorage.vue'
    )
  },
  {
    path: '/projects/:pid/storage/add/material',
    name: 'projects_storage_add_material',
    component: () => import(
      /* webpackChunkName: "project_storage_add_material" */
      '@/project/VStorageAddMaterial.vue'
    )
  },
  {
    path: '/projects/:pid/settings/general',
    name: 'projects_settings_general',
    component: () => import(
      /* webpackChunkName: "projects_settings_general" */
      '@/project/SettingsGeneral.vue'
    )
  },
  {
    path: '/settings/characters',
    name: 'settings_characters',
    component: () => import(
      /* webpackChunkName: "settings_characters" */
      '@/settings/Characters.vue'
    )
  },
  {
    path: '/settings/structures',
    name: 'settings_structures',
    component: () => import(
      /* webpackChunkName: "settings_structures" */
      '@/structure/Structures.vue'
    )
  },
  {
    path: '/settings/structures/new',
    name: 'settings_structures_new',
    component: () => import(
      /* webpackChunkName: "settings_structures_new" */
      '@/structure/NewStructure.vue'
    )
  },
  {
    path: '/character/blueprints',
    name: 'character_blueprints',
    component: () => import(
      /* webpackChunkName: "character_blueprints" */
      '@/views/CharacterBlueprints.vue'
    )
  },
  {
    path: '/industry_jobs',
    name: 'industry_jobs',
    component: () => import(
      /* webpackChunkName: "industry_jobs" */
      '@/settings/IndustryJobs.vue'
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
