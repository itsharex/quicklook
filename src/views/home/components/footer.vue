<script setup lang="ts">
import { watch, ref } from 'vue'
import { lstat, type FileInfo  } from '@tauri-apps/plugin-fs'

interface File {
    path: string
    file_type: string
    extension: string
}
interface Props {
    file?: File
}
const { file } = defineProps<Props>();

const fileInfo = ref<FileInfo>();

watch(() => file, async (val: File, oldVal: File) => {
    if(val.path !== oldVal.path || !oldVal) {
        fileInfo.value = await lstat(val.path)
        // console.log(fileInfo.value, file)
    }
})
</script>

<template>
    <div class="footer">
        <span class="footer-item">文件类型：{{ file?.file_type }}</span>
        <span class="footer-item">文件格式：{{ file?.extension }}</span>
        <span class="footer-item">文件大小：{{ fileInfo?.size }}</span>
    </div>
</template>

<style scoped lang="scss">
.footer {
    height: 20px;
    padding: 0 8px;
    background: #f5f5f5;
    display: flex;
    align-items: center;
    align-content: center;
    font-size: 12px;
    gap: 12px;
    &-item {
        zoom: 0.8;
    }
}
</style>