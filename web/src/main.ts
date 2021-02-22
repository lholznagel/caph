import Vue from 'vue';
import App from './App.vue';
import router from './router';
import vuetify from './plugins/vuetify';
import ECharts from 'vue-echarts';
import VueCompositionAPI from '@vue/composition-api';

import 'echarts';

export const HOME_SYSTEM: number = 30002755;

Vue.config.productionTip = false;

Vue.use(VueCompositionAPI);

Vue.component('v-chart', ECharts);

Vue.component('c-format-number', () => import('@/components/FormatNumber.vue'));
Vue.component('c-name-by-id', () => import('@/components/NameById.vue'));
Vue.component('c-route', () => import('@/components/Route.vue'));

Vue.component('c-item-info', () => import('@/components/item/Info.vue'));
Vue.component('c-item-reprocessing', () => import('@/components/item/Reprocessing.vue'));

Vue.component('c-market-orders', () => import('@/components/market/Orders.vue'));
Vue.component('c-market-stats', () => import('@/components/market/Stats.vue'));
Vue.component('c-market-charts', () => import('@/components/market/Chart.vue'));

new Vue({
  router,
  vuetify,
  render: h => h(App)
}).$mount('#app');
