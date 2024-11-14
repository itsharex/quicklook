<script setup lang="ts">
import { ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { getWindow } from '@/utils/index'
import Header from './header.vue'
import Footer from './footer.vue'
import { useRouter } from 'vue-router'

const router = useRouter()

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
        const localePath = convertFileSrc(file.value?.path)
        console.log(localePath)

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
    <div class="preview">
        <Header class="preview-header" />
        <div class="preview-body">
            <RouterView v-slot="{ Component }">
                <template v-if="Component">
                    <Transition mode="out-in">
                        <Suspense>
                            <!-- 主要内容 -->
                            <component :is="Component"></component>

                            <!-- 加载中状态 -->
                            <template #fallback> 正在加载... </template>
                        </Suspense>
                    </Transition>
                </template>
            </RouterView>
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
