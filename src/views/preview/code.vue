<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { codeToHtml } from 'shiki'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import LayoutPreview from '@/components/layout-preview.vue'

const route = useRoute()

defineOptions({
    name: 'CodeSupport',
})

const fileInfo = ref<FileInfo>()
const content = ref<string>()
const loading = ref<boolean>(false)

onMounted(async () => {
    loading.value = false
    fileInfo.value = route.query as unknown as FileInfo
    const code = await readTextFile(fileInfo.value.path)
    const html = await codeToHtml(code, {
        lang: 'javascript',
        themes: {
            light: 'github-light',
            dark: 'github-dark',
        },
    })
    content.value = html
    loading.value = true
})
</script>

<template>
    <LayoutPreview :file="fileInfo" :loading="loading">
        <div class="code-support">
            <div class="code-support-inner" v-html="content"></div>
        </div>
    </LayoutPreview>
</template>

<style scoped lang="scss">
.code-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    &-inner {
        width: 100%;
        height: 100%;
        overflow: auto;
        padding: 12px 24px;
        font-size: 13px;
        & :deep(pre code) {
            font-family: 'Courier New', Courier, monospace;
            line-height: 1.2em;
        }
    }
}
</style>
