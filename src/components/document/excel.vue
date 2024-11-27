<script lang="ts" setup>
import Sheet from './sheet.vue'

defineOptions({
    name: 'Excel',
})

interface Sheet {
    name: string
    rows: string[][]
}

interface Props {
    data?: Array<Sheet>
}
const props = defineProps<Props>()
</script>

<template>
    <div>
        <div>
            <Sheet v-for="sheet in props.data" :key="sheet.name" :data="sheet" />
        </div>
        <div>
            <div v-for="sheet in props.data" :key="sheet.name">{{ sheet.name }}</div>
        </div>
    </div>
</template>
