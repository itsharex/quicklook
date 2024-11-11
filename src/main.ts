import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

import { NIcon, create } from 'naive-ui'
const naive = create({
    components: [NIcon],
})

import ElementPlus from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(naive)
app.use(ElementPlus, {
    locale: zhCn,
})

app.mount('#app')
