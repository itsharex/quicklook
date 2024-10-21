<script setup lang="ts">
import { ref, shallowRef, type ComponentInstance } from 'vue'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { getWindow  } from "@/utils/index"
import Header from './components/header.vue'
import Footer from './components/footer.vue'
import NotSupport from './not-support.vue'
import ImageSupport from './image.vue'
import VideoSupport from './video.vue'

const path = ref<string>('')
const componentName = shallowRef<ComponentInstance<any>>(NotSupport)

interface File {
    path: string
    file_type: string
    extension: string
}

const init = async () => {
    const win = await getWindow('main')
    win?.listen('file-preview', async (e) => {

        const payload = e.payload as string
        const file: File = await invoke("preview_file", { path: payload })
        console.log("file path is ", file);
        const localePath = convertFileSrc(file.path);
        console.log(localePath)

        const fileType = file.file_type;
        switch (fileType) {
            case "Image":
                componentName.value = ImageSupport;
                break;
            case "Video":
                componentName.value = VideoSupport;
                break;
            default:
                componentName.value = NotSupport
                break;
        }
        path.value = localePath;
    })
}

init()

</script>

<template>
    <div class="preview">
        <Header class="preview-header" />
        <div class="preview-body">
            <component :is="componentName" :src="path"></component>
        </div>
        <Footer class="preview-footer" />
    </div>
</template>

<style scoped lang="scss">
.preview {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    position: relative;
    &-header{
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
    }
    &-footer{
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