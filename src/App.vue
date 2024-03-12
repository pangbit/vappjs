<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";

import Codec from "./components/Codec.vue";
import Regex from "./components/Regex.vue";
import Format from "./components/Format.vue";
import Http from "./components/Http.vue";
import X509 from "./components/X509.vue";

const currentTab = ref('Codec');

const tabs = ref({
  Codec,
  Regex,
  Format,
  Http,
  X509,
});
</script>

<template>
  <div class="main">
    <div>
      <button v-for="(_, tab) in tabs" :key="tab" :class="['tab-button', { active: currentTab == tab }]"
        @click="currentTab = tab">
        {{ tab }}
      </button>
    </div>
    <component :is="tabs[currentTab]" class="tab"></component>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.main {
  font-family: sans-serif;
  border: 1px solid #eee;
  border-radius: 2px;
  padding: 20px 30px;
  margin-top: 1em;
  margin-bottom: 40px;
  user-select: none;
  overflow-x: auto;
  display: flex;
  flex-direction: column;
}

.tab-button {
  padding: 6px 10px;
  border-top-left-radius: 3px;
  border-top-right-radius: 3px;
  border: 1px solid #ccc;
  cursor: pointer;
  background: #f0f0f0;
  margin-bottom: 10px;
  margin-right: -1px;
}

.tab-button:hover {
  background: #e0e0e0;
}

.tab-button.active {
  background: #e0e0e0;
}

.tab {
  border: 1px solid #ccc;
  padding: 10px;

  margin-top: 500px;
  widows: 100%;

}
</style>
