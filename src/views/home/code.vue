<script setup lang="ts">
import { ref, watch } from 'vue'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { codeToHtml } from 'shiki'

defineOptions({
    name: 'CodeSupport',
})

interface Props {
    src?: string
    type?: string
}
const props = withDefaults(defineProps<Props>(), {
    src: '',
    type: '',
})

const content = ref<string>()

watch(
    () => props.src,
    async (val, old) => {
        if (!old || val !== old) {
            const code = await readTextFile(props.src)
            const html = await codeToHtml(code, {
                lang: 'javascript',
                themes: {
                    light: 'github-light',
                    dark: 'github-dark',
                },
            })
            content.value = html
        }
    },
    { immediate: true },
)
</script>

<template>
    <div class="code-support">
        <div class="code-support-inner" v-html="content"></div>
    </div>
</template>

<style scoped lang="scss">
.code-support {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    align-content: center;
    &-inner {
        width: 100%;
        height: 100%;
        overflow: auto;
        padding: 12px 24px;
        font-size: 14px;
        & :deep(pre code) {
            font-family: 'Courier New', Courier, monospace;
        }
    }
}
</style>
