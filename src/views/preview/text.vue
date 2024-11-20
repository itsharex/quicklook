<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { readTextFile } from '@tauri-apps/plugin-fs'
// import { codeToHtml } from 'shiki'
import LayoutPreview from '@/components/layout-preview.vue'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'

const route = useRoute()

defineOptions({
    name: 'TextSupport',
})

const fileInfo = ref<FileInfo>()
const content = ref<string>()

onMounted(async () => {
    const val = route?.query?.path as string
    console.log('path', val)
    const txt = await readTextFile(val)

    content.value = txt
})
</script>

<template>
    <LayoutPreview :file="fileInfo">
        <div class="text-support">
            <div class="text-support-inner">
                <pre>{{ content }}</pre>
            </div>
        </div>
    </LayoutPreview>
</template>

<style scoped lang="scss">
.text-support {
    width: 100%;
    height: 100%;
    display: flex;
    &-inner {
        width: 100%;
        height: 100%;
        overflow: auto;
        padding: 12px 24px;
        font-size: 14px;
        font-family: 'Microsoft YaHei', 'PingFang SC', 'Helvetica Neue', 'Helvetica', 'Arial', sans-serif;
        & pre {
            white-space: pre-wrap;
            word-wrap: break-word;
            font-family: inherit;
            font-size: inherit;
        }
    }
}
</style>
