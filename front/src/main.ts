import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { plugin, defaultConfig } from '@formkit/vue'
import config from './formkit.config'
import './style.css'
import '@formkit/themes/genesis'

createApp(App).use(plugin, defaultConfig(config)).use(router).mount('#app') 
