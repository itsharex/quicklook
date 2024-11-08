<script setup lang="ts">
import { ref, watch } from 'vue'
import { readTextFile } from '@tauri-apps/plugin-fs'
import MarkdownIt from 'markdown-it'

const md = new MarkdownIt()

defineOptions({
    name: 'MdSupport',
})

interface Props {
    src?: string
}
const props = withDefaults(defineProps<Props>(), {
    src: '',
})

const content = ref<string>()

watch(
    () => props.src,
    async (val, old) => {
        if (!old || val !== old) {
            const ctx = await readTextFile(props.src)
            content.value = md.render(ctx || '')
        }
    },
    { immediate: true },
)
</script>

<template>
    <div class="video-support">
        <div class="video-support-inner" v-html="content"></div>
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
