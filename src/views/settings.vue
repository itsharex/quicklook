<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { BaseDirectory } from '@tauri-apps/plugin-fs'
// import { emit } from '@tauri-apps/api/event'
import { readTextFile } from '@/utils'

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
    const target = [
        { name: 'markdown', code: 'md', data: data['preview.markdown.cheked'] },
        { name: '图片', code: 'image', data: data['preview.image.checked'] },
        { name: '视频', code: 'video', data: data['preview.video.checked'] },
        { name: '文档', code: 'doc', data: data['preview.doc.checked'] },
        { name: '代码', code: 'code', data: data['preview.code.checked'] },
        { name: '字体', code: 'font', data: data['preview.font.checked'] },
        { name: '压缩包', code: 'archive', data: data['preview.archive.checked'] },
        { name: '书籍', code: 'book', data: data['preview.book.checked'] },
    ]
    return target
}

interface Config {
    name: string
    code: string
    data: Array<string>
}

const config = ref<Array<Config>>()
onMounted(async () => {
    config.value = await getConfig()
    console.log(config.value)
})
</script>

<template>
    <div class="setting">
        <h1>This is an settings page</h1>
        <div class="setting-item support">
            <h2>支持的格式</h2>
            <div>
                <div class="support-item" v-for="type in config" :key="type.code">
                    <div class="support-item-header">
                        <span>{{ type.name }}（{{ type.code }}）</span>
                    </div>
                    <div class="support-item-body">
                        <el-space class="support-item-body-wrap">
                            <template v-for="item in type.data" :key="item">
                                <div class="support-item-body-item">
                                    {{ item }}
                                </div>
                            </template>
                        </el-space>
                    </div>
                </div>
            </div>
        </div>
        <div class="setting-item">
            <h2>更新</h2>
            <div>
                <el-button type="primary">检查更新</el-button>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.setting {
    padding: 16px;
    width: 100%;
}
.support {
    &-item {
        padding: 0 12px;
        & + & {
            margin-bottom: 24px;
        }

        &-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid #ebeef5;
            padding: 12px 0;
        }

        &-body {
            padding: 12px 16px;
            .support-item-body-wrap {
                display: flex;
                flex-wrap: wrap;
                .support-item-body-item {
                    flex: 0 0 80px;
                }
            }
        }
    }
}
</style>
