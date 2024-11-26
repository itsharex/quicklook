<script lang="ts" setup>
import { ref } from 'vue'

const imageList = ['png', 'jpg', 'jpeg', 'gif', 'svg', 'apng']
const imageChecked = ref([])
</script>

<template>
    <div class="setting">
        <h1>This is an settings page</h1>
        <div class="setting-item support">
            <h2>支持的格式</h2>
            <div class="support-item">
                <div class="support-item-header">
                    <span>图像（Image）</span>
                </div>
                <div class="support-item-body">
                    <el-checkbox-group class="support-item-body-wrap" v-model="imageChecked">
                        <template v-for="item in imageList" :key="item">
                            <div class="support-item-body-item">
                                <el-checkbox :value="item" :label="item" />
                            </div>
                        </template>
                    </el-checkbox-group>
                </div>
            </div>
            <div class="support-item">
                <div class="support-item-header">
                    <span>视频（Video）</span>
                </div>
                <div class="support-item-body">
                    <el-checkbox-group class="support-item-body-wrap" v-model="imageChecked">
                        <template v-for="item in imageList" :key="item">
                            <div class="support-item-body-item">
                                <el-checkbox :value="item" :label="item" />
                            </div>
                        </template>
                    </el-checkbox-group>
                </div>
            </div>
            <div class="support-item">
                <div class="support-item-header">
                    <span>字体（Font）</span>
                </div>
                <div class="support-item-body">
                    <el-checkbox-group class="support-item-body-wrap" v-model="imageChecked">
                        <template v-for="item in imageList" :key="item">
                            <div class="support-item-body-item">
                                <el-checkbox :value="item" :label="item" />
                            </div>
                        </template>
                    </el-checkbox-group>
                </div>
            </div>
            <div class="support-item">
                <div class="support-item-header">
                    <span>书籍（Book）</span>
                </div>
                <div class="support-item-body">
                    <el-checkbox-group class="support-item-body-wrap" v-model="imageChecked">
                        <template v-for="item in imageList" :key="item">
                            <div class="support-item-body-item">
                                <el-checkbox :value="item" :label="item" />
                            </div>
                        </template>
                    </el-checkbox-group>
                </div>
            </div>
        </div>
        <div class="setting-item">
            <h2>更新</h2>
            <div>
                <el-button type="primary">检查更新</el-button>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.setting {
    padding: 16px;
    width: 100%;
}
.support {
    &-item {
        padding: 0 12px;
        & + & {
            margin-bottom: 24px;
        }

        &-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid #ebeef5;
            padding: 12px 0;
        }

        &-body {
            padding: 12px 16px;
            .support-item-body-wrap {
                display: flex;
                flex-wrap: wrap;
                .support-item-body-item {
                    flex: 0 0 80px;
                }
            }
        }
    }
}
</style>
