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
    path: '/items/:id/info',
    name: 'ItemInfo',
    component: () => import(/* webpackChunkName: "item_info" */ '../views/ItemInfo.vue')
  }
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
});

export default router;
