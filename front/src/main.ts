import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { plugin, defaultConfig } from '@formkit/vue'
import config from './formkit.config'
createApp(App).use(plugin, defaultConfig(config)).use(router).mount('#app') 
