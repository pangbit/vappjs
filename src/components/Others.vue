<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const xResult = ref("");

async function ipinfo() {
    xResult.value = await invoke("ipinfo");
}

</script>


<template>
    <div class="box">
        <div class="btns">
            <button class="btn" @click="ipinfo">本机IP</button>
        </div>
        <textarea class="show" rows="20" v-model="xResult"></textarea>
    </div>
</template>

<style scoped></style>