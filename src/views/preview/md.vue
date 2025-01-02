<script setup lang="ts">
import { ref, onMounted } from 'vue'
import LayoutPreview from '@/components/layout-preview.vue'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import { readTextFile } from '@/utils'
import { createMd } from '@/utils/markdown/index'

import MdViewer from '@/components/md-viewer/index.vue'
import type MarkdownIt from 'markdown-it'

const route = useRoute()

let md: MarkdownIt | null = null

defineOptions({
    name: 'MdSupport',
})
const file = ref<FileInfo>()
const content = ref<string>()
const loading = ref<boolean>(true)

onMounted(async () => {
    if (md === null) {
        md = await createMd()
    }
    loading.value = true
    file.value = route?.query as unknown as FileInfo
    const path = file.value.path as string
    const txt = await readTextFile(path)

    content.value = md.render(txt)
    loading.value = false
})
</script>

<template>
    <LayoutPreview :file="file" :loading="loading">
        <div class="md-support">
            <div class="md-support-inner" id="markdown-body">
                <MdViewer :key="file?.path" :content="content" />
            </div>
        </div>
    </LayoutPreview>
</template>

<style scoped lang="scss">
.md-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    &-inner {
        width: 100%;
        height: 100%;
        overflow: hidden auto;
        padding: 12px 24px;
        font-size: 14px;
    }
}
</style>
