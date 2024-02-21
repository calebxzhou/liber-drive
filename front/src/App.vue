<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import FileItem from './components/FileItem.ts'
import { toReadableSize, extractNumbers } from './util.js'
import { GALLERY } from './const.js'
import { Ref } from 'vue'

const serverUrl = 'http://' + window.location.hostname + ':7789/'

//顶栏标题
let title = ref("首页")
//现在路径
let pathNow: Ref<string[]> = ref([])
//是否显示viewer
let showViewer = ref(false)
//图片尺寸 bytes
let imageSizeNow = ref(0)
//图片exif信息
let imageExifNow = ref({})
//文件项目展示模式，默认为 相册
let displayMode = ref(GALLERY)
//第x个文件
let viewingMediaIndex = ref(0)
//所有文件
let allFiles = ref<FileItem>([])
let allDirs: Ref<FileItem[]> = ref([])

const getPathNow = () => pathNow.value.join('/').replaceAll("//", "/").replaceAll("..", "")
const getQueryUrl = () => serverUrl + getPathNow()
function  viewingMedia(): FileItem {return  allFiles.value[viewingMediaIndex.value];}
        
async function getFileList() {
    const res = await fetch(getQueryUrl())
    const array = await res.json()
    array.sort((a: string, b: string) => b.localeCompare(a))
    return array
}

async function renderFiles() {
    allFiles.value = []
    let fileNames = await getFileList()
    let items = fileNames.map((name: string) => new FileItem(name, getQueryUrl() + name))
    let dirs = items.filter((i: FileItem) => i.isDir())
    if (displayMode.value === GALLERY) {
        items = items.filter((i: FileItem) => i.isMedia())
    }
    allFiles.value = items
    allDirs.value = dirs
}
function getExifStr() {
    let m = viewingMedia();
    return `${m.name} 
            📷${imageExifNow.make} 
            🔭${imageExifNow.lens} 
            📐${imageExifNow.focal_len}(🔎${(extractNumbers(imageExifNow.focal_len) / 23).toFixed(2)}x) 
            📸${imageExifNow.xp_prog}挡 👁️${imageExifNow.aperture} ⏱${imageExifNow.shutter} ISO${imageExifNow.iso}
             ⏰${imageExifNow.shot_time}`;
}
function openOriginal() {
    window.open(viewingMedia().queryUrl);
}
function goNextDir(dirName: string) {
    pathNow.value.push(dirName)
    title.value = getPathNow()
    renderFiles()
}

function goPrevDir() {
    pathNow.value.pop()
    title.value = getPathNow()
    renderFiles()
}
function openMedia(idx) {
    showViewer = !showViewer;
    viewingMediaIndex = idx;
    updateMedia();

}
function updateMedia() {
    updateImageSize();
    updateImageExif();
}
function prevMedia() {
    if (viewingMediaIndex > 0) {
        viewingMediaIndex--;
    }
    updateMedia();
}
function
    nextMedia() {
    if (viewingMediaIndex < allFiles.length - 1) {
        viewingMediaIndex++;
    }
    updateMedia();
}
function updateImageSize() {
    fetch(viewingMedia().queryUrl + '?meta=size')
        .then(res => res.text())
        .then(data => imageSizeNow = data);
}
function updateImageExif() {
    fetch(viewingMedia().queryUrl + '?meta=exif')
        .then(res => res.json())
        .then((data) => {
            imageExifNow = data;
        }).catch((error) => {
            imageExifNow = null;
            console.log(error);
        }

        );

}
function onKeyDown(event: KeyboardEvent) {
    switch (event.key) {
        case 'Backspace':
            goPrevDir()
            break
        case 'ArrowLeft':
            if (showViewer.value) {
                prevMedia()
            }
            break
        case 'ArrowRight':
            if (showViewer.value) {
                nextMedia()
            }
            break
        case 'Escape':
            if (showViewer.value) {
                showViewer.value = false
            } else {
                goPrevDir()
            }
    }
}

onMounted(async () => {
    window.addEventListener('keydown', onKeyDown)
    await renderFiles()
})

onUnmounted(() => {
    window.removeEventListener('keydown', onKeyDown)
})

// ... rest of your methods

</script>


<template>
    <!-- 观看器 -->
    <div v-if="showViewer" class="fixed inset-0 flex flex-col justify-center z-10 bg-black bg-opacity-40">
        <!-- 顶部 -->
        <div class="absolute top-0 items-center">
            <!-- 关闭按钮 -->
            <button class="text-2xl mr-3" @click="openMedia">❌</button>
            <button class="mr-2" @click="alertExif">📸🔎.. </button>
            <!-- 文件名 尺寸 -->
            <span v-if="imageExifNow != null" class="max-lg:hidden">
                {{ getExifStr() }}
            </span>
        </div>
        <img v-if="allFiles[viewingFileIndex].isImg()" :src="allFiles[viewingFileIndex].queryUrl + '?preview=0'"
            class="block mx-auto object-contain h-[90vh]" />
        <video controls crossorigin playsinline v-if="allFiles[viewingFileIndex].isVideo()"
            :src="allFiles[viewingFileIndex].queryUrl" class="block mx-auto object-contain h-[90vh]"></video>
        <div class="absolute bottom-0 text-center text-white mb-4 w-full">
            <button @click="prevMedia()">⬅️上一张</button>&emsp;
            <button @click="openOriginal()">💾下载原图{{ toReadableSize(imageSizeNow) }}</button>&emsp;
            <button @click="nextMedia()">下一张➡️</button>&emsp;

        </div>
    </div>
    <!-- 导航栏 -->
    <nav class="flex sticky top-0 bg-white shadow-md items-center">
        <button @click="goPrevDir()" class="text-4xl mr-2">◀️</button>
        <span class="text-xl">{{ title }}</span>
        <!--button @click="toggleDisplayMode()" class="text-2xl mr-1"></button-->
    </nav>
    <div class="flex flex-wrap justify-start pt-2">
        <!-- 目录 -->
        <File v-for="(item, index) in allDirs" :key="index" :name="item.name" :queryUrl="getQueryUrl() + item.name"
            :mode="displayMode" @click="goNextDir(item.name)" />
        <!-- 媒体 -->
        <File v-for="(item, index) in allFiles" :key="index" :name="item.name" :queryUrl="getQueryUrl() + item.name"
            :mode="displayMode" @click="openMedia(index)" />
    </div>
    <span class="text-2xl" v-if="allFiles.length == 0 && allDirs.length == 0">
        <hr>空
    </span>
</template>
 
