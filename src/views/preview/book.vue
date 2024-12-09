<!-- eslint-disable @typescript-eslint/no-explicit-any -->
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import LayoutPreview from '@/components/layout-preview.vue'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import { convertFileSrc } from '@tauri-apps/api/core'
import * as PDFJS from 'pdfjs-dist'
import { ArrowLeft, ArrowRight } from '@element-plus/icons-vue'

const route = useRoute()

defineOptions({
    name: 'BookSupport',
})

const fileInfo = ref<FileInfo>()

const loadDocument = (url: string) => {
    console.log('loadDocument', PDFJS)

    return new Promise((resolve, reject) => {
        PDFJS.getDocument(url)
            .promise.then((pdf: any) => {
                console.log('pdf111', pdf)
                resolve(pdf)
            })
            .catch(e => {
                console.log('pdf222', e)
                reject(null)
            })
    })
}

const renderPage = (pdf: any, num: number) => {
    pdf.getPage(num).then((page: any) => {
        page.cleanup()
        const context = canvasRef.value?.getContext('2d')
        const viewport = page.getViewport({ scale: 1 })
        ;(canvasRef.value as HTMLCanvasElement).height = viewport.height
        ;(canvasRef.value as HTMLCanvasElement).width = viewport.width
        page.render({
            canvasContext: context,
            viewport,
        })
    })
}

const handlePrev = (pdf: any) => {
    if (pager.value <= 1) {
        return
    }
    pager.value--
    renderPage(pdf, pager.value)
}
const handleNext = (pdf: any) => {
    if (pager.value >= pdf.numPages) {
        return
    }
    pager.value++
    renderPage(pdf, pager.value)
}

const canvasRef = ref<HTMLCanvasElement>()
const pager = ref<number>(1)
let pdf: any = null
onMounted(async () => {
    console.log('onMounted', PDFJS.GlobalWorkerOptions)
    PDFJS.GlobalWorkerOptions.workerSrc = '/pdf.worker.mjs'
    pager.value = 1
    fileInfo.value = route?.query as unknown as FileInfo
    const path = convertFileSrc(fileInfo.value.path)
    console.log('path', path)
    pdf = await loadDocument(path)
    console.log('pdf', pdf)

    if (pdf) {
        renderPage(pdf, pager.value)
    } else {
        console.log('pdf is null')
    }
})
</script>

<template>
    <LayoutPreview :file="fileInfo">
        <div class="text-support">
            <div class="text-support-inner">
                <canvas ref="canvasRef" class="canvas"></canvas>
            </div>
            <div class="pager">
                <div class="pager-item" @click="handlePrev(pdf)">
                    <el-icon size="18px"><ArrowLeft /></el-icon>
                </div>
                <div class="pager-item" @click="handleNext(pdf)">
                    <el-icon size="18px"><ArrowRight /></el-icon>
                </div>
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
    }
    .canvas {
        display: block;
        margin: 0 auto;
    }
    .pager {
        position: fixed;
        top: 50%;
        left: 0;
        transform: translateY(-50%);
        pointer-events: none;
        display: none;
        justify-content: space-between;
        width: 100%;
        padding: 0 24px;
        &-item {
            pointer-events: auto;
            cursor: pointer;
            display: inline-block;
            justify-content: center;
            align-items: center;
            border-radius: 50%;
            overflow: hidden;
            width: 40px;
            height: 40px;
            &:hover :deep(.el-icon) {
                color: var(--el-color-primary);
            }
        }
    }
    &:hover {
        .pager {
            display: flex;
        }
    }
}
</style>
