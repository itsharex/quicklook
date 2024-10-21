import { Window, type WindowLabel } from '@tauri-apps/api/window'

export const getWindow = async (label: WindowLabel) => {
    return await Window.getByLabel(label)
}

/**
 * 获取类型
 * @param val unknown
 */
export const type = (val: any) => {
    return Object.prototype.toString.call(val).slice(8, -1).toLowerCase()
}

/**
 * 是否对象
 * @param val
 */
export const isObject = (val: any) => {
    return type(val) === 'object'
}

/**
 * 是否数组
 * @param val
 */
export const isArray = (val: any) => {
    return type(val) === 'array'
}
