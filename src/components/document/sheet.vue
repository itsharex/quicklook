<script lang="ts" setup>
defineOptions({
    name: 'Sheet',
})

interface Props {
    data: {
        name: string
        rows: string[][]
    }
}
const props = defineProps<Props>()
</script>

<template>
    <table class="table">
        <tr v-for="(row, rowIndex) in props.data.rows" :key="rowIndex" class="table-row">
            <td v-for="(col, colIndex) in row" :key="colIndex" class="table-col">{{ col }}</td>
        </tr>
    </table>
</template>

<style scoped lang="scss">
.table {
    border-collapse: collapse;
    overflow: hidden;
    &-row {
    }
    &-col {
        padding: 4px 8px;
        border: 1px solid #ccc;
        min-width: 80px;
    }
}
</style>
