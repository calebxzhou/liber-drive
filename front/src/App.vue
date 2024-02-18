<!-- eslint-disable no-irregular-whitespace -->
<script>
import File from './components/FileItem.vue'
import FileItem from './components/file_item';
import { toReadableSize, extractNumbers } from './util.js';
import { GALLERY } from './const.js';
import { onMounted } from 'vue'

const serverUrl = 'http://' + window.location.hostname + ':7789/';


/**
 * 渲染文件项目
 */
async function renderFiles() {
    allFiles.value = {};
    let fileNames = await getFileList();
    let items = fileNames.map((name) => new FileItem(name, getQueryUrl() + name));
    let dirs = items.filter((i) => i.isDir());
    //相册模式 只保留 媒体
    if (displayMode.value === GALLERY) {
        items = items.filter((i) => i.isMedia());
    }
    allFiles.value = items;
    allDirs.value = dirs;
}

export default {
    data() {
        return {
            //顶栏标题
            title: "首页",
            //现在路径
            pathNow: [],
            //是否显示viewer
            showViewer: false,
            //图片尺寸
            imageSizeNow: 0,
            //图片exif信息
            imageExifNow: {},
            //文件项目展示模式，默认为 相册
            displayMode: GALLERY,
            //第x个文件
            viewingMediaIndex: 0,
            //所有文件
            allFiles: [],
            //所有目录
            allDirs: []
        }
    },
    methods: {
        //路径
        getPathNow() {
            return this.pathNow.join('/').replaceAll("//", "/").replaceAll("..", "");
        },
        //请求url
        getQueryUrl() {
            return serverUrl + this.getPathNow();
        },
        /**
         * 进入目录
         * @param {string} dirName 目录名
         */
        goNextDir(dirName) {
            this.pathNow.push(dirName);
            this.title = this;
            renderFiles();
        },
        /**
         * 获取文件列表
         * @returns {string[]}
         */
        async getFileList() {
            const res = await fetch(this.getQueryUrl());
            const array = await res.json();
            // Sort file names in reverse order
            array.sort((a, b) => b.localeCompare(a));
            return array;
        },
        viewingMedia() {
            return this.allFiles[this.viewingMediaIndex];
        },
        /**
         * 去上级目录
         */
        goPrevDir() {
            this.pathNow.pop();
            this.title = this.getPathNow();
            renderFiles();
        },
        getQueryUrl() {
            return getQueryUrl();
        },
        toggleDisplayMode() {


        },
        openMedia(idx) {
            showViewer.value = !showViewer.value;
            viewingMediaIndex.value = idx;
            this.updateMedia();

        },
        updateMedia() {
            this.updateImageSize();
            this.updateImageExif();
        },
        prevMedia() {
            if (viewingMediaIndex.value > 0) {
                viewingMediaIndex.value--;
            }
            this.updateMedia();
        },
        nextMedia() {
            if (viewingMediaIndex.value < allFiles.value.length - 1) {
                viewingMediaIndex.value++;
            }
            this.updateMedia();
        },

        updateImageSize() {
            fetch(viewingMedia().queryUrl + '?meta=size')
                .then(res => res.text())
                .then(data => this.imageSizeNow = data);
        },
        updateImageExif() {
            fetch(viewingMedia().queryUrl + '?meta=exif')
                .then(res => res.json())
                .then((data) => {
                    this.imageExifNow = data;
                }).catch((error) => {
                    this.imageExifNow = null;
                }

                );

        },
        onKeyDown(event) {

            switch (event.key) {
                case 'Backspace':
                    this.goPrevDir();
                    break;
                case 'ArrowLeft':
                    if (showViewer.value) {
                        this.prevMedia();
                    }
                    break;
                case 'ArrowRight':
                    if (showViewer.value) {
                        this.nextMedia();
                    }
                    break;
                case 'Escape':
                    if (showViewer.value) {
                        showViewer.value = false;
                    } else {
                        this.goPrevDir();
                    }

            }

        },
        //要显示的exif信息
        getExifStr() {
            let m = viewingMedia();
            return `${m.name} 
            📷${this.imageExifNow.make} 
            🔭${this.imageExifNow.lens} 
            📐${this.imageExifNow.focal_len}(🔎${(extractNumbers(this.imageExifNow.focal_len) / 23).toFixed(2)}x) 
            📸${this.imageExifNow.xp_prog}挡 👁️${this.imageExifNow.aperture} ⏱${this.imageExifNow.shutter} ISO${this.imageExifNow.iso}
             ⏰${this.imageExifNow.shot_time}`;
        },
        alertExif() { alert(this.getExifStr()) },
        openOriginal() {
            window.open(viewingMedia().queryUrl);
        },
        toReadableSize(size) {
            return toReadableSize(size);
        }
    },
    mounted() {
        // Attach the event listener for the backspace key
        window.addEventListener('keydown', this.onKeyDown);
    },
    beforeUnmount() {
        // Clean up: remove the event listener when the component is unmounted
        window.removeEventListener('keydown', this.onKeyDown);
    },

    components: {
        File
    },
    setup() {
        onMounted(() => {
            //获取文件信息
            renderFiles();
        })
        return {
            allFiles, allDirs, showViewer, viewingFileIndex: viewingMediaIndex, displayMode
        }
    }
};
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
 
