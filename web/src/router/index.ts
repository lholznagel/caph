import Vue from 'vue';
import VueRouter, { RouteConfig } from 'vue-router';
import Home from '../views/Home.vue';

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/items/my',
    name: 'MyItems',
    component: () => import(/* webpackChunkName: "my:items" */ '../views/MyItems.vue')
  },
  {
    path: '/market/:id/info',
    name: 'MarketInfo',
    component: () => import(/* webpackChunkName: "item_info" */ '../views/MarketInfo.vue')
  }
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
});

export default router;
