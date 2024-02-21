<script setup lang="ts">
import { toRefs } from 'vue';
import { FileItem } from './FileItem';
const props = defineProps<FileItem>()
const { name, queryUrl } = toRefs(props);
</script>
<template>
    <div class="flex flex-col items-center
  max-sm:w-1/3 sm:w-1/3 md:w-1/4 lg:w-1/6 xl:w-1/12 2xl:w-1/16
  text-center hover:bg-gray-300 cursor-pointer">
        <div class="text-7xl">
            📁
        </div>
        <span class="">
            {{ name.slice(0, -1) }}
        </span>
    </div>
</template>