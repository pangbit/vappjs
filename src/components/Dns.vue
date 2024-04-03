<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const xHost = ref("");
const xResult = ref("");
const xError = ref("");

async function dnsLookup() {
    try {
        let records = await invoke("dns_lookup", { host: xHost.value });
        xResult.value = records.join("\n");
    } catch (error) {
        xError.value = error;
    }
}

</script>

<template>
    <div class="box">
        <label>Host</label>
        <form class="form-box" @submit.prevent="dnsLookup">
            <input class="form-input" v-model="xHost" />
            <button class="btn form-btn" type="submit">Query</button>
        </form>
        <label>Records</label>
        <textarea class="show" v-model="xResult"></textarea>
        <p class="error">{{ xError }}</p>
    </div>
</template>

<style scoped>
.show {
    margin-bottom: 5px;
    min-height: 50px;
    height: 200px;
}

.error {
    color: #f37171;
}
</style>