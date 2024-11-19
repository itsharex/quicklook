<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { Leafer, Image } from 'leafer-ui'
import { useRoute } from 'vue-router'

defineOptions({
    name: 'ImageSupport',
})

const route = useRoute()
let leafer: Leafer | null = null
let image: Image | null = null

const init = () => {
    if (leafer !== null) {
        leafer?.destroy()
        image?.destroy()
    }
    leafer = new Leafer({
        view: 'canvas', // 支持 window 、div、canvas 标签对象， 可使用id字符串(不用加 # 号)
        width: 0, // 不能设置为 0， 否则会变成自动布局
        height: 0,
    })

    image = new Image({
        url: route.query.src as string,
        draggable: true,
    })

    leafer.add(image)
}
onMounted(() => {
    init()
})

watch(
    () => route.path,
    (val, oldVal) => {
        if (val !== oldVal) {
            init()
        }
    },
)
</script>

<template>
    <div class="image-support">
        <div class="image-support-inner" id="canvas">
            <!--        <img :src="props.src" alt="image" crossorigin="anonymous">-->
        </div>
    </div>
</template>

<style scoped lang="scss">
.image-support {
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
    & img {
        max-width: 100%;
        max-height: 100%;
    }
}
</style>
