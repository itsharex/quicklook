<script setup lang="ts">
import { onMounted, ref } from 'vue'
import Player, { I18N } from 'xgplayer'
import 'xgplayer/dist/index.min.css'
import ZH from 'xgplayer/es/lang/zh-cn'
import { useRoute } from 'vue-router'
import type { FileInfo } from '@/utils/typescript'
import LayoutPreview from '@/components/layout-preview.vue'
import { convertFileSrc } from '@tauri-apps/api/core'

const route = useRoute()

// 启用中文
I18N.use(ZH)

defineOptions({
    name: 'VideoSupport',
})

const fileInfo = ref<FileInfo>()
let player: Player | null = null

onMounted(async () => {
    fileInfo.value = route.query as unknown as FileInfo
    const path = convertFileSrc(fileInfo.value.path)
    if (player !== null) {
        player.destroy()
        ;(document.querySelector('#videos') as HTMLElement).innerHTML = ''
    }
    player = new Player({
        id: 'videos',
        url: path,
        height: '100%',
        width: '100%',
    })
})
</script>

<template>
    <LayoutPreview :file="fileInfo">
        <div class="video-support">
            <div class="video-support-inner">
                <div id="videos"></div>
            </div>
        </div>
    </LayoutPreview>
</template>

<style scoped lang="scss">
.video-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    &-inner {
        width: 100%;
        height: 100%;
    }
}
</style>
