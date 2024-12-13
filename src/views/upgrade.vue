<script lang="ts" setup>
import { check, Update } from '@tauri-apps/plugin-updater'
import { onMounted, ref, computed } from 'vue'
import { info } from '@tauri-apps/plugin-log'

const checked = ref(false)
const checkUpgrade = async (): Promise<{ updaterInstance: Update | null; isUpgrade: boolean }> => {
    const result = await check()
    return {
        updaterInstance: result,
        isUpgrade: !!result,
    }
}
let updaterInstance: Update | null = null
const isUpgrade = ref(false)
onMounted(async () => {
    checked.value = false
    const tmp = await checkUpgrade()
    checked.value = true
    updaterInstance = tmp.updaterInstance
    isUpgrade.value = tmp.isUpgrade
})

const notes = computed(() => {
    return updaterInstance?.body
})

const calcPercentage = (current: number, total: number) => {
    const tmp = (current / total) * 100
    return parseFloat(tmp.toFixed(2))
}

const percentage = ref<number>(0)
const upgrade = async () => {
    let total: number = 0
    let current: number = 0
    if (isUpgrade.value) {
        await updaterInstance?.downloadAndInstall(payload => {
            const { event } = payload
            switch (event) {
                case 'Started':
                    total = payload.data?.contentLength || 0
                    info(`started downloading ${total} bytes`)
                    break
                case 'Progress':
                    current += payload.data?.chunkLength
                    percentage.value = calcPercentage(current, total)
                    info(`downloaded ${current} from ${total}`)
                    break
                case 'Finished':
                    info('download finished')
                    break
            }
        })
    }
}

const cancelUpgrade = () => {
    updaterInstance?.close()
}

const isDownloading = ref(false)
const handleUpgrade = () => {
    if (isDownloading.value) {
        cancelUpgrade()
    } else {
        handleUpgrade()
    }
}
</script>

<template>
    <div class="upgrade" v-loading="!checked">
        <template v-if="checked">
            <div v-if="isUpgrade" class="upgrade-yes">
                <p>更新内容：</p>
                <el-scrollbar max-height="400px" style="height: auto">
                    <pre v-html="notes"></pre>
                </el-scrollbar>
                <p>更新进度：</p>
                <el-progress :percentage="percentage" text-inside striped striped-flow :stroke-width="15" />
                <el-button @click="handleUpgrade">{{ isDownloading ? '取消升级' : '现在升级' }}</el-button>
            </div>
            <div v-else class="upgrade-none">
                <p>当前已经是最新版本了</p>
            </div>
        </template>
    </div>
</template>

<style scoped lang="scss">
.upgrade {
    width: 100vw;
    height: 100vh;
    &-none {
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        color: var(--el-color-success);
        box-sizing: border-box;
    }
    &-yes {
        width: 100%;
        height: 100%;
        padding: 12px;
        box-sizing: border-box;
        pre {
            white-space: pre-wrap;
            word-wrap: break-word;
            font-size: 13px;
            font-family: Inter, 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei',
                '微软雅黑', Arial, sans-serif;
        }
    }
}
</style>
