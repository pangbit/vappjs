<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const xResult = ref("");
const xUseProxy = ref(false);

async function ipinfo() {
    xResult.value = await invoke("ipinfo", { useProxy: xUseProxy.value });
}

</script>


<template>
    <div class="box">
        <div class="btns">
            <button class="btn" @click="ipinfo">本机IP</button>
        </div>
        <label class="check"><input type="checkbox" v-model="xUseProxy">使用系统代理</label>
        <textarea class="show" rows="20" v-model="xResult"></textarea>
    </div>
</template>

<style scoped>
.check {
    width: 8em;
}

</style>