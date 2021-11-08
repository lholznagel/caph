import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { Events } from '@/event_bus';

export const events = new Events();

createApp(App)
  .use(router)
  .mount('#app');
