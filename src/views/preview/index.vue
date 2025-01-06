<script setup lang="ts"></script>

<template>
    <router-view v-slot="{ Component }">
        <template v-if="Component">
            <transition mode="out-in">
                <component :is="Component"></component>
            </transition>
        </template>
    </router-view>
</template>

<style scoped lang="scss">
.preview {
    &-loading {
        width: 100vw;
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: rgba(0, 0, 0, 0.2);
        color: white;
        & .spin {
            font-size: 34px;
            animation: spin 1.4s linear infinite;
        }
        @keyframes spin {
            from {
                transform: rotate(0deg);
            }
            to {
                transform: rotate(360deg);
            }
        }
    }
}
</style>
