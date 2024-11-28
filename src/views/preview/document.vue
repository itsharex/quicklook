<script setup lang="ts">
import { ref, onMounted } from 'vue'
import LayoutPreview from '@/components/layout-preview.vue'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import { invoke } from '@tauri-apps/api/core'

import Excel from '@/components/document/excel.vue'

const route = useRoute()

defineOptions({
    name: 'ArchiveSupport',
})

interface Sheet {
    name: string
    rows: string[][]
}

const fileInfo = ref<FileInfo>()
const content = ref<Array<Sheet>>()

onMounted(async () => {
    fileInfo.value = route?.query as unknown as FileInfo
    const val = fileInfo.value.path as string
    const txt: Array<Sheet> = await invoke('document', { path: val, mode: fileInfo.value.extension })

    console.log('path', txt)
    content.value = txt
})
</script>

<template>
    <LayoutPreview :file="fileInfo">
        <div class="text-support">
            <div class="text-support-inner">
                <Excel :data="content" />
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
        overflow: hidden;
        padding: 0;
        font-size: 1.4rem;
        font-family: 'Microsoft YaHei', 'PingFang SC', 'Helvetica Neue', 'Helvetica', 'Arial', sans-serif;
    }
}
</style>
