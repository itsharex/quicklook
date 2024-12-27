<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { writeFile, BaseDirectory } from '@tauri-apps/plugin-fs'
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
    try {
        JSON.parse(config)
    } catch (e) {
        console.error(e)
    }
    const data = JSON.parse(config)
    console.log(typeof config, data)
    return {
        markdown: data['preview.markdown'],
        markdown_checked: data['preview.markdown.checked'],
        image: data['preview.image'],
        image_checked: data['preview.image.checked'],
        video: data['preview.video'],
        video_checked: data['preview.video.checked'],
        doc: data['preview.doc'],
        doc_checked: data['preview.doc.checked'],
        code: data['preview.code'],
        code_checked: data['preview.code.checked'],
        font: data['preview.font'],
        font_checked: data['preview.font.checked'],
        archive: data['preview.archive'],
        archive_checked: data['preview.archive.checked'],
        book: data['preview.book'],
        book_checked: data['preview.book.checked'],
    }
}

interface Config {
    markdown: Array<string>
    markdown_checked: Array<string>
    image: Array<string>
    image_checked: Array<string>
    video: Array<string>
    video_checked: Array<string>
    doc: Array<string>
    doc_checked: Array<string>
    code: Array<string>
    code_checked: Array<string>
    font: Array<string>
    font_checked: Array<string>
    archive: Array<string>
    archive_checked: Array<string>
    book: Array<string>
    book_checked: Array<string>
}

const config = ref<Config>({
    markdown: [],
    markdown_checked: [],
    image: [],
    image_checked: [],
    video: [],
    video_checked: [],
    doc: [],
    doc_checked: [],
    code: [],
    code_checked: [],
    font: [],
    font_checked: [],
    archive: [],
    archive_checked: [],
    book: [],
    book_checked: [],
})
onMounted(async () => {
    config.value = await getConfig()
    console.log(config.value)
})

// const config.image_checked = ['png', 'jpg', 'jpeg', 'gif', 'svg', 'apng']
// const imageChecked = ref([])
</script>

<template>
    <div class="setting">
        <h1>This is an settings page</h1>
        <div class="setting-item support">
            <h2>支持的格式</h2>

            <div class="support-item">
                <div class="support-item-header">
                    <span>图像（Image）</span>
                </div>
                <div class="support-item-body">
                    <el-space class="support-item-body-wrap">
                        <template v-for="item in config.image_checked" :key="item">
                            <div class="support-item-body-item">
                                {{ item }}
                            </div>
                        </template>
                    </el-space>
                </div>
            </div>
            <div class="support-item">
                <div class="support-item-header">
                    <span>视频（Video）</span>
                </div>
                <div class="support-item-body">
                    <el-space class="support-item-body-wrap">
                        <template v-for="item in config.video_checked" :key="item">
                            <div class="support-item-body-item">
                                {{ item }}
                            </div>
                        </template>
                    </el-space>
                </div>
            </div>
            <div class="support-item">
                <div class="support-item-header">
                    <span>字体（Font）</span>
                </div>
                <div class="support-item-body">
                    <el-space class="support-item-body-wrap">
                        <template v-for="item in config.font_checked" :key="item">
                            <div class="support-item-body-item">
                                {{ item }}
                            </div>
                        </template>
                    </el-space>
                </div>
            </div>
            <div class="support-item">
                <div class="support-item-header">
                    <span>书籍（Book）</span>
                </div>
                <div class="support-item-body">
                    <el-space class="support-item-body-wrap">
                        <template v-for="item in config.book_checked" :key="item">
                            <div class="support-item-body-item">
                                {{ item }}
                            </div>
                        </template>
                    </el-space>
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
