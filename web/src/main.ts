import App from './App.vue';
import ECharts from 'vue-echarts';
import router from './router';
import Vue from 'vue';
import VueCompositionAPI from '@vue/composition-api';
import vuetify from './plugins/vuetify';

import 'echarts';

Vue.config.productionTip = false;

Vue.use(VueCompositionAPI);

Vue.component('v-chart', ECharts);

Vue.component('c-format-number', () => import('@/components/FormatNumber.vue'));
Vue.component('c-name-by-id', () => import('@/components/NameById.vue'));

Vue.component('c-blueprint-info', () => import('@/components/blueprint/Info.vue'));
Vue.component('c-blueprint-graph', () => import('@/components/blueprint/Graph.vue'));
Vue.component('c-blueprint-item', () => import('@/components/blueprint/Item.vue'));

Vue.component('c-item-info', () => import('@/components/item/Info.vue'));
Vue.component('c-item-reprocessing', () => import('@/components/item/Reprocessing.vue'));
Vue.component('c-item-icon', () => import('@/components/item/Icon.vue'));

Vue.component('c-market-chart-historic', () => import('@/components/market/ChartHistoric.vue'));
Vue.component('c-market-orders', () => import('@/components/market/Orders.vue'));
Vue.component('c-market-stats', () => import('@/components/market/Stats.vue'));

new Vue({
  router,
  vuetify,
  render: h => h(App)
}).$mount('#app');
