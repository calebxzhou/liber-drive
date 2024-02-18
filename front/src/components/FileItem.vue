<script setup>
import axios from 'axios';
import FileItem from './file_item.js'

import { ref, onMounted } from 'vue'
const props = defineProps({
    name: String,
    queryUrl: String,
    mode: String
})
const fileItem = new FileItem(props.name, props.queryUrl);
const mediaPreview = ref(null);
const loadPercent = ref(0);
/**
 * 文件的显示名称 不显示下划线 目录不显示结尾斜杠
 */
const displayName = () => {
    let name = props.name.replaceAll("_", " ").replaceAll(".", " .");
    if (fileItem.isDir()) {
        name = name.slice(0, -1);
    }
    return name;
}
/**
 * 载入预览图
 * @param {number} viewLvl 
 */
const fetchPreview = (viewLvl) => {
    axios.get(props.queryUrl + "?preview=" + viewLvl, {
        responseType: 'blob',
        onDownloadProgress: function (progressEvent) {
            loadPercent.value = Math.round((progressEvent.loaded * 100) / progressEvent.total);
        }
    })
        .then(function (response) {
            var blob = window.URL.createObjectURL(new Blob([response.data]));
            mediaPreview.value = blob;
        })
        .catch(function (error) {
            console.log('Failed to download image: ' + error);
        });
}
onMounted(() => {
    //相册模式
    if (props.mode === "GALLERY") {
        //媒体文件
        if (fileItem.isMedia()) {
            //预览
            fetchPreview(2); //128px
        }
    }

})
</script>


<template>
    <div class="flex flex-col items-center
  max-sm:w-1/3 sm:w-1/3 md:w-1/4 lg:w-1/6 xl:w-1/12 2xl:w-1/16
  text-center hover:bg-gray-300 cursor-pointer">
        <!-- 相册模式 -->
        <div v-if="mode === 'GALLERY'">
            <!-- 目录 -->
            <div v-if="fileItem.isDir()">
                <div class="text-7xl">
                    {{ fileItem.getIcon() }}
                </div>
                <span class="">
                    {{ displayName() }}
                </span>
            </div>
            <!-- 媒体 -->
            <div v-if="fileItem.getType() === 'img'">
                <!-- 预览载入中 -->
                <div v-if="mediaPreview === null">
                    <v-progress-circular :rotate="360" :size="64" :width="15" :model-value="loadPercent" color="teal">
                        <template v-slot:default> {{ loadPercent }} % </template>
                    </v-progress-circular>
                </div>
                <!-- 预览载入完成 -->
                <div v-else>
                    <img class="w-32 h-32 object-cover object-center" :src="mediaPreview">
                </div>
            </div>
            <div v-if="fileItem.getType() === 'video'">
                <div class="text-7xl">
                    {{ fileItem.getIcon() }}
                </div>
                <span class="">
                    {{ displayName() }}
                </span>

            </div>
        </div>
        <!-- 文件模式 TODO -->
        <div v-if="mode === 'FILE'">
            <div v-if="!fileItem.isMedia()" class="text-7xl">
                {{ fileItem.getIcon() }}
            </div>
            <span class="">
                {{ displayName() }}
            </span>
        </div>


    </div>
</template>
