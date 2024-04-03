<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";

import Codec from "./components/Codec.vue";
import Regex from "./components/Regex.vue";
import Format from "./components/Format.vue";
import Http from "./components/Http.vue";
import X509 from "./components/X509.vue";
import Random from "./components/Random.vue";
import Others from "./components/Others.vue";

const currentTab = ref('Codec');

const tabs = ref({
  Codec,
  Regex,
  Format,
  Http,
  X509,
  Random,
  Others,
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
    <keep-alive>
      <component :is="tabs[currentTab]" class="tab"></component>
    </keep-alive>
  </div>
</template>

<style scoped>
.main {
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
  /* border-top-left-radius: 3px; */
  /* border-top-right-radius: 3px; */
  border: 1px solid #ccc;
  cursor: pointer;
  color: rgba(85, 85, 85, 1);
  background: #f0f0f0;
  margin-bottom: -1px;
  margin-right: -1px;
  min-width: 80px;
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

  /* width: 100%; */

  /* margin-top: 10px; */
}
</style>