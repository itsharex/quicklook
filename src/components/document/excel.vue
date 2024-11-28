<script lang="ts" setup>
import { computed, ref } from 'vue'
import { HotTable } from '@handsontable/vue3'
import 'handsontable/dist/handsontable.full.css'

// translation modules
import { registerLanguageDictionary, zhCN } from 'handsontable/i18n'

// registering functions that let you quickly register all modules at once
import {
    registerAllCellTypes,
    registerAllRenderers,
    registerAllValidators,
    registerAllPlugins,
    registerAllModules,
} from 'handsontable/registry'

// register all cell types at once
registerAllCellTypes()

// register all renderers at once
registerAllRenderers()

// register all validators at once
registerAllValidators()

// register all plugins at once
registerAllPlugins()

// register individual translations
registerLanguageDictionary(zhCN)

// or, register all of Handsontable's modules at once
registerAllModules()

defineOptions({
    name: 'Excel',
})

// register Handsontable's modules
registerAllModules()

interface Sheet {
    name: string
    rows: string[][]
}

interface Props {
    data?: Array<Sheet>
}
const props = defineProps<Props>()
const name = ref<string | null>(null)
const activeSheet = computed({
    get: () => {
        if (name.value === null && props.data) {
            return props.data[0].name
        } else {
            return name.value
        }
    },
    set(val: string) {
        name.value = val
    },
})
</script>

<template>
    <div style="height: 100%">
        <div style="height: calc(100% - 32px)">
            <template v-for="sheet in props.data" :key="sheet.name">
                <HotTable
                    v-show="sheet.name == activeSheet"
                    :settings="{
                        data: sheet.rows,
                        readOnly: true,
                        rowHeaders: true,
                        colHeaders: true,
                        width: '100%',
                        height: '100%',
                        stretchH: 'all',
                        autoWrapRow: true,
                        autoWrapCol: true,
                        manualColumnResize: true,
                        licenseKey: 'non-commercial-and-evaluation',
                    }"
                />
            </template>
        </div>
        <div style="height: 32px; display: flex; justify-content: flex-start; align-items: center; padding: 0 12px">
            <el-space>
                <div
                    v-for="sheet in props.data"
                    :key="sheet.name"
                    :class="['sheet-name', { 'is-active': sheet.name == activeSheet }]"
                    @click="activeSheet = sheet.name"
                >
                    {{ sheet.name }}
                </div>
            </el-space>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.sheet {
    &-name {
        padding: 0 12px;
        cursor: pointer;
        &:hover {
            color: var(--el-color-primary-lighter-2);
        }
        &.is-active {
            color: var(--el-color-primary);
            cursor: default;
        }
    }
}
</style>
