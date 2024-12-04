<script setup lang="ts">
interface Props {
    content: string | undefined
}
const props = defineProps<Props>()
</script>

<template>
    <div v-html="props.content" class="vp-doc"></div>
</template>

<style>
@import './styles/vars.css';
@import './styles/base.css';
@import './styles/icons.css';
@import './styles/utils.css';
@import './styles/components/custom-block.css';
@import './styles/components/vp-doc.css';
</style>
