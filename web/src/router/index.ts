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
  }
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
});

export default router;
