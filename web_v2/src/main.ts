import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import 'vfonts/Lato.css';

createApp(App)
  .use(router)
  .mount('#app')
