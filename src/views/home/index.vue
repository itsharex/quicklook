<script setup lang="ts">
import { ref, shallowRef } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { getWindow } from '@/utils/index'
import Header from './components/header.vue'
import Footer from './components/footer.vue'
import NotSupport from './not-support.vue'
import ImageSupport from './image.vue'
import VideoSupport from './video.vue'
import FontSupport from './font.vue'
import MdSupport from './md.vue'
import CodeSupport from './code.vue'
import TextSupport from './text.vue'

type Component =
    | typeof NotSupport
    | typeof ImageSupport
    | typeof VideoSupport
    | typeof FontSupport
    | typeof MdSupport
    | typeof CodeSupport
    | typeof TextSupport

const path = ref<string>('')
const componentName = shallowRef<Component>(NotSupport)

interface File {
    path: string
    file_type: string
    extension: string
}
const file = ref<File>()
const init = async () => {
    const win = await getWindow('main')
    win?.listen('file-preview', async e => {
        const payload = e.payload
        file.value = payload as File
        console.log('file path is ', file.value)
        const localePath = convertFileSrc(file.value?.path)
        console.log(localePath)

        const fileType = file.value?.file_type
        switch (fileType) {
            case 'Image':
                componentName.value = ImageSupport
                path.value = localePath
                break
            case 'Video':
                componentName.value = VideoSupport
                path.value = localePath
                break
            case 'Font':
                componentName.value = FontSupport
                path.value = localePath
                break
            case 'Markdown':
                componentName.value = MdSupport
                path.value = file.value?.path
                break
            case 'Code':
                componentName.value = CodeSupport
                path.value = file.value?.path
                break
            case 'Text':
                componentName.value = TextSupport
                path.value = file.value?.path
                break
            default:
                componentName.value = NotSupport
                path.value = localePath
                break
        }
    })
}

init()
</script>

<template>
    <div class="preview">
        <Header class="preview-header" />
        <div class="preview-body">
            <Suspense>
                <template #default>
                    <component :is="componentName" :src="path" :type="file?.extension"></component>
                </template>
                <template #fallback>
                    <el-loading> </el-loading>
                </template>
            </Suspense>
        </div>
        <Footer :file="file" class="preview-footer" />
    </div>
</template>

<style scoped lang="scss">
.preview {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    position: relative;
    &-header {
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
    }
    &-footer {
        position: absolute;
        left: 0;
        bottom: 0;
        width: 100%;
        font-size: 12px;
    }
    &-body {
        padding: 28px 0 20px;
        display: flex;
        justify-content: center;
        align-items: center;
        align-content: center;
        height: 100%;
    }
}
</style>
