<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/primitives";

const xText = ref("");
const xError = ref("");

async function json_format() {
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

async function xml_format() {
    let input = xText.value;
    if (input == null |input == "") {
        return;
    }

    try {
        xText.value = await invoke("xml_format", {input: input});
        xError.value = "";
    } catch (error) {
        xError.value = error; 
    }
}

</script>

<template>
    <textarea class="row" v-model="xText" rows="10"></textarea>
    <div>
        <button class="btn" @click="json_format">Json</button>
        <button class="btn" @click="xml_format">XML</button>
    </div>
    <div>
        <p>{{ xError }}</p>
    </div>
</template>