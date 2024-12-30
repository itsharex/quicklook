<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { BaseDirectory } from '@tauri-apps/plugin-fs'
// import { emit } from '@tauri-apps/api/event'
import { readTextFile } from '@/utils'
import { app } from '@tauri-apps/api'

// // const updateConfig = async (val: string) => {
// //     const blob = new Blob([val], { type: 'application/json' })
// //     const arrayBuffer = new Uint8Array(await blob.arrayBuffer())

// //     writeFile('config.json', arrayBuffer, { baseDir: BaseDirectory.Resource })
// //         .then(() => {
// //             console.log('写入成功')
// //             emit('config_updated')
// //         })
// //         .catch(e => {
// //             console.error(e)
// //         })
// // }

const getConfig = async () => {
    const config = await readTextFile('config.json', { baseDir: BaseDirectory.Resource })
    const data = JSON.parse(config)
    console.log(data)
    const target = [
        { name: 'Markdown', code: 'Md', data: data['preview.markdown.checked'] },
        { name: '图片', code: 'Image', data: data['preview.image.checked'] },
        { name: '视频', code: 'Video', data: data['preview.video.checked'] },
        { name: '文档', code: 'Doc', data: data['preview.doc.checked'] },
        { name: '代码', code: 'Code', data: data['preview.code.checked'] },
        { name: '字体', code: 'Font', data: data['preview.font.checked'] },
        { name: '压缩包', code: 'Archive', data: data['preview.archive.checked'] },
        { name: '书籍', code: 'Book', data: data['preview.book.checked'] },
    ]
    return target
}

interface Config {
    name: string
    code: string
    data: Array<string>
}

const config = ref<Array<Config>>()
const version = ref<string>('')
onMounted(async () => {
    config.value = await getConfig()
    console.log(config.value)
    version.value = await app.getVersion()
})
</script>

<template>
    <div class="setting">
        <div class="setting-item support">
            <p class="setting-item-title">支持的格式</p>
            <div>
                <div class="support-item" v-for="type in config" :key="type.code">
                    <div class="support-item-header">
                        <span>{{ type.code }}：</span>
                    </div>
                    <div class="support-item-body">
                        {{ type.data.join('、') }}
                    </div>
                </div>
            </div>
        </div>
        <div class="setting-item">
            <p class="setting-item-title">版本</p>
            <div>
                <span>app 版本：{{ version }}</span>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.setting {
    padding: 16px;
    width: 100%;
    &-item {
        &-title {
            font-size: 16px;
            font-weight: 600;
            margin-bottom: 8px;
        }
    }
}
.support {
    &-item {
        padding: 4px 6px;
        display: flex;
        align-items: flex-start;

        &-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            flex: 0 0 70px;
            min-width: 70px;
            justify-content: flex-end;
        }
    }
}
</style>
