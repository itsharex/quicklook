<script setup lang="ts">
import { ref, onMounted } from 'vue'
import LayoutPreview from '@/components/layout-preview.vue'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import { invoke } from '@tauri-apps/api/core'

import Excel from '@/components/document/excel.vue'

const route = useRoute()

defineOptions({
    name: 'DocumentSupport',
})

interface Sheet {
    name: string
    rows: string[][]
}

enum DocType {
    Excel = 'Excel',
    Docx = 'Docx',
    Pptx = 'Pptx',
}

interface Docs {
    Excel?: Array<Sheet>
    Docx?: string
}

const fileInfo = ref<FileInfo>()
const content = ref<Docs[keyof Docs]>()
const type = ref<DocType>()

onMounted(async () => {
    fileInfo.value = route?.query as unknown as FileInfo
    const val = fileInfo.value.path as string
    const docs: Docs = await invoke('document', { path: val, mode: fileInfo.value.extension })
    type.value = docs.Excel ? DocType.Excel : DocType.Docx
    console.log('docs', type.value)
    switch (type.value) {
        case DocType.Excel:
            content.value = docs.Excel as Array<Sheet>
            break
        case DocType.Docx:
            content.value = docs.Docx as string
            break
        case DocType.Pptx:
            content.value = '' as string
    }
})
</script>

<template>
    <LayoutPreview :file="fileInfo">
        <div class="text-support">
            <div class="text-support-inner">
                <Excel v-if="type === DocType.Excel" :data="content as Array<Sheet>" />
                <div v-else-if="type === DocType.Docx" v-html="content"></div>
                <div v-else>暂不支持</div>
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
