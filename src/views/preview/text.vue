<script setup lang="ts">
import { ref, watch } from 'vue'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { codeToHtml } from 'shiki'

defineOptions({
    name: 'TextSupport',
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
            const txt = await readTextFile(props.src)

            content.value = txt
        }
    },
    { immediate: true },
)
</script>

<template>
    <div class="text-support">
        <div class="text-support-inner">
            <pre>{{ content }}</pre>
        </div>
    </div>
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
        & pre {
            white-space: pre-wrap;
            word-wrap: break-word;
            font-family: inherit;
            font-size: inherit;
        }
    }
}
</style>
