<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const xText = ref("");
const xError = ref("");

async function jsonFormat() {
    let input = xText.value;
    if (input == null || input == "") {
        return;
    }

    try {
        xText.value = await invoke("json_format", { input: input });
        xError.value = "";
    } catch (error) {
        xError.value = error;
    }

}

async function xmlFormat() {
    let input = xText.value;
    if (input == null | input == "") {
        return;
    }

    try {
        xText.value = await invoke("xml_format", { input: input });
        xError.value = "";
    } catch (error) {
        xError.value = error;
    }
}

</script>

<template>
    <div class="box">
        <div class="btns">
            <button class="btn" @click="jsonFormat">Json</button>
            <button class="btn" @click="xmlFormat">XML</button>
        </div>
        <textarea class="show" v-model="xText" rows="10"></textarea>
        <div>
            <p class="error">{{ xError }}</p>
        </div>
    </div>
</template>

<style scoped>
.show {
    min-height: 300px;
}

.error {
    color: #f37171;
}
</style>