<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

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

async function regex_captures() {
    try {
        let text = xText.value;
        let pattern = xPattern.value;

        if (text == "") {
            xResult.value = "[Warning] Empty Data"
            return;
        }

        if (pattern == "") {
            xResult.value = "[Warning] Empty Pattern"
            return;
        }

        let caps = await invoke("regex_captures", { pattern: pattern, text: text });
        let result = "";
        for (let i = 0; i < caps.length; i++) {
            const cap = caps[i];
            result = result + i + ": " + cap + "\n";
        }

        xResult.value = result;
    } catch (error) {
        let einfo = error.toString();
        einfo = einfo.replaceAll("\\n", "\n");
        xResult.value = "[Error]: \n\n" + einfo;
    }
}


</script>

<template>
    <label>REGULAR EXPRESSION</label>
    <form class="row" @submit.prevent="regex_captures">
        <input id="regex-input" v-model="xPattern" placeholder="insert your regular expression here" />
        <button type="submit">Go</button>
    </form>
    <label>TEST STRING</label>
    <textarea class="row" rows="10" v-model="xText" placeholder="insert your test string here"></textarea>
    <textarea class="row" rows="6" readonly id="regex-result">{{ xResult }}</textarea>
</template>
