<script setup lang="ts">
import { FILE_TYPE_EXTENSION } from '@/const';
import axios from 'axios';
import { toRefs } from 'vue';
import { ref, onMounted } from 'vue' 
import { FileItem } from './FileItem';
const props = defineProps<FileItem>()
const {name,queryUrl} = toRefs(props);
//扩展名
const extension = name.value
      .slice(((name.value.lastIndexOf(".") - 1) >>> 0) + 2)
      .toLowerCase()
//是否目录
const isDir = name.value.endsWith('/');
//显示名称 不显示下划线 不显示结尾斜杠
const displayName = name.value.replaceAll("_", " ").replaceAll(".", " .").replaceAll("/","")
//类型 无法识别则为file
const type = Object.entries(FILE_TYPE_EXTENSION).reduce(
      (acc, [key, value]) =>
        value.includes(extension) ? key : acc,
      "file"
    ); 
//是否图片
const isImage = type === 'img';
//是否视频
const isVideo = type === 'video';
//是否媒体（图片+视频）
const isMedia = isImage || isVideo;
//图标
const icon: string = isDir? '📁': (isImage?'🖼️':isVideo?'🎥':'📜')
//预览图      
const preview = ref("");
//载入进度
const loadPercent = ref(0);

//载入预览图
async function fetchPreview(viewLvl:number){
    let resp = await axios.get(queryUrl.value + "?preview=" + viewLvl, {
        responseType: 'blob',
        onDownloadProgress: function (progressEvent) {
            if (progressEvent.total) {
                return loadPercent.value = Math.round((progressEvent.loaded * 100) / progressEvent.total);
            }
        }
    })
    var blob = window.URL.createObjectURL(new Blob([resp.data]));
    preview.value = blob;
}

onMounted(() => {
        if (isMedia) {
            //预览
            fetchPreview(2); //128px
        }

})
</script>


<template>
    <div class="flex flex-col items-center
  max-sm:w-1/3 sm:w-1/3 md:w-1/4 lg:w-1/6 xl:w-1/12 2xl:w-1/16
  text-center hover:bg-gray-300 cursor-pointer">
            
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


</template>
