<script setup lang="ts">
import { onMounted } from "vue"
import Player, { I18N } from 'xgplayer'
import 'xgplayer/dist/index.min.css'
import ZH from 'xgplayer/es/lang/zh-cn'

// 启用中文
I18N.use(ZH)

defineOptions({
    name: "VideoSupport",
})

interface Props {
    src?: string
}
const props = withDefaults(defineProps<Props>(),{
    src: ""
})
let player: Player | null = null;
onMounted(() => {
    if (player !== null){
        player.switchURL(props.src)
    } else {
        player = new Player({
            id: 'videos',
            url: props.src,
            height: '100%',
            width: '100%',
        })
    }
})
</script>

<template>
    <div class="video-support">
        <div class="video-support-inner">
            <div id="videos"></div>
<!--            <video :src="props.src" controls></video>-->
        </div>
    </div>
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