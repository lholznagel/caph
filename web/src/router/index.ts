import Vue from 'vue';
import VueRouter, { RouteConfig } from 'vue-router';

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: '/',
    redirect: 'Market'
  },
  {
    path: '/market',
    name: 'Market',
    component: () => import(/* webpackChunkName: "market" */ '../views/Market.vue')
  },
  {
    path: '/market/:id',
    name: 'MarketInfo',
    component: () => import(/* webpackChunkName: "market_info" */ '../views/MarketInfo.vue')
  },
  {
    path: '/blueprint/:id',
    name: 'BlueprintInfo',
    component: () => import( /* webpackChunkName: "blueprint_info" */ '../views/BlueprintInfo.vue')
  },
  {
    path: '/my/assets',
    name: 'CharacterAssets',
    component: () => import(/* webpackChunkName: "character_assets" */ '../views/CharacterAssets.vue')
  },
  {
    path: '/my/blueprints',
    name: 'CharacterBlueprints',
    component: () => import(/* webpackChunkName: "character_blueprints" */ '../views/CharacterBlueprints.vue')
  }
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
});

export default router;
