import { Window, type WindowLabel } from '@tauri-apps/api/window'

export const getWindow = async (label: WindowLabel) => {
    return await Window.getByLabel(label)
}

export class Type {
    /**
     * 获取类型
     * @param val
     * @returns
     */
    static get = (val: unknown) => {
        return Object.prototype.toString.call(val).slice(8, -1).toLowerCase()
    }
    /**
     * 是否对象
     * @param val
     * @returns
     */
    static isObject = (val: unknown) => {
        return Type.get(val) === 'object'
    }
    /**
     * 是否数组
     * @param val
     * @returns
     */
    static isArray = (val: unknown) => {
        return Type.get(val) === 'array'
    }
    /**
     * 是否函数
     * @param val
     * @returns
     */
    static isFunction = (val: unknown) => {
        return Type.get(val) === 'function'
    }
    /**
     * 是否字符串
     * @param val
     * @returns
     */
    static isString = (val: unknown) => {
        return Type.get(val) === 'string'
    }
    /**
     * 是否数字
     * @param val
     * @returns
     */
    static isNumber = (val: unknown) => {
        return Type.get(val) === 'number'
    }
    /**
     * 是否布尔
     * @param val
     * @returns
     */
    static isBoolean = (val: unknown) => {
        return Type.get(val) === 'boolean'
    }
    /**
     * 是否undefined
     * @param val
     * @returns
     */
    static isUndefined = (val: unknown) => {
        return Type.get(val) === 'undefined'
    }
    /**
     * 是否null
     * @param val
     * @returns
     */
    static isNull = (val: unknown) => {
        return Type.get(val) === 'null'
    }
    /**
     * 是否symbol
     * @param val
     * @returns
     */
    static isSymbol = (val: unknown) => {
        return Type.get(val) === 'symbol'
    }
    /**
     * 是否bigint
     * @param val
     * @returns
     */
    static isBigInt = (val: unknown) => {
        return Type.get(val) === 'bigint'
    }
}
