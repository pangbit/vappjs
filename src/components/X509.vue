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
    <div class="box">
        <div class="btns">
            <button class="btn" @click="parse">Parse</button>
        </div>
        <textarea class="show" rows="10" v-model="xPem"></textarea>
        <textarea class="show output" rows="10" v-model="xRes"></textarea>
    </div>
</template>

<style scoped>
.show {
    min-height: 50px;
    height: 200px;
}

.output {
    margin-top: 5px;
}
</style>