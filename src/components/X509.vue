<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const xPem = ref("");
const xRes = ref("");

async function parse() {
    try {
        console.log("pem:", xPem.value);
        xRes.value = await invoke("x509_parse", { pem: xPem.value });
    } catch (error) {
        console.log("--->", error);
    }
}

</script>

<template>
    <textarea class="input" rows="10" v-model="xPem"></textarea>
    <textarea class="input" rows="10" v-model="xRes"></textarea>
    <button class="btn" @click="parse">Parse</button>
</template>

<style scoped>
.input {
    display: flex;
    margin-top: 1vh;
    margin-bottom: 1vh;

    border-radius: 2px;
    border: 1px solid transparent;
    padding: auto;

    color: black;

    font-size: large;
    font-family: 'Courier New', Courier, monospace;

    outline: none;
}

.btn {
    width: 100px;
}
</style>