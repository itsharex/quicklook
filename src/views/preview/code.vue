<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { codeToHtml } from 'shiki'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import LayoutPreview from '@/components/layout-preview.vue'
import { readTextFile } from '@/utils'

const route = useRoute()

defineOptions({
    name: 'CodeSupport',
})

const fileInfo = ref<FileInfo>()
const content = ref<string>()
const loading = ref<boolean>(false)

onMounted(async () => {
    loading.value = true
    fileInfo.value = route.query as unknown as FileInfo
    const path = fileInfo.value.path as string

    const code = await readTextFile(path)
    const html = await codeToHtml(code, {
        lang: fileInfo.value.extension || 'plaintext',
        themes: {
            light: 'github-light',
            dark: 'github-dark',
        },
    })
    content.value = html
    loading.value = false
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
        padding: 12px 16px;
        font-size: 13px;
        & :deep(pre),
        & :deep(pre code) {
            font-family: 'Courier New', Courier, monospace;
            line-height: 1.2em;
        }
    }
}
</style>
