import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import { error, debug } from '@tauri-apps/plugin-log'

import { NIcon, create } from 'naive-ui'
const naive = create({
    components: [NIcon],
})

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/es/locale/lang/zh-cn'

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(naive)
app.use(ElementPlus, {
    locale: zhCn,
})

app.config.errorHandler = (err, vm, code) => {
    error(`[Vue Error]: Instance-${vm}；Error- ${err}Code- ${code}`)
}
app.config.warnHandler = (msg, vm, trace) => {
    debug(`[Vue Warn]: Message-${msg}；Instance-${vm}；Trace- ${trace}`)
}

app.mount('#app')
