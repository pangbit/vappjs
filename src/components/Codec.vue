<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const xText = ref("");

async function base64Decode() {
  xText.value = await invoke("base64_decode", { input: xText.value });
}

async function base64Encode() {
  xText.value = await invoke("base64_encode", { input: xText.value });
}

async function urlDecode() {
  xText.value = await invoke("url_decode", { input: xText.value });
}

async function urlEncode() {
  xText.value = await invoke("url_encode", { input: xText.value });
}

</script>

<template>
  <div class="box">
    <div class="btns">
      <button class="btn" @click="base64Decode">Base64 Dec</button>
      <button class="btn" @click="base64Encode">Base64 Enc</button>

      <button class="btn" @click="urlDecode">URL Dec</button>
      <button class="btn" @click="urlEncode">URL Enc</button>

      <button class="btn" @click="xText = ''">Clean</button>
      <button class="btn" @click="writeText(xText)">Copy</button>
    </div>
    <textarea class="show" v-model="xText" rows="10"></textarea>
  </div>
</template>

<style scoped>
.show {
  min-height: 300px;
}
</style>