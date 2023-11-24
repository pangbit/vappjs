<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/primitives";

const xText = ref("");
const xPattern = ref("");
const xResult = ref("");

async function is_match() {
    try {
        if (xText.value == "") {
            xResult.value = "[Warning] Empty Data"
            return
        }

        let result = await invoke("regex_is_match", { pattern: xPattern.value, text: xText.value });
        if (result) {
            xResult.value = "Match";
        } else {
            xResult.value = "Not Match";
        }
    } catch (error) {
        let einfo = error.toString();
        einfo = einfo.replaceAll("\\n", "\n");
        xResult.value = "[Error]: \n\n" + einfo;
    }
}


</script>

<template>
    <label>REGULAR EXPRESSION</label>
    <form class="row" @submit.prevent="is_match">
        <input id="regex-input" v-model="xPattern" placeholder="insert your regular expression here" />
        <button type="submit">Go</button>
    </form>
    <label>TEST STRING</label>
    <textarea class="row" rows="10" v-model="xText" placeholder="insert your test string here"></textarea>
    <textarea class="row" rows="6" readonly id="regex-result">{{ xResult }}</textarea>
</template>
