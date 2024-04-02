<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const xText = ref("");

async function base64_decode() {
  xText.value = await invoke("base64_decode", { input: xText.value });
}

async function base64_encode() {
  xText.value = await invoke("base64_encode", { input: xText.value });
}

async function url_decode() {
  xText.value = await invoke("url_decode", { input: xText.value });
}

async function url_encode() {
  xText.value = await invoke("url_encode", { input: xText.value });
}

</script>

<template>
  <div class="box">
    <div class="btns">
      <button class="btn" @click="base64_decode">Base64 Dec</button>
      <button class="btn" @click="base64_encode">Base64 Enc</button>

      <button class="btn" @click="url_decode">URL Dec</button>
      <button class="btn" @click="url_encode">URL Enc</button>

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