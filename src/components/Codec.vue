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
  <textarea class="row" v-model="xText" rows="10"></textarea>
  <div>
    <button class="btn" @click="base64_decode">Base64 Decode</button>
    <button class="btn" @click="base64_encode">Base64 Encode</button>

    <button class="btn" @click="url_decode">URL Decode</button>
    <button class="btn" @click="url_encode">URL Encode</button>

  </div>
  <div>
    <button class="btn" @click="xText = ''">Clean</button>
    <button class="btn" @click="writeText(xText)">Copy</button>
  </div>
</template>
