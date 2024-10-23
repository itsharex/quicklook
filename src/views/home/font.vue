<script setup lang="ts">
import { ref, onMounted } from "vue"

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

const isError = ref(false)

onMounted(() => {
    if (props.src) {
        const font = new FontFace('MyFont', `url(${props.src})`);
        // 加载字体
        font.load().then((loadedFont) => {
            if(curFont) {
                document.fonts.delete(curFont)
                curFont = null
            }
            curFont = loadedFont

            // 将字体添加到文档中
            document.fonts.add(loadedFont);

            // 使用字体
            fontFamily.value = 'MyFont, sans-serif';
            isError.value = false
        }).catch(function(error) {
            isError.value = true
            console.error('Font could not be loaded: ' + error);
        });
    }
})
</script>

<template>
<div class="font-support">
    <div class="font-support-inner" :style="{fontFamily: fontFamily }">
        <template v-if="isError">
            字体解析错误
        </template>
        <template v-else>
            <p>ABCDEFGHIGKLMNOPQRSTUVWXYZ</p>
            <p>abcdefghigklmnopqrstuvwxyz</p>
            <p>0123456789</p>
            <p>中国制造惠及全球</p>
        </template>
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