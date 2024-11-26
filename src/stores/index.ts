import { defineStore } from 'pinia'

interface MainState {
    audio: Array<string>
    audioChecked: Array<string>
    video: Array<string>
    videoChecked: Array<string>
}

export const useMainStore = defineStore('main', {
    state: (): MainState => {
        return {
            audio: [],
            audioChecked: [],
            video: [],
            videoChecked: [],
        }
    },
})
