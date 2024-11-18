<script setup lang="ts">
import { ref, shallowRef } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { getWindow } from '@/utils/index'
import { useRouter } from 'vue-router'

import LayoutPreview from '@/components/layout-preview.vue'

const router = useRouter()

const path = ref<string>('')

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
        let route = {
            name: 'peviewNotSupport',
            query: {},
        }

        const fileType = file.value?.file_type
        switch (fileType) {
            case 'Image':
                route = {
                    name: 'peviewImage',
                    query: { src: localePath },
                }
                break
            case 'Video':
                route = {
                    name: 'peviewVideo',
                    query: { src: localePath },
                }
                break
            case 'Font':
                route = {
                    name: 'peviewFont',
                    query: { src: localePath },
                }
                break
            case 'Markdown':
                route = {
                    name: 'peviewMd',
                    query: { src: file.value?.path },
                }
                break
            case 'Code':
                route = {
                    name: 'peviewCode',
                    query: { src: file.value?.path },
                }
                break
            case 'Text':
                route = {
                    name: 'peviewText',
                    query: { src: file.value?.path },
                }
                break
            default:
                route = {
                    name: 'peviewNotSupport',
                    query: {},
                }
                break
        }

        router.push(route as object)
    })
}

init()
</script>

<template>
    <LayoutPreview class="preview"> </LayoutPreview>
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
