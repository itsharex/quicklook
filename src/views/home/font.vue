<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue"

defineOptions({
    name: "FontSupport",
})

interface Props {
    src?: string
}
const props = withDefaults(defineProps<Props>(),{
    src: ""
})

const fontFamily = ref<string>("")
let curFont: FontFace | null = null

onMounted(() => {
    if (props.src) {
        const font = new FontFace('MyFont', `url(${props.src})`);
        // 加载字体
        font.load().then((loadedFont) => {
            curFont = loadedFont
            // 将字体添加到文档中
            document.fonts.add(loadedFont);

            // 使用字体
            fontFamily.value = 'MyFont, sans-serif';
        }).catch(function(error) {
            console.error('Font could not be loaded: ' + error);
        });
    }
})
onUnmounted(() => {
    if(curFont) {
        document.fonts.delete(curFont)
        curFont = null
    }
})
</script>

<template>
<div class="font-support">
    <div class="font-support-inner" :style="{fontFamily: fontFamily }">
        <p>ABCDEFGHIGKLMNOPQRSTUVWXYZ</p>
        <p>abcdefghigklmnopqrstuvwxyz</p>
        <p>0123456789</p>
        <p>中国制造惠及全球</p>
    </div>
</div>
</template>

<style scoped lang="scss">
.font-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    &-inner{
        font-size: 24px;
    }
    & p {
        font-family: inherit;
        font-size: inherit;
    }
}
</style>