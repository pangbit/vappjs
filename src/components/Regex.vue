<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const xText = ref("");
const xPattern = ref("");
const xResult = ref("");

async function isMatch() {
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

async function regexCaptures() {
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
    <div class="box">
        <label>REGULAR EXPRESSION</label>
        <form class="form-box" @submit.prevent="regexCaptures">
            <input class="form-input" v-model="xPattern" />
            <button class="btn form-btn" type="submit">Go</button>
        </form>
        <label>TEST STRING</label>
        <textarea class="show" v-model="xText"></textarea>
        <textarea class="show capture" readonly>{{ xResult }}</textarea>
    </div>
</template>

<style scoped>
.show {
    margin-bottom: 5px;
    min-height: 50px;
    height: 200px;
}

.capture {
    color: #f37171;
}
</style>